use winterfell::{Air, Assertion, ByteWriter, EvaluationFrame, Prover, Serializable, Trace, TraceInfo, TraceTable, TransitionConstraintDegree};
use winterfell::math::{fields::f128::BaseElement, FieldElement, StarkField};
use winterfell::ProofOptions;
use winterfell::AirContext;
use rounded_div::RoundedDiv;
use crate::griffin::p128_t12_c4_s100::{enforce_round, enforce_first_round_plus_absorb, apply_round, get_round_constants_periodic, STATE_WIDTH, RATE};
use crate::utils::{create_meta, get_meta, IndexDefinition, IndexDefinitionSlice, next_power_of_two};

// changing FRAME SIZE and the ROI:
// change FRAME_SIZE:        "frame" in terms of the video, e.g. 1920x1080
// change STAT_MASK_ROI:     defines the ROI to compute statistics on


// STARK F (opt) parameter m: MUST BE 1 for STARK G
pub const FACTOR_M: usize = 1;

const ROUND_CONSTS_SHIFT: usize = 0;
pub const CYCLE_LENGTH: usize = 8;
pub const TRACE_WIDTH: usize = 16 * FACTOR_M + 4 + 2;
pub const CYCLE_LENGTH_ROI: usize = next_power_of_two(FRAME_SIZE); // cycle that fully encapsulates one frame, a power of two

// "frame" in terms of the video, e.g. 1920x1080, must be a multiple of CYCLE_LENGTH
#[cfg(feature = "master_thesis_full")]
pub const FRAME_SIZE: usize = 110016;
#[cfg(feature = "master_thesis_half")]
pub const FRAME_SIZE: usize = 65280;
#[cfg(feature = "master_thesis_quarter")]
pub const FRAME_SIZE: usize = 32640;
#[cfg(feature = "master_thesis_test")]
pub const FRAME_SIZE: usize = 56;

pub fn get_identity_mask_roi() -> Vec<Vec<BaseElement>> {
    let mut result = vec![vec![BaseElement::ZERO; CYCLE_LENGTH_ROI]; CYCLE_LENGTH];
    let mut j: usize = 0;
    while j < (CYCLE_LENGTH_ROI / CYCLE_LENGTH) {
        let mut i: usize = 0;
        while i < CYCLE_LENGTH {
            result[i][CYCLE_LENGTH * j + i] = BaseElement::ONE;
            i += 1;
        }
        j += 1;
    }
    result
}

// derived from FRAME_SIZE, where the first FRAME_SIZE pixels are masked with ONE
#[allow(non_snake_case)]
pub fn get_hash_mask_roi() -> Vec<BaseElement>  {
    let mut result = vec![BaseElement::ZERO; CYCLE_LENGTH_ROI];
    let mut i = 0usize;
    while i < FRAME_SIZE {
        result[i] = BaseElement::ONE;
        i += 1;
    }
    result
}

// this is the definition of the ROI we want to perform computations on: Define your ROI here
#[allow(non_snake_case)]
pub fn get_stat_mask_roi() -> Vec<BaseElement> {
    let mut result = vec![BaseElement::ZERO; CYCLE_LENGTH_ROI];
    result[0] = BaseElement::ONE;
    result[1] = BaseElement::ONE;
    result[2] = BaseElement::ONE;
    result[3] = BaseElement::ONE;
    result
}

#[allow(non_snake_case)]
pub fn get_num_ones_in_stat_mask() -> usize {
    get_stat_mask_roi().iter().map(|e| {e.as_int() as u128}).sum::<u128>() as usize
}

#[allow(non_snake_case)]
pub fn get_masks() -> Vec<Vec<BaseElement>> {
    let mut result = get_identity_mask_roi();
    result.push(get_hash_mask_roi());
    result.push(get_stat_mask_roi());
    result
}

// AET index definitions
const T_PIXELS: IndexDefinition = IndexDefinition { idx: 0, size: 8 * FACTOR_M };
pub const T_PIXELS_HASH: IndexDefinition = IndexDefinition { idx: 8 * FACTOR_M, size: 8 * FACTOR_M + 4};
pub const T_SUM: IndexDefinition = IndexDefinition { idx: 16 * FACTOR_M + 4, size: 1 };
pub const T_VAR: IndexDefinition = IndexDefinition { idx: 16 * FACTOR_M + 5, size: 1 };

