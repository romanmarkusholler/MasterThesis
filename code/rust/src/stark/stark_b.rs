use winterfell::{Air, Assertion, ByteWriter, EvaluationFrame, Prover, Serializable, Trace, TraceInfo, TraceTable, TransitionConstraintDegree};
use winterfell::math::{fields::f128::BaseElement, FieldElement, StarkField};
use winterfell::ProofOptions;
use winterfell::AirContext;
use crate::rescue::p128_m4_c2_s128::{enforce_round, apply_round, get_round_constants_periodic, STATE_WIDTH, RATE, NUM_ROUNDS};
use crate::utils::{create_meta, get_meta, IndexDefinition, IndexDefinitionSlice, next_power_of_two};

const HASH_MASK: [BaseElement; CYCLE_LENGTH] = [
    BaseElement::ZERO,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ONE,
    BaseElement::ZERO,
];

const STAT_MASK: [BaseElement; CYCLE_LENGTH] = [
    BaseElement::ONE,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
];

const COPY_MASK: [BaseElement; CYCLE_LENGTH] = [
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ZERO,
    BaseElement::ONE,
];

const NUM_MASKS: usize = 3;

const ROUND_CONSTS_SHIFT: usize = 1;
pub const CYCLE_LENGTH: usize = 16;
pub const TRACE_WIDTH: usize = 6;

const MASKS: [[BaseElement; CYCLE_LENGTH]; NUM_MASKS] = [
    HASH_MASK,
    STAT_MASK,
    COPY_MASK,
];

// AET index definitions
const T_PIXELS: IndexDefinition = IndexDefinition { idx: 0, size: 2 };
pub const T_PIXELS_HASH: IndexDefinition = IndexDefinition { idx: 2, size: 4 };

// constraint index definitions
const C_ABSORB: IndexDefinition = IndexDefinition {idx: 0, size: 4};
const C_COPY: IndexDefinition = IndexDefinition {idx: 4, size: 4};
const C_ROUND: IndexDefinition = IndexDefinition {idx: 8, size: 4};

// periodic column index definitions
const P_ROUND_CONSTANTS: IndexDefinition = IndexDefinition { idx: 0, size: 8 };
const P_HASH_FLAG: IndexDefinition = IndexDefinition { idx: 8, size: 1 };
const P_STAT_FLAG: IndexDefinition = IndexDefinition { idx: 9, size: 1 };
const P_COPY_FLAG: IndexDefinition = IndexDefinition { idx: 10, size: 1 };

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
}

pub struct TheAir {
    context: AirContext<BaseElement>,
    hash: [BaseElement; RATE],
    input_length: BaseElement,
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
        assert_eq!(0, pub_inputs.input_length.as_int() % 2u128);
        let degrees = vec![
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]),
            TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]),
        ];
        TheAir {
            context: AirContext::new(trace_info, degrees, options),
            hash: pub_inputs.hash,
            input_length: pub_inputs.input_length
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
        let hash_flag = periodic_values[P_HASH_FLAG.idx];
        let stat_flag = periodic_values[P_STAT_FLAG.idx];
        let copy_flag = periodic_values[P_COPY_FLAG.idx];
        let round_constants = periodic_values.id_slice(P_ROUND_CONSTANTS);
        enforce_round(&mut result[C_ROUND.begin()..C_ROUND.end()], current.id_slice(T_PIXELS_HASH), next.id_slice(T_PIXELS_HASH), round_constants, hash_flag);
        enforce_absorb(&mut result[C_ABSORB.begin()..C_ABSORB.end()], current, next, stat_flag);
        enforce_copy(&mut result[C_COPY.begin()..C_COPY.end()], current, next, copy_flag);
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let idx_result = CYCLE_LENGTH * (self.input_length.as_int() as usize) / NUM_ELEMS_PER_CYCLE;
        let mut result = vec![];

        for c in 0..STATE_WIDTH {
            result.push(Assertion::single(T_PIXELS_HASH.idx + c, 0, Self::BaseField::ZERO));
        }

        for c in 0..RATE {
            result.push(Assertion::single(T_PIXELS_HASH.idx + c, idx_result, self.hash[c]));
        }

        result
    }

    fn get_periodic_column_values(&self) -> Vec<Vec<Self::BaseField>> {
        let mut periodic_columns = get_round_constants_periodic(CYCLE_LENGTH, ROUND_CONSTS_SHIFT);
        periodic_columns.append(MASKS.iter().map(|inner| inner.to_vec()).collect::<Vec<Vec<BaseElement>>>().as_mut());
        periodic_columns
    }
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

fn enforce_copy<E: FieldElement + From<BaseElement>>(
    result_slice: &mut [E],
    current: &[E],
    next: &[E],
    flag: E,
) {
    for i in 0..STATE_WIDTH {
        result_slice[i] += flag * (current[T_PIXELS_HASH.idx + i] - next[T_PIXELS_HASH.idx + i]);
    }
}

pub fn build_trace(pixels: &Vec<u16>) -> TraceTable<BaseElement> {
    // The desired hash value is located at step i = (pixels.len() / NUM_ELEMS_PER_CYCLE) * CYCLE_LENGTH, which is
    // one more than (pixels.len() / NUM_ELEMS_PER_CYCLE) * CYCLE_LENGTH
    assert_eq!(0, pixels.len() % 2);
    let trace_len = next_power_of_two((pixels.len() / NUM_ELEMS_PER_CYCLE) * CYCLE_LENGTH + 1);
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
                1..=NUM_ROUNDS => {
                    // apply round of hash function
                    let round = cyclic_step - 1;
                    apply_round(&mut state[T_PIXELS_HASH.begin()..T_PIXELS_HASH.end()], round);
                },
                _ => {}
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
        PubInputs {
            hash,
            input_length: BaseElement::new(input_length as u128)
        }
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}