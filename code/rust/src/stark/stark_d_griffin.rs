use std::cmp::max;
use winterfell::{Air, Assertion, ByteWriter, EvaluationFrame, Prover, Serializable, Trace, TraceInfo, TraceTable, TransitionConstraintDegree};
use winterfell::math::{fields::f128::BaseElement, FieldElement, StarkField};
use winterfell::ProofOptions;
use winterfell::AirContext;
use crate::griffin::p128_t4_c2_s128::{enforce_round, enforce_first_round, apply_round, get_round_constants_periodic, STATE_WIDTH, RATE, NUM_ROUNDS};
use crate::utils::{create_meta, get_meta, IndexDefinition, IndexDefinitionSlice, next_power_of_two};

const IDENTITY_MASK: [[BaseElement; CYCLE_LENGTH]; CYCLE_LENGTH] = {
    let mut result = [[BaseElement::ZERO; CYCLE_LENGTH]; CYCLE_LENGTH];
    let mut i: usize = 0;
    while i < CYCLE_LENGTH {
        result[i][i] = BaseElement::ONE;
        i += 1;
    }
    result
};

const NUM_MASKS: usize = CYCLE_LENGTH;

const ROUND_CONSTS_SHIFT: usize = 1;
pub const CYCLE_LENGTH: usize = 16;
pub const TRACE_WIDTH: usize = 29;

const MASKS: [[BaseElement; CYCLE_LENGTH]; NUM_MASKS] = IDENTITY_MASK;

// AET index definitions
const T_PIXELS: IndexDefinition = IndexDefinition { idx: 0, size: 16 };
pub const T_PIXELS_HASH: IndexDefinition = IndexDefinition { idx: 16, size: 4 };
#[allow(non_upper_case_globals)]
const T_t: IndexDefinition = IndexDefinition { idx: 20, size: 1 };
#[allow(non_upper_case_globals)]
const T_s: IndexDefinition = IndexDefinition { idx: 21, size: 1 };
#[allow(non_upper_case_globals)]
const T_F_f: IndexDefinition = IndexDefinition { idx: 22, size: 1 };
#[allow(non_upper_case_globals)]
const T_F_t: IndexDefinition = IndexDefinition { idx: 23, size: 1 };
const T_G: IndexDefinition = IndexDefinition { idx: 24, size: 1 };
const T_R: IndexDefinition = IndexDefinition { idx: 25, size: 1 };
#[allow(non_upper_case_globals)]
const T_f_f: IndexDefinition = IndexDefinition { idx: 26, size: 1 };
#[allow(non_upper_case_globals)]
const T_f_t: IndexDefinition = IndexDefinition { idx: 27, size: 1 };
#[allow(non_upper_case_globals)]
const T_f_s: IndexDefinition = IndexDefinition { idx: 28, size: 1 };
const T_FLAGS: IndexDefinition = IndexDefinition { idx: 26, size: 3 };

// constraint index definitions
const C_ABSORB: IndexDefinition = IndexDefinition {idx: 0, size: 4};
const C_COPY: IndexDefinition = IndexDefinition {idx: 4, size: 4};
const C_ROUND: IndexDefinition = IndexDefinition {idx: 8, size: 4};
const C_COPY_PIXEL: IndexDefinition = IndexDefinition {idx: 12, size: 16};
const C_FLAG_DOMAIN: IndexDefinition = IndexDefinition {idx: 28, size: 3};
const C_FLAG_TRANSITION: IndexDefinition = IndexDefinition {idx: 31, size: 3};
const C_T_TRANSITION: IndexDefinition = IndexDefinition {idx: 34, size: 1};
const C_R: IndexDefinition = IndexDefinition {idx: 35, size: 1};
const C_G_TRANSITION: IndexDefinition = IndexDefinition {idx: 36, size: 1};
#[allow(non_upper_case_globals)]
const C_F_t_TRANSITION: IndexDefinition = IndexDefinition {idx: 37, size: 1};
#[allow(non_upper_case_globals)]
const C_F_f_TRANSITION: IndexDefinition = IndexDefinition {idx: 38, size: 1};

// periodic column index definitions
const P_ROUND_CONSTANTS: IndexDefinition = IndexDefinition { idx: 0, size: 4 };
const P_IDENTITY: IndexDefinition = IndexDefinition { idx: 4, size: 16 };

// number of elements being absorbed in one absorption phase
pub const NUM_ELEMS_PER_CYCLE: usize = RATE * NUM_PIXELS_PER_ELEM;
const NUM_PIXELS_PER_ELEM: usize = 8;
const NUM_BITS_PER_PIXEL: usize = 16; // This also defines the range for the plookup check. Careful, needs to conform to the modulus. MSBs for 62, 64, and 128 bit modulus allow up to 2^16-2 as a value, we are safe there. DO NOT go beyond 2^16 here.
const COMPRESSOR: [BaseElement; NUM_PIXELS_PER_ELEM] = {
    let mut result = [BaseElement::ZERO; NUM_PIXELS_PER_ELEM];
    let mut i: usize = 0;
    while i < NUM_PIXELS_PER_ELEM {  // for loop not allowed, but while loop is .. https://github.com/rust-lang/rust/issues/87575
        let the_number = 1u128 << NUM_BITS_PER_PIXEL as u128 * i as u128;
        result[i] = BaseElement::new(the_number);
        i += 1;
    }
    result
};


#[cfg(feature = "master_thesis_full")]
pub const SIZE_OF_T: usize = (1usize << (NUM_BITS_PER_PIXEL)) - 1;
#[cfg(feature = "master_thesis_half")]
pub const SIZE_OF_T: usize = (1usize << (NUM_BITS_PER_PIXEL - 1)) - 1;
#[cfg(feature = "master_thesis_quarter")]
pub const SIZE_OF_T: usize = (1usize << (NUM_BITS_PER_PIXEL - 2)) - 1;
#[cfg(feature = "master_thesis_test")]
pub const SIZE_OF_T: usize = 4096;

pub struct PubInputs {
    pub hash: [BaseElement; RATE],
    pub input_length: BaseElement,
}

pub struct TheAir {
    // Note that we do not derive proper randomness here, as this solution will never be used for performance reasons anyways ..
    context: AirContext<BaseElement>,
    hash: [BaseElement; RATE],
    input_length: BaseElement,
    beta: BaseElement,
    gamma: BaseElement,
}

impl Serializable for PubInputs {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        for elem in self.hash {
            target.write(elem);
        }
        target.write(self.input_length);
    }
}

impl Air for TheAir {
    type BaseField = BaseElement;
    type PublicInputs = PubInputs;

    fn new(trace_info: TraceInfo, pub_inputs: Self::PublicInputs, options: ProofOptions) -> Self {
        assert_eq!(0, pub_inputs.input_length.as_int() % 16);
        let degrees = vec![
            // hash absorb
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            // hash copy
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            // hash enforce
            TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]),
            // pixel copy
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            // flag domain
            TransitionConstraintDegree::new(2),
            TransitionConstraintDegree::new(2),
            TransitionConstraintDegree::new(2),
            // flag transition
            TransitionConstraintDegree::new(2),
            TransitionConstraintDegree::new(2),
            TransitionConstraintDegree::new(2),
            // t transition
            TransitionConstraintDegree::new(2),
            // R
            TransitionConstraintDegree::new(2),
            // G
            TransitionConstraintDegree::new(3),
            // F_t
            TransitionConstraintDegree::new(3),
            // F_f
            TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]),
        ];
        TheAir {
            context: AirContext::new(trace_info, degrees, options),
            hash: pub_inputs.hash,
            input_length: pub_inputs.input_length,
            beta: pub_inputs.hash[0],
            gamma: pub_inputs.hash[0],
        }
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        return &self.context
    }

    fn evaluate_transition<E: FieldElement<BaseField=Self::BaseField> + From<BaseElement>>(
        &self,
        frame: &EvaluationFrame<E>,
        periodic_values: &[E],
        result: &mut [E]
    ) {
        let current = frame.current();
        let next = frame.next();
        let hash_flag_a = periodic_values.id_slice(P_IDENTITY)[1];
        let hash_flag_b = {
            let mut result = E::ZERO;
            for i in 2usize..=12 {
                result += E::from(periodic_values.id_slice(P_IDENTITY)[i]);
            }
            result
        };
        let stat_flag = periodic_values.id_slice(P_IDENTITY)[0];
        let hash_copy_flag = {
            let mut result = E::ZERO;
            for i in 13usize..=15 {
                result += E::from(periodic_values.id_slice(P_IDENTITY)[i]);
            }
            result
        };
        let pixel_copy_flag = {
            let mut result = E::ZERO;
            for i in 1usize..=15 {
                result += E::from(periodic_values.id_slice(P_IDENTITY)[i]);
            }
            result
        };
        let round_constants = periodic_values.id_slice(P_ROUND_CONSTANTS);
        enforce_first_round(&mut result[C_ROUND.begin()..C_ROUND.end()], current.id_slice(T_PIXELS_HASH), next.id_slice(T_PIXELS_HASH), round_constants, hash_flag_a);
        enforce_round(&mut result[C_ROUND.begin()..C_ROUND.end()], current.id_slice(T_PIXELS_HASH), next.id_slice(T_PIXELS_HASH), round_constants, hash_flag_b);
        enforce_absorb(&mut result[C_ABSORB.begin()..C_ABSORB.end()], current, next, stat_flag);
        enforce_hash_copy(&mut result[C_COPY.begin()..C_COPY.end()], current, next, hash_copy_flag);
        enforce_pixel_copy(&mut result[C_COPY_PIXEL.begin()..C_COPY_PIXEL.end()], current, next, pixel_copy_flag);
        enforce_plookup(result, periodic_values, current, next, self);
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let idx_result = CYCLE_LENGTH * (self.input_length.as_int() as usize) / NUM_ELEMS_PER_CYCLE;
        let mut result = vec![];

        // hash IV must all be 0
        for c in 0..STATE_WIDTH {
            result.push(Assertion::single(T_PIXELS_HASH.begin() + c, 0, Self::BaseField::ZERO));
        }

        // computed hash must match the hash from public inputs
        for c in 0..RATE {
            result.push(Assertion::single(T_PIXELS_HASH.begin() + c, idx_result, self.hash[c]));
        }

        // plookup
        let size_f = self.input_length.as_int() as usize;
        let size_t = SIZE_OF_T;
        let size_s = size_f + size_t;
        result.push(Assertion::single(T_F_f.begin(), 0, Self::BaseField::ONE));
        result.push(Assertion::single(T_F_t.begin(), 0, Self::BaseField::ONE));
        result.push(Assertion::single(T_G.begin(), 0, Self::BaseField::ONE));
        result.push(Assertion::single(T_f_f.begin(), 0, Self::BaseField::ONE));
        result.push(Assertion::single(T_f_t.begin(), 0, Self::BaseField::ONE));
        result.push(Assertion::single(T_f_s.begin(), 0, Self::BaseField::ONE));
        result.push(Assertion::single(T_t.begin(), 0, Self::BaseField::ZERO));
        result.push(Assertion::single(T_f_f.begin(), size_f, Self::BaseField::ONE));
        result.push(Assertion::single(T_f_t.begin(), size_t - 1, Self::BaseField::ONE));
        result.push(Assertion::single(T_f_s.begin(), size_s - 1, Self::BaseField::ONE));
        result.push(Assertion::single(T_f_f.begin(), size_f + 1, Self::BaseField::ZERO));
        result.push(Assertion::single(T_f_t.begin(), size_t, Self::BaseField::ZERO));
        result.push(Assertion::single(T_f_s.begin(), size_s, Self::BaseField::ZERO));
        result.push(Assertion::single(T_R.begin(), size_s - 1, Self::BaseField::ZERO));

        result
    }

    fn get_periodic_column_values(&self) -> Vec<Vec<Self::BaseField>> {
        let mut periodic_columns = get_round_constants_periodic(CYCLE_LENGTH, ROUND_CONSTS_SHIFT);
        periodic_columns.append(MASKS.iter().map(|inner| inner.to_vec()).collect::<Vec<Vec<BaseElement>>>().as_mut());
        periodic_columns
    }
}

// function S(a, f_x)
fn helper_s<E: FieldElement + From<BaseElement>>(
    value: E,
    flag: E
) -> E {
    value * flag + E::ONE - flag
}

// function T(f_x^cu, f_x^ne)
fn helper_t<E: FieldElement + From<BaseElement>>(
    flag_current: E,
    flag_next: E
) -> E {
    (flag_current - flag_next) * (flag_current - flag_next - E::ONE)
}

// function U(f_x)
fn helper_u<E: FieldElement + From<BaseElement>>(
    flag: E
) -> E {
    (flag - E::ZERO) * (flag - E::ONE)
}

fn enforce_plookup<E: FieldElement + From<BaseElement>>(
    result_slice: &mut [E],
    periodic_values: &[E],
    current: &[E],
    next: &[E],
    air: &TheAir,
) {
    // flags: functions U and T
    for i in 0..C_FLAG_DOMAIN.size {
        result_slice[C_FLAG_DOMAIN.begin() + i] += helper_u(next[T_FLAGS.begin() + i]);
    }
    for i in 0..C_FLAG_TRANSITION.size {
        result_slice[C_FLAG_TRANSITION.begin() + i] += helper_t(current[T_FLAGS.begin() + i], next[T_FLAGS.begin() + i]);
    }
    // transition of t
    result_slice[C_T_TRANSITION.begin()] += (next[T_t.begin()] - current[T_t.begin()] - E::ONE) * next[T_f_t.begin()];
    // calculation of R
    result_slice[C_R.begin()] += next[T_F_f.begin()] * next[T_F_t.begin()] - next[T_G.begin()] - next[T_R.begin()];
    // calculation of G
    result_slice[C_G_TRANSITION.begin()] += helper_s(E::from(air.gamma) * (E::ONE + E::from(air.beta)) + current[T_s.begin()] + E::from(air.beta) * next[T_s.begin()], next[T_f_s.begin()]) * current[T_G.begin()] - next[T_G.begin()];
    // calculation of F_t
    result_slice[C_F_t_TRANSITION.begin()] += helper_s(E::from(air.gamma) * (E::ONE + E::from(air.beta)) + current[T_t.begin()] + E::from(air.beta) * next[T_t.begin()], next[T_f_t.begin()]) * current[T_F_t.begin()] - next[T_F_t.begin()];
    // calculation of F_f
    #[allow(non_snake_case)]
    let mut s_sum_F_f = E::ZERO;
    for i in 0usize..=15 {
        s_sum_F_f += periodic_values[P_IDENTITY.begin() + i] * helper_s((E::ONE + E::from(air.beta)) * (E::from(air.gamma) + next[T_PIXELS.begin() + i]), next[T_f_f.begin()]) * current[T_F_f.begin()];
    }
    result_slice[C_F_f_TRANSITION.begin()] += s_sum_F_f - next[T_F_f.begin()];
}

fn enforce_absorb<E: FieldElement + From<BaseElement>>(
    result_slice: &mut [E],
    current: &[E],
    next: &[E],
    flag: E,
) {
    for c in 0..RATE {
        let mut tmp = E::ZERO;
        for d in 0..NUM_PIXELS_PER_ELEM {
            tmp += next[T_PIXELS.idx + NUM_PIXELS_PER_ELEM * c + d] * E::from(COMPRESSOR[d]);
        }
        result_slice[c] += flag * (current[T_PIXELS_HASH.idx + c] + tmp - next[T_PIXELS_HASH.idx + c]);
    }

    for c in RATE..STATE_WIDTH {
        result_slice[c] += flag * (current[T_PIXELS_HASH.idx + c] - next[T_PIXELS_HASH.idx + c]);
    }
}

fn enforce_hash_copy<E: FieldElement + From<BaseElement>>(
    result_slice: &mut [E],
    current: &[E],
    next: &[E],
    flag: E,
) {
    for i in 0..STATE_WIDTH {
        result_slice[i] += flag * (current[T_PIXELS_HASH.idx + i] - next[T_PIXELS_HASH.idx + i]);
    }
}
fn enforce_pixel_copy<E: FieldElement + From<BaseElement>>(
    result_slice: &mut [E],
    current: &[E],
    next: &[E],
    flag: E,
) {
    for i in 0..T_PIXELS.size {
        result_slice[i] += flag * (current[T_PIXELS.idx + i] - next[T_PIXELS.idx + i]);
    }
}

pub fn build_trace(pixels: &Vec<u16>, hash: &[BaseElement; RATE]) -> TraceTable<BaseElement> {
    // The desired hash value is located at step i = (pixels.len() / NUM_ELEMS_PER_CYCLE) * CYCLE_LENGTH, which is
    // one more than (pixels.len() / NUM_ELEMS_PER_CYCLE) * CYCLE_LENGTH
    assert_eq!(0, pixels.len() % 16);
    let len_req_hash = next_power_of_two((pixels.len() / NUM_ELEMS_PER_CYCLE) * CYCLE_LENGTH + 1);
    let size_f = pixels.len();
    let size_s = SIZE_OF_T + size_f;
    let len_req_plookup = next_power_of_two(size_s + 1); // make sure we include at least one step where f_s is zero to definitely have all possible combinations of transition constraints
    let trace_len = max(len_req_hash, len_req_plookup);
    let mut table = TraceTable::<BaseElement>::with_meta(TRACE_WIDTH, trace_len, create_meta(pixels.len()));
    let beta = hash[0];
    let gamma = hash[0];
    // let us use some hash func with rate = 1, absorb all hash elements and then squeeze the sponge to get 2 elems out of this.
    let mut s = vec![0u128; size_s];
    for i in 0..SIZE_OF_T {
        s[i] = i as u128;
    }
    for i in 0..pixels.len() {
        s[SIZE_OF_T + i] = pixels[i] as u128;
    }
    s.sort();
    table.fill(
        |state| {
            for i in 0..TRACE_WIDTH {
                state[i] = BaseElement::ZERO;
            }
            state[T_F_f.begin()] = BaseElement::ONE;
            state[T_F_t.begin()] = BaseElement::ONE;
            state[T_G.begin()] = BaseElement::ONE;
            state[T_f_f.begin()] = BaseElement::ONE;
            state[T_f_t.begin()] = BaseElement::ONE;
            state[T_f_s.begin()] = BaseElement::ONE;
            state[T_s.begin()] = BaseElement::new(s[0]);
        },
        |step, state| {  // step .. index of the last updated row (starting with 0)
            let cyclic_step = step % CYCLE_LENGTH; // cyclic_step can be considered the index in the MASKs
            // load pixels and do hashing
            match cyclic_step {
                0 => {
                    // read elements into trace
                    for c in 0..NUM_ELEMS_PER_CYCLE {
                        let next_pixel_idx = NUM_ELEMS_PER_CYCLE * step / CYCLE_LENGTH + c;
                        if next_pixel_idx < pixels.len() {
                            state[T_PIXELS.idx + c] = BaseElement::from(pixels[next_pixel_idx]);
                        }
                    }
                    // absorb elements into hash state
                    for c in 0..RATE {
                        for d in 0..NUM_PIXELS_PER_ELEM {
                            state[T_PIXELS_HASH.idx + c] += state[T_PIXELS.idx + NUM_PIXELS_PER_ELEM * c + d] * COMPRESSOR[d];
                        }
                    }
                }
                1..=NUM_ROUNDS => {
                    // apply round of hash function
                    let round = cyclic_step - 1;
                    apply_round(&mut state[T_PIXELS_HASH.begin()..T_PIXELS_HASH.end()], round);
                },
                _ => {}
            }
            // plookup
            let s_current = state[T_s.begin()];
            let t_current = state[T_t.begin()];
            if step + 1 < size_s {
                state[T_s.begin()] = BaseElement::new(s[step + 1]);
            }
            state[T_t.begin()] += BaseElement::ONE;
            let s_next = state[T_s.begin()];
            let t_next = state[T_t.begin()];
            if step == pixels.len() {
                state[T_f_f.begin()] = BaseElement::ZERO;
            }
            if step == (SIZE_OF_T - 1) {
                state[T_f_t.begin()] = BaseElement::ZERO;
            }
            if step == (SIZE_OF_T + pixels.len() - 1) {
                state[T_f_s.begin()] = BaseElement::ZERO;
            }
            state[T_F_f.begin()] *= helper_s((BaseElement::ONE + beta) * (gamma + state[T_PIXELS.begin() + cyclic_step]), state[T_f_f.begin()]);
            state[T_F_t.begin()] *= helper_s(gamma * (BaseElement::ONE + beta) + t_current + beta * t_next, state[T_f_t.begin()]);
            state[T_G.begin()] *= helper_s(gamma * (BaseElement::ONE + beta) + s_current + beta * s_next, state[T_f_s.begin()]);
            state[T_R.begin()] = state[T_F_f.begin()] * state[T_F_t.begin()] - state[T_G.begin()];
        }
    );
    table
}

pub struct TheProver {
    options: ProofOptions
}

impl TheProver {
    pub fn new(options: ProofOptions) -> Self {
        Self { options }
    }
}

impl Prover for TheProver {
    type BaseField = BaseElement;
    type Air = TheAir;
    type Trace = TraceTable<Self::BaseField>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> <<Self as Prover>::Air as Air>::PublicInputs {
        let input_length = get_meta(&trace.meta().to_vec());
        let result_step = input_length * CYCLE_LENGTH / NUM_ELEMS_PER_CYCLE;
        let mut hash = [BaseElement::ZERO; RATE];
        for c in 0..RATE {
            hash[c] = trace.get(T_PIXELS_HASH.idx + c, result_step);
        }
        PubInputs {
            hash,
            input_length: BaseElement::new(input_length as u128)
        }
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}