// constraint index definitions
const C_ROUND_FIRST: IndexDefinition = IndexDefinition {idx: 0, size: 8 * FACTOR_M + 4};
const C_ROUND_REMAINING: IndexDefinition = IndexDefinition {idx: 8 * FACTOR_M + 4, size: 8 * FACTOR_M + 4};
const C_COPY: IndexDefinition = IndexDefinition {idx: 2 * (8 * FACTOR_M + 4), size: 8 * FACTOR_M};
const C_SUM: IndexDefinition = IndexDefinition {idx: 2 * (8 * FACTOR_M + 4) + 8 * FACTOR_M, size: 1};
const C_VAR: IndexDefinition = IndexDefinition {idx: 2 * (8 * FACTOR_M + 4) + 8 * FACTOR_M + 1, size: 1};
const C_COPY_HASH_STATE: IndexDefinition = IndexDefinition {idx: 2 * (8 * FACTOR_M + 4) + 8 * FACTOR_M + 2, size: STATE_WIDTH};
const C_COPY_STAT_STATE: IndexDefinition = IndexDefinition {idx: 2 * (8 * FACTOR_M + 4) + 8 * FACTOR_M + 2 + STATE_WIDTH, size: 2};

// periodic column index definitions
const P_ROUND_CONSTANTS: IndexDefinition = IndexDefinition { idx: 0, size: (8 * FACTOR_M + 4) };
const P_IDENTITY: IndexDefinition = IndexDefinition { idx: (8 * FACTOR_M + 4), size: 8 };
const P_HASH: IndexDefinition = IndexDefinition { idx: (8 * FACTOR_M + 4) + 8, size: 1 };
const P_STAT: IndexDefinition = IndexDefinition { idx: (8 * FACTOR_M + 4) + 8 + 1, size: 1 };

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
        assert_eq!(pub_inputs.input_length.as_int() % FRAME_SIZE as u128, 0);
        assert_eq!(FRAME_SIZE % CYCLE_LENGTH, 0);
        let mut degrees = vec![];

        for _ in 0..C_ROUND_FIRST.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH, CYCLE_LENGTH_ROI]));
        }
        for _ in 0..C_ROUND_REMAINING.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH, CYCLE_LENGTH_ROI]));
        }
        for _ in 0..C_COPY.size {
            degrees.push(TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]));
        }
        // C_SUM
        degrees.push(TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH, CYCLE_LENGTH_ROI]));
        // C_VAR
        degrees.push(TransitionConstraintDegree::with_cycles(2, vec![CYCLE_LENGTH, CYCLE_LENGTH_ROI]));
        for _ in 0..C_COPY_HASH_STATE.size {
            degrees.push(TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH_ROI]));
        }
        for _ in 0..C_COPY_STAT_STATE.size {
            degrees.push(TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH_ROI]));
        }

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
        let roi_hash = periodic_values.id_slice(P_HASH)[0];
        let roi_stat = periodic_values.id_slice(P_STAT)[0];

        let hash_first_flag = identity[0];
        let hash_remaining_flag = E::ONE - identity[0];
        let copy_flag = hash_remaining_flag;

        enforce_first_round_plus_absorb(&mut result[C_ROUND_FIRST.begin()..C_ROUND_FIRST.end()], next.id_slice(T_PIXELS), current.id_slice(T_PIXELS_HASH), next.id_slice(T_PIXELS_HASH), round_constants, hash_first_flag * roi_hash);
        enforce_round(&mut result[C_ROUND_REMAINING.begin()..C_ROUND_REMAINING.end()], current.id_slice(T_PIXELS_HASH), next.id_slice(T_PIXELS_HASH), round_constants, hash_remaining_flag * roi_hash);
        enforce_copy(&mut result[C_COPY.begin()..C_COPY.end()], current.id_slice(T_PIXELS), next.id_slice(T_PIXELS), copy_flag);
        enforce_sum(&mut result[C_SUM.begin()..C_SUM.end()], next.id_slice(T_PIXELS), identity, current.id_slice(T_SUM)[0], next.id_slice(T_SUM)[0], roi_stat);
        enforce_var(&mut result[C_VAR.begin()..C_VAR.end()], next.id_slice(T_PIXELS), identity, current.id_slice(T_VAR)[0], next.id_slice(T_VAR)[0], E::from(self.avg_rounded), roi_stat);
        enforce_copy(&mut result[C_COPY_HASH_STATE.begin()..C_COPY_HASH_STATE.end()], current.id_slice(T_PIXELS_HASH), next.id_slice(T_PIXELS_HASH), E::ONE - roi_hash);
        enforce_copy(&mut result[C_COPY_STAT_STATE.begin()..C_COPY_STAT_STATE.end()], &current[T_SUM.begin()..T_VAR.end()], &next[T_SUM.begin()..T_VAR.end()], E::ONE - roi_stat);
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let idx_result = CYCLE_LENGTH_ROI * CYCLE_LENGTH * (self.input_length.as_int() as usize / FRAME_SIZE as usize) / NUM_ELEMS_PER_CYCLE;
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
        let periodic_columns_hash = get_round_constants_periodic(CYCLE_LENGTH, ROUND_CONSTS_SHIFT);
        let mut periodic_columns = vec![];
        for j in 0..periodic_columns_hash.len() {
            periodic_columns.push(vec![]);
            for _ in 0..(CYCLE_LENGTH_ROI / CYCLE_LENGTH) {
                periodic_columns[j].append(periodic_columns_hash[j].clone().as_mut());
            }
        }
        periodic_columns.append(get_masks().iter().map(|inner| inner.to_vec()).collect::<Vec<Vec<BaseElement>>>().as_mut());
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
    assert_eq!(pixels.len() % FRAME_SIZE, 0);
    assert_eq!(FRAME_SIZE % CYCLE_LENGTH, 0);
    let orig_pixels = pixels;
    let mut orig_pixel_idx = 0usize;
    let mut counter = 0usize;
    let mut pixels = vec![];
    let stat_mask = get_stat_mask_roi();
    let hash_mask = get_hash_mask_roi();
    while orig_pixel_idx < orig_pixels.len() {
        if hash_mask[counter % CYCLE_LENGTH_ROI] == BaseElement::ONE {
            pixels.push(orig_pixels[orig_pixel_idx]);
            orig_pixel_idx += 1;
        } else {
            pixels.push(42u16);
        }
        counter += 1;
    }

    while counter % CYCLE_LENGTH_ROI != 0 {
        pixels.push(42u16);
        counter += 1;
    }

    let trace_len = next_power_of_two((pixels.len() / NUM_ELEMS_PER_CYCLE) * CYCLE_LENGTH + 1);
    let mut sum = 0u128;
    let mut count = 0u128;
    for idx in 0..pixels.len() {
        if stat_mask[idx % CYCLE_LENGTH_ROI] == BaseElement::ONE {
            sum += pixels[idx] as u128;
            count += 1;
        }
    }
    let avg = BaseElement::new(sum.rounded_div(count as u128));
    let mut table = TraceTable::<BaseElement>::with_meta(TRACE_WIDTH, trace_len, create_meta(orig_pixels.len()));
    table.fill(
        |state| {
            for i in 0..TRACE_WIDTH {
                state[i] = BaseElement::ZERO;
            }
        },
        |step, state| {  // step .. index of the last updated row (starting with 0)
            let cyclic_step = step % CYCLE_LENGTH; // cyclic_step can be considered the index in the MASKs
            let roi_cyclic_step = step % CYCLE_LENGTH_ROI;
            if hash_mask[roi_cyclic_step] == BaseElement::ONE {
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
            }
            if stat_mask[roi_cyclic_step] == BaseElement::ONE {
                for i in 0..FACTOR_M {
                    state[T_SUM.begin()] += state[T_PIXELS.begin() + i * CYCLE_LENGTH + cyclic_step];
                    state[T_VAR.begin()] += (state[T_PIXELS.begin() + i * CYCLE_LENGTH + cyclic_step] - avg) * (state[T_PIXELS.begin() + i * CYCLE_LENGTH + cyclic_step] - avg);
                }
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
        let num_frames = input_length / FRAME_SIZE;

        let result_step = CYCLE_LENGTH_ROI * num_frames * CYCLE_LENGTH / NUM_ELEMS_PER_CYCLE;
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
            avg_rounded: BaseElement::new((sum.as_int()).rounded_div(num_frames as u128 * get_num_ones_in_stat_mask() as u128)),
            variance,
        }
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}