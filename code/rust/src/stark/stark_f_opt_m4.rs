use winterfell::{Air, Assertion, ByteWriter, EvaluationFrame, Prover, Serializable, Trace, TraceInfo, TraceTable, TransitionConstraintDegree};
use winterfell::math::{fields::f128::BaseElement, FieldElement, StarkField};
use winterfell::ProofOptions;
use winterfell::AirContext;
use rounded_div::RoundedDiv;
use crate::rescue::p128_m33_c1_s128::{enforce_round, enforce_first_round, apply_round, get_round_constants_periodic, STATE_WIDTH, RATE};
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

// STARK F (opt) parameter m:
pub const FACTOR_M: usize = 4;

const NUM_MASKS: usize = CYCLE_LENGTH;

const ROUND_CONSTS_SHIFT: usize = 0;
pub const CYCLE_LENGTH: usize = 8;
pub const TRACE_WIDTH: usize = 16 * FACTOR_M + 3;

const MASKS: [[BaseElement; CYCLE_LENGTH]; NUM_MASKS] = IDENTITY_MASK;

// AET index definitions
const T_PIXELS: IndexDefinition = IndexDefinition { idx: 0, size: 8 * FACTOR_M };
pub const T_PIXELS_HASH: IndexDefinition = IndexDefinition { idx: 8 * FACTOR_M, size: 8 * FACTOR_M + 1};
pub const T_SUM: IndexDefinition = IndexDefinition { idx: 16 * FACTOR_M + 1, size: 1 };
pub const T_VAR: IndexDefinition = IndexDefinition { idx: 16 * FACTOR_M + 2, size: 1 };

// constraint index definitions
const C_ROUND_FIRST: IndexDefinition = IndexDefinition {idx: 0, size: 8 * FACTOR_M + 1};
const C_ROUND_REMAINING: IndexDefinition = IndexDefinition {idx: 8 * FACTOR_M + 1, size: 8 * FACTOR_M + 1};
const C_COPY: IndexDefinition = IndexDefinition {idx: 2 * (8 * FACTOR_M + 1), size: 8 * FACTOR_M};
const C_SUM: IndexDefinition = IndexDefinition {idx: 2 * (8 * FACTOR_M + 1) + 8 * FACTOR_M, size: 1};
const C_VAR: IndexDefinition = IndexDefinition {idx: 2 * (8 * FACTOR_M + 1) + 8 * FACTOR_M + 1, size: 1};

// periodic column index definitions
const P_ROUND_CONSTANTS: IndexDefinition = IndexDefinition { idx: 0, size: 2 * (8 * FACTOR_M + 1) };
const P_IDENTITY: IndexDefinition = IndexDefinition { idx: 2 * (8 * FACTOR_M + 1), size: 8 };

// number of elements being absorbed in one absorption phase
pub const NUM_ELEMS_PER_CYCLE: usize = RATE * NUM_PIXELS_PER_ELEM;
const NUM_PIXELS_PER_ELEM: usize = 1;
const NUM_BITS_PER_PIXEL: usize = 128;
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

pub struct PubInputs {
    pub hash: [BaseElement; RATE],
    pub input_length: BaseElement,
    pub sum: BaseElement,
    pub avg_rounded: BaseElement,
    pub variance: BaseElement,
}

pub struct TheAir {
    context: AirContext<BaseElement>,
    hash: [BaseElement; RATE],
    input_length: BaseElement,
    sum: BaseElement,
    avg_rounded: BaseElement,
    variance: BaseElement,
}

impl Serializable for PubInputs {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        for elem in self.hash {
            target.write(elem);
        }
        target.write(self.input_length);
        target.write(self.sum);
        target.write(self.avg_rounded);
        target.write(self.variance);
    }
}

impl Air for TheAir {
    type BaseField = BaseElement;
    type PublicInputs = PubInputs;

    fn new(trace_info: TraceInfo, pub_inputs: Self::PublicInputs, options: ProofOptions) -> Self {
        assert_eq!(0, pub_inputs.input_length.as_int() % CYCLE_LENGTH as u128);
        let mut degrees = vec![];

        for _ in 0..C_ROUND_FIRST.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_ROUND_REMAINING.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_COPY.size {
            degrees.push(TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]));
        }
        // C_SUM
        degrees.push(TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]));
        // C_VAR
        degrees.push(TransitionConstraintDegree::with_cycles(2, vec![CYCLE_LENGTH]));

        TheAir {
            context: AirContext::new(trace_info, degrees, options),
            hash: pub_inputs.hash,
            input_length: pub_inputs.input_length,
            sum: pub_inputs.sum,
            avg_rounded: pub_inputs.avg_rounded,
            variance: pub_inputs.variance
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
        let round_constants = periodic_values.id_slice(P_ROUND_CONSTANTS);
        let identity = periodic_values.id_slice(P_IDENTITY);

        let hash_first_flag = identity[0];
        let hash_remaining_flag = E::ONE - identity[0];
        let copy_flag = hash_remaining_flag;

        enforce_first_round(&mut result[C_ROUND_FIRST.begin()..C_ROUND_FIRST.end()], next.id_slice(T_PIXELS), current.id_slice(T_PIXELS_HASH), next.id_slice(T_PIXELS_HASH), round_constants, hash_first_flag);
        enforce_round(&mut result[C_ROUND_REMAINING.begin()..C_ROUND_REMAINING.end()], current.id_slice(T_PIXELS_HASH), next.id_slice(T_PIXELS_HASH), round_constants, hash_remaining_flag);
        enforce_copy(&mut result[C_COPY.begin()..C_COPY.end()], current.id_slice(T_PIXELS), next.id_slice(T_PIXELS), copy_flag);
        enforce_sum(&mut result[C_SUM.begin()..C_SUM.end()], next.id_slice(T_PIXELS), identity, current.id_slice(T_SUM)[0], next.id_slice(T_SUM)[0], E::ONE);
        enforce_var(&mut result[C_VAR.begin()..C_VAR.end()], next.id_slice(T_PIXELS), identity, current.id_slice(T_VAR)[0], next.id_slice(T_VAR)[0], E::from(self.avg_rounded), E::ONE);
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let idx_result = CYCLE_LENGTH * (self.input_length.as_int() as usize) / NUM_ELEMS_PER_CYCLE;
        let mut result = vec![];

        for c in 0..STATE_WIDTH {
            result.push(Assertion::single(T_PIXELS_HASH.begin() + c, 0, Self::BaseField::ZERO));
        }
        for c in 0..RATE {
            result.push(Assertion::single(T_PIXELS_HASH.begin() + c, idx_result, self.hash[c]));
        }
        result.push(Assertion::single(T_SUM.begin(), 0, Self::BaseField::ZERO));
        result.push(Assertion::single(T_VAR.begin(), 0, Self::BaseField::ZERO));
        result.push(Assertion::single(T_SUM.begin(), idx_result, self.sum));
        result.push(Assertion::single(T_VAR.begin(), idx_result, self.variance));

        result
    }

    fn get_periodic_column_values(&self) -> Vec<Vec<Self::BaseField>> {
        let mut periodic_columns = get_round_constants_periodic(CYCLE_LENGTH, ROUND_CONSTS_SHIFT);
        periodic_columns.append(MASKS.iter().map(|inner| inner.to_vec()).collect::<Vec<Vec<BaseElement>>>().as_mut());
        periodic_columns
    }
}

fn enforce_copy<E: FieldElement + From<BaseElement>>(
    result_slice: &mut [E],
    current: &[E],
    next: &[E],
    flag: E,
) {
    for i in 0..current.len() {
        result_slice[i] += flag * (current[i] - next[i]);
    }
}

fn enforce_sum<E: FieldElement + From<BaseElement>>(
    result_slice: &mut [E],
    pixels: &[E],
    identity: &[E],
    current_sum: E,
    next_sum: E,
    flag: E,
) {
    let mut sum_part = E::ZERO;
    for i in 0..pixels.len() {
        sum_part += identity[i % CYCLE_LENGTH] * pixels[i];
    }
    result_slice[0] += flag * (current_sum - next_sum + sum_part);
}

fn enforce_var<E: FieldElement + From<BaseElement>>(
    result_slice: &mut [E],
    pixels: &[E],
    identity: &[E],
    current_var: E,
    next_var: E,
    avg: E,
    flag: E,
) {
    let mut sum_part = E::ZERO;
    for i in 0..pixels.len() {
        sum_part += identity[i % CYCLE_LENGTH] * (pixels[i] - avg) * (pixels[i] - avg);
    }
    result_slice[0] += flag * (current_var - next_var + sum_part);
}

pub fn build_trace(pixels: &Vec<u16>) -> TraceTable<BaseElement> {
    // The desired hash value is located at step i = (pixels.len() / NUM_ELEMS_PER_CYCLE) * CYCLE_LENGTH, which is
    // one more than (pixels.len() / NUM_ELEMS_PER_CYCLE) * CYCLE_LENGTH
    assert_eq!(0, pixels.len() % CYCLE_LENGTH);
    let trace_len = next_power_of_two((pixels.len() / NUM_ELEMS_PER_CYCLE) * CYCLE_LENGTH + 1);
    let sum = pixels.iter().map(|e| {*e as u128}).sum::<u128>();
    let avg = BaseElement::new(sum.rounded_div(pixels.len() as u128));
    let mut table = TraceTable::<BaseElement>::with_meta(TRACE_WIDTH, trace_len, create_meta(pixels.len()));
    table.fill(
        |state| {
            for i in 0..TRACE_WIDTH {
                state[i] = BaseElement::ZERO;
            }
        },
        |step, state| {  // step .. index of the last updated row (starting with 0)
            let cyclic_step = step % CYCLE_LENGTH; // cyclic_step can be considered the index in the MASKs
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
                _ => {}
            }
            apply_round(&mut state[T_PIXELS_HASH.begin()..T_PIXELS_HASH.end()], cyclic_step);
            for i in 0..FACTOR_M {
                state[T_SUM.begin()] += state[T_PIXELS.begin() + i * CYCLE_LENGTH + cyclic_step];
                state[T_VAR.begin()] += (state[T_PIXELS.begin() + i * CYCLE_LENGTH + cyclic_step] - avg) * (state[T_PIXELS.begin() + i * CYCLE_LENGTH + cyclic_step] - avg);
            }
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
        let sum = trace.get(T_SUM.begin(), result_step);
        let variance = trace.get(T_VAR.begin(), result_step);
        PubInputs {
            hash,
            input_length: BaseElement::new(input_length as u128),
            sum,
            avg_rounded: BaseElement::new((sum.as_int()).rounded_div(input_length as u128)),
            variance,
        }
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}