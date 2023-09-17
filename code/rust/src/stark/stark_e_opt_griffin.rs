use winterfell::{Air, Assertion, ByteWriter, EvaluationFrame, Prover, Serializable, Trace, TraceInfo, TraceTable, TransitionConstraintDegree};
use winterfell::math::{fields::f128::BaseElement, FieldElement, StarkField};
use winterfell::ProofOptions;
use winterfell::AirContext;
use rounded_div::RoundedDiv;
use crate::griffin::p128_t12_c4_s100::{enforce_round, enforce_first_round_plus_absorb, apply_round, get_round_constants_periodic, STATE_WIDTH, RATE};
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

// STARK F (opt) parameter m: MUST BE 1 for STARK E
pub const FACTOR_M: usize = 1;

const NUM_MASKS: usize = CYCLE_LENGTH;

const ROUND_CONSTS_SHIFT: usize = 0;
pub const CYCLE_LENGTH: usize = 8;
pub const TRACE_WIDTH: usize = 186 + 10 * 3;

// for plookup: size of t, number of allowed values are in the range [0, SIZE_OF_T - 1]
// SIZE_OF_T - 1 must be a multiple of 40.
#[cfg(feature = "master_thesis_full")]
pub const SIZE_OF_T: usize = 65521;
#[cfg(feature = "master_thesis_half")]
pub const SIZE_OF_T: usize = 32761;
#[cfg(feature = "master_thesis_quarter")]
pub const SIZE_OF_T: usize = 16361;
#[cfg(feature = "master_thesis_test")]
pub const SIZE_OF_T: usize = 81;

const MASKS: [[BaseElement; CYCLE_LENGTH]; NUM_MASKS] = IDENTITY_MASK;

// AET index definitions
pub const T_PIXELS: IndexDefinition = IndexDefinition { idx: 0, size: 8 };
pub const T_PIXELS_HASH: IndexDefinition = IndexDefinition { idx: 8, size: 8 + 4};
pub const T_SUM: IndexDefinition = IndexDefinition { idx: 17 + 3, size: 1 };
pub const T_VAR: IndexDefinition = IndexDefinition { idx: 18 + 3, size: 1 };
pub const T_MIN: IndexDefinition = IndexDefinition { idx: 19 + 3, size: 1 };
pub const T_MAX: IndexDefinition = IndexDefinition { idx: 20 + 3, size: 1 };
pub const T_OMEGA_L: IndexDefinition = IndexDefinition { idx: 21 + 3, size: 8 };
pub const T_OMEGA_H: IndexDefinition = IndexDefinition { idx: 29 + 3, size: 8 };
pub const T_F_L: IndexDefinition = IndexDefinition { idx: 37 + 3, size: 1 };
pub const T_F_H: IndexDefinition = IndexDefinition { idx: 38 + 3, size: 1 };
pub const T_MED: IndexDefinition = IndexDefinition { idx: 39 + 3, size: 8 };
pub const T_OMEGA_M: IndexDefinition = IndexDefinition { idx: 47 + 3, size: 8 };
pub const T_Z: IndexDefinition = IndexDefinition { idx: 55 + 3, size: 1 };
pub const T_S: IndexDefinition = IndexDefinition { idx: 56 + 3, size: 40 };
pub const T_F: IndexDefinition = IndexDefinition { idx: 96 + 3, size: 3 };
pub const T_G: IndexDefinition = IndexDefinition { idx: 99 + 3, size: 3 };
pub const T_R: IndexDefinition = IndexDefinition { idx: 102 + 3, size: 1 };
pub const T_F_F: IndexDefinition = IndexDefinition { idx: 103 + 3, size: 1 };
pub const T_F_S: IndexDefinition = IndexDefinition { idx: 104 + 3, size: 1 };
pub const T_OMEGA_L_HASH: IndexDefinition = IndexDefinition { idx: 105 + 1 * 3, size: 8 + 4};
pub const T_OMEGA_H_HASH: IndexDefinition = IndexDefinition { idx: 114 + 2 * 3, size: 8 + 4};
pub const T_MED_HASH: IndexDefinition = IndexDefinition { idx: 123 + 3 * 3, size: 8 + 4};
pub const T_OMEGA_M_HASH: IndexDefinition = IndexDefinition { idx: 132 + 4 * 3, size: 8 + 4};
pub const T_S_HASH_1: IndexDefinition = IndexDefinition { idx: 141 + 5 * 3, size: 8 + 4};
pub const T_S_HASH_2: IndexDefinition = IndexDefinition { idx: 150 + 6 * 3, size: 8 + 4};
pub const T_S_HASH_3: IndexDefinition = IndexDefinition { idx: 159 + 7 * 3, size: 8 + 4};
pub const T_S_HASH_4: IndexDefinition = IndexDefinition { idx: 168 + 8 * 3, size: 8 + 4};
pub const T_S_HASH_5: IndexDefinition = IndexDefinition { idx: 177 + 9 * 3, size: 8 + 4};

// constraint index definitions
const C_PIXEL_ROUND_FIRST: IndexDefinition = IndexDefinition {idx: 0, size: 8 + 4};
const C_PIXEL_ROUND_REMAINING: IndexDefinition = IndexDefinition {idx: 8 + 4, size: 8 + 4};
const C_PIXEL_COPY: IndexDefinition = IndexDefinition {idx: 2 * (8 + 4), size: 8};
const C_SUM: IndexDefinition = IndexDefinition {idx: 2 * (8 + 4) + 8, size: 1};
const C_VAR: IndexDefinition = IndexDefinition {idx: 2 * (8 + 4) + 8 + 1, size: 1};
const C_MIN: IndexDefinition = IndexDefinition {idx: C_VAR.idx + C_VAR.size, size: 1};
const C_MAX: IndexDefinition = IndexDefinition {idx: C_MIN.idx + C_MIN.size, size: 1};
const C_OMEGA_L: IndexDefinition = IndexDefinition {idx: C_MAX.idx + C_MAX.size, size: 1};
const C_OMEGA_H: IndexDefinition = IndexDefinition {idx: C_OMEGA_L.idx + C_OMEGA_L.size, size: 1};
const C_OMEGA_L_ROUND_FIRST: IndexDefinition = IndexDefinition {idx: C_OMEGA_H.idx + C_OMEGA_H.size, size: 8 + 4};
const C_OMEGA_L_ROUND_REMAINING: IndexDefinition = IndexDefinition {idx: C_OMEGA_L_ROUND_FIRST.idx + C_OMEGA_L_ROUND_FIRST.size, size: 8 + 4};
const C_OMEGA_L_COPY: IndexDefinition = IndexDefinition {idx: C_OMEGA_L_ROUND_REMAINING.idx + C_OMEGA_L_ROUND_REMAINING.size, size: 8};
const C_OMEGA_H_ROUND_FIRST: IndexDefinition = IndexDefinition {idx: C_OMEGA_L_COPY.idx + C_OMEGA_L_COPY.size, size: 8 + 4};
const C_OMEGA_H_ROUND_REMAINING: IndexDefinition = IndexDefinition {idx: C_OMEGA_H_ROUND_FIRST.idx + C_OMEGA_H_ROUND_FIRST.size, size: 8 + 4};
const C_OMEGA_H_COPY: IndexDefinition = IndexDefinition {idx: C_OMEGA_H_ROUND_REMAINING.idx + C_OMEGA_H_ROUND_REMAINING.size, size: 8};
const C_F_L_U: IndexDefinition = IndexDefinition {idx: C_OMEGA_H_COPY.idx + C_OMEGA_H_COPY.size, size: 1};
const C_F_H_U: IndexDefinition = IndexDefinition {idx: C_F_L_U.idx + C_F_L_U.size, size: 1};
const C_MED_ROUND_FIRST: IndexDefinition = IndexDefinition {idx: C_F_H_U.idx + C_F_H_U.size, size: 8 + 4};
const C_MED_ROUND_REMAINING: IndexDefinition = IndexDefinition {idx: C_MED_ROUND_FIRST.idx + C_MED_ROUND_FIRST.size, size: 8 + 4};
const C_MED_COPY: IndexDefinition = IndexDefinition {idx: C_MED_ROUND_REMAINING.idx + C_MED_ROUND_REMAINING.size, size: 8};
const C_OMEGA_M_ROUND_FIRST: IndexDefinition = IndexDefinition {idx: C_MED_COPY.idx + C_MED_COPY.size, size: 8 + 4};
const C_OMEGA_M_ROUND_REMAINING: IndexDefinition = IndexDefinition {idx: C_OMEGA_M_ROUND_FIRST.idx + C_OMEGA_M_ROUND_FIRST.size, size: 8 + 4};
const C_OMEGA_M_COPY: IndexDefinition = IndexDefinition {idx: C_OMEGA_M_ROUND_REMAINING.idx + C_OMEGA_M_ROUND_REMAINING.size, size: 8};
const C_OMEGA_M: IndexDefinition = IndexDefinition {idx: C_OMEGA_M_COPY.idx + C_OMEGA_M_COPY.size, size: 1};
const C_Z: IndexDefinition = IndexDefinition {idx: C_OMEGA_M.idx + C_OMEGA_M.size, size: 1};
const C_S_ROUND_FIRST_1: IndexDefinition = IndexDefinition {idx: C_Z.idx + C_Z.size, size: 8 + 4};
const C_S_ROUND_FIRST_2: IndexDefinition = IndexDefinition {idx: C_S_ROUND_FIRST_1.idx + C_S_ROUND_FIRST_1.size, size: 8 + 4};
const C_S_ROUND_FIRST_3: IndexDefinition = IndexDefinition {idx: C_S_ROUND_FIRST_2.idx + C_S_ROUND_FIRST_2.size, size: 8 + 4};
const C_S_ROUND_FIRST_4: IndexDefinition = IndexDefinition {idx: C_S_ROUND_FIRST_3.idx + C_S_ROUND_FIRST_3.size, size: 8 + 4};
const C_S_ROUND_FIRST_5: IndexDefinition = IndexDefinition {idx: C_S_ROUND_FIRST_4.idx + C_S_ROUND_FIRST_4.size, size: 8 + 4};
const C_S_ROUND_REMAINING_1: IndexDefinition = IndexDefinition {idx: C_S_ROUND_FIRST_5.idx + C_S_ROUND_FIRST_5.size, size: 8 + 4};
const C_S_ROUND_REMAINING_2: IndexDefinition = IndexDefinition {idx: C_S_ROUND_REMAINING_1.idx + C_S_ROUND_REMAINING_1.size, size: 8 + 4};
const C_S_ROUND_REMAINING_3: IndexDefinition = IndexDefinition {idx: C_S_ROUND_REMAINING_2.idx + C_S_ROUND_REMAINING_2.size, size: 8 + 4};
const C_S_ROUND_REMAINING_4: IndexDefinition = IndexDefinition {idx: C_S_ROUND_REMAINING_3.idx + C_S_ROUND_REMAINING_3.size, size: 8 + 4};
const C_S_ROUND_REMAINING_5: IndexDefinition = IndexDefinition {idx: C_S_ROUND_REMAINING_4.idx + C_S_ROUND_REMAINING_4.size, size: 8 + 4};
const C_S_COPY: IndexDefinition = IndexDefinition {idx: C_S_ROUND_REMAINING_5.idx + C_S_ROUND_REMAINING_5.size, size: 8*5};
const C_F: IndexDefinition = IndexDefinition {idx: C_S_COPY.idx + C_S_COPY.size, size: 3};
const C_G: IndexDefinition = IndexDefinition {idx: C_F.idx + C_F.size, size: 3};
const C_R: IndexDefinition = IndexDefinition {idx: C_G.idx + C_G.size, size: 1};
const C_F_F_U: IndexDefinition = IndexDefinition {idx: C_R.idx + C_R.size, size: 1};
const C_F_F_T: IndexDefinition = IndexDefinition {idx: C_F_F_U.idx + C_F_F_U.size, size: 1};
const C_F_S_U: IndexDefinition = IndexDefinition {idx: C_F_F_T.idx + C_F_F_T.size, size: 1};
const C_F_S_T: IndexDefinition = IndexDefinition {idx: C_F_S_U.idx + C_F_S_U.size, size: 1};

// periodic column index definitions
const P_ROUND_CONSTANTS: IndexDefinition = IndexDefinition { idx: 0, size: (8 + 4) };
const P_IDENTITY: IndexDefinition = IndexDefinition { idx: (8 + 4), size: 8 };

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

pub fn get_beta_gamma(inputs: &PubInputs) -> (BaseElement, BaseElement) {
    use crate::rescue::p128_m4_c2_s128 as local_hash;
    let mut randomness: Vec<BaseElement> = vec![];
    randomness.append(inputs.hash_pixels.to_vec().clone().as_mut());
    randomness.append(inputs.hash_omega_l.to_vec().clone().as_mut());
    randomness.append(inputs.hash_omega_h.to_vec().clone().as_mut());
    randomness.append(inputs.hash_omega_m.to_vec().clone().as_mut());
    randomness.append(inputs.hash_s.to_vec().clone().as_mut());
    let local_hash_result = local_hash::hash(randomness.as_ref());
    let beta = local_hash_result[0];
    let gamma = local_hash_result[1];
    (beta, gamma)
}

pub fn get_lambda(inputs: &PubInputs) -> BaseElement {
    use crate::rescue::p128_m4_c3_s128 as local_hash;
    let mut randomness: Vec<BaseElement> = vec![];
    randomness.append(inputs.hash_pixels.to_vec().clone().as_mut());
    randomness.append(inputs.hash_med.to_vec().clone().as_mut());
    let local_hash_result = local_hash::hash(randomness.as_ref());
    local_hash_result[0]
}

pub struct PubInputs {
    pub hash_pixels: [BaseElement; RATE],
    pub hash_omega_l: [BaseElement; RATE],
    pub hash_omega_h: [BaseElement; RATE],
    pub hash_med: [BaseElement; RATE],
    pub hash_omega_m: [BaseElement; RATE],
    pub hash_s: [BaseElement; 5 * RATE],
    pub input_length: BaseElement,
    pub sum: BaseElement,
    pub avg_rounded: BaseElement,
    pub variance: BaseElement,
    pub min: BaseElement,
    pub max: BaseElement,
    pub med_low: BaseElement,
    pub med_high: BaseElement,
}

pub struct TheAir {
    context: AirContext<BaseElement>,
    hash_pixels: [BaseElement; RATE],
    hash_omega_l: [BaseElement; RATE],
    hash_omega_h: [BaseElement; RATE],
    hash_med: [BaseElement; RATE],
    hash_omega_m: [BaseElement; RATE],
    hash_s: [BaseElement; 5 * RATE],
    input_length: BaseElement,
    sum: BaseElement,
    avg_rounded: BaseElement,
    variance: BaseElement,
    min: BaseElement,
    max: BaseElement,
    med_low: BaseElement,
    med_high: BaseElement,
    pub_inputs: PubInputs,
    beta: BaseElement,
    gamma: BaseElement,
    lambda: BaseElement
}

impl Serializable for PubInputs {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        for elem in self.hash_pixels {
            target.write(elem);
        }
        for elem in self.hash_omega_l {
            target.write(elem);
        }
        for elem in self.hash_omega_h {
            target.write(elem);
        }
        for elem in self.hash_med {
            target.write(elem);
        }
        for elem in self.hash_omega_m {
            target.write(elem);
        }
        for elem in self.hash_s {
            target.write(elem);
        }
        target.write(self.input_length);
        target.write(self.sum);
        target.write(self.avg_rounded);
        target.write(self.variance);
        target.write(self.min);
        target.write(self.max);
        target.write(self.med_low);
        target.write(self.med_high);
    }
}

impl Air for TheAir {
    type BaseField = BaseElement;
    type PublicInputs = PubInputs;

    fn new(trace_info: TraceInfo, pub_inputs: Self::PublicInputs, options: ProofOptions) -> Self {
        assert_eq!(0, (SIZE_OF_T - 1) % (5 * CYCLE_LENGTH));
        assert_eq!(0, pub_inputs.input_length.as_int() % (5 * CYCLE_LENGTH) as u128);

        let mut degrees = vec![];

        for _ in 0..C_PIXEL_ROUND_FIRST.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_ROUND_REMAINING.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_COPY.size {
            degrees.push(TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]));
        }
        // C_SUM
        degrees.push(TransitionConstraintDegree::with_cycles(2, vec![CYCLE_LENGTH]));
        // C_VAR
        degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        // C_MIN
        degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        // C_MAX
        degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        // C_OMEGA_L
        degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        // C_OMEGA_H
        degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        // OMEGA_L HASH
        for _ in 0..C_PIXEL_ROUND_FIRST.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_ROUND_REMAINING.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_COPY.size {
            degrees.push(TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]));
        }
        // OMEGA_H HASH
        for _ in 0..C_PIXEL_ROUND_FIRST.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_ROUND_REMAINING.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_COPY.size {
            degrees.push(TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]));
        }
        // C_F_L_U
        degrees.push(TransitionConstraintDegree::new(2));
        // C_F_H_U
        degrees.push(TransitionConstraintDegree::new(2));
        // MED HASH
        for _ in 0..C_PIXEL_ROUND_FIRST.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_ROUND_REMAINING.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_COPY.size {
            degrees.push(TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]));
        }
        // OMEGA_M HASH
        for _ in 0..C_PIXEL_ROUND_FIRST.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_ROUND_REMAINING.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_COPY.size {
            degrees.push(TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]));
        }
        // C_OMEGA_M
        degrees.push(TransitionConstraintDegree::with_cycles(2, vec![CYCLE_LENGTH]));
        // C_Z
        degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        // S HASH
        for _ in 0..C_PIXEL_ROUND_FIRST.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_ROUND_REMAINING.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_ROUND_FIRST.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_ROUND_REMAINING.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_ROUND_FIRST.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_ROUND_REMAINING.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_ROUND_FIRST.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_ROUND_REMAINING.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_ROUND_FIRST.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_PIXEL_ROUND_REMAINING.size {
            degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        }
        for _ in 0..C_S_COPY.size {
            degrees.push(TransitionConstraintDegree::with_cycles(1, vec![CYCLE_LENGTH]));
        }
        // F
        degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH, CYCLE_LENGTH, CYCLE_LENGTH]));
        degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH]));
        degrees.push(TransitionConstraintDegree::new(2));
        // G
        degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH, CYCLE_LENGTH, CYCLE_LENGTH]));
        degrees.push(TransitionConstraintDegree::with_cycles(3, vec![CYCLE_LENGTH, CYCLE_LENGTH]));
        degrees.push(TransitionConstraintDegree::new(3));
        // R
        degrees.push(TransitionConstraintDegree::new(1));
        // C_F_F_U
        degrees.push(TransitionConstraintDegree::new(2));
        // C_F_F_T
        degrees.push(TransitionConstraintDegree::new(2));
        // C_F_S_U
        degrees.push(TransitionConstraintDegree::new(2));
        // C_F_S_T
        degrees.push(TransitionConstraintDegree::new(2));

        let (beta, gamma) = get_beta_gamma(&pub_inputs);
        let lambda = get_lambda(&pub_inputs);

        TheAir {
            context: AirContext::new(trace_info, degrees, options),
            hash_pixels: pub_inputs.hash_pixels,
            hash_omega_l: pub_inputs.hash_omega_l,
            hash_omega_h: pub_inputs.hash_omega_h,
            hash_med: pub_inputs.hash_med,
            hash_omega_m: pub_inputs.hash_omega_m,
            hash_s: pub_inputs.hash_s,
            input_length: pub_inputs.input_length,
            sum: pub_inputs.sum,
            avg_rounded: pub_inputs.avg_rounded,
            variance: pub_inputs.variance,
            min: pub_inputs.min,
            max: pub_inputs.max,
            med_low: pub_inputs.med_low,
            med_high: pub_inputs.med_high,
            pub_inputs,
            beta,
            gamma,
            lambda,
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

        enforce_first_round_plus_absorb(&mut result[C_PIXEL_ROUND_FIRST.begin()..C_PIXEL_ROUND_FIRST.end()], next.id_slice(T_PIXELS), current.id_slice(T_PIXELS_HASH), next.id_slice(T_PIXELS_HASH), round_constants, hash_first_flag);
        enforce_round(&mut result[C_PIXEL_ROUND_REMAINING.begin()..C_PIXEL_ROUND_REMAINING.end()], current.id_slice(T_PIXELS_HASH), next.id_slice(T_PIXELS_HASH), round_constants, hash_remaining_flag);
        enforce_copy(&mut result[C_PIXEL_COPY.begin()..C_PIXEL_COPY.end()], current.id_slice(T_PIXELS), next.id_slice(T_PIXELS), copy_flag);

        enforce_first_round_plus_absorb(&mut result[C_OMEGA_L_ROUND_FIRST.begin()..C_OMEGA_L_ROUND_FIRST.end()], next.id_slice(T_OMEGA_L), current.id_slice(T_OMEGA_L_HASH), next.id_slice(T_OMEGA_L_HASH), round_constants, hash_first_flag);
        enforce_round(&mut result[C_OMEGA_L_ROUND_REMAINING.begin()..C_OMEGA_L_ROUND_REMAINING.end()], current.id_slice(T_OMEGA_L_HASH), next.id_slice(T_OMEGA_L_HASH), round_constants, hash_remaining_flag);
        enforce_copy(&mut result[C_OMEGA_L_COPY.begin()..C_OMEGA_L_COPY.end()], current.id_slice(T_OMEGA_L), next.id_slice(T_OMEGA_L), copy_flag);

        enforce_first_round_plus_absorb(&mut result[C_OMEGA_H_ROUND_FIRST.begin()..C_OMEGA_H_ROUND_FIRST.end()], next.id_slice(T_OMEGA_H), current.id_slice(T_OMEGA_H_HASH), next.id_slice(T_OMEGA_H_HASH), round_constants, hash_first_flag);
        enforce_round(&mut result[C_OMEGA_H_ROUND_REMAINING.begin()..C_OMEGA_H_ROUND_REMAINING.end()], current.id_slice(T_OMEGA_H_HASH), next.id_slice(T_OMEGA_H_HASH), round_constants, hash_remaining_flag);
        enforce_copy(&mut result[C_OMEGA_H_COPY.begin()..C_OMEGA_H_COPY.end()], current.id_slice(T_OMEGA_H), next.id_slice(T_OMEGA_H), copy_flag);

        enforce_first_round_plus_absorb(&mut result[C_MED_ROUND_FIRST.begin()..C_MED_ROUND_FIRST.end()], next.id_slice(T_MED), current.id_slice(T_MED_HASH), next.id_slice(T_MED_HASH), round_constants, hash_first_flag);
        enforce_round(&mut result[C_MED_ROUND_REMAINING.begin()..C_MED_ROUND_REMAINING.end()], current.id_slice(T_MED_HASH), next.id_slice(T_MED_HASH), round_constants, hash_remaining_flag);
        enforce_copy(&mut result[C_MED_COPY.begin()..C_MED_COPY.end()], current.id_slice(T_MED), next.id_slice(T_MED), copy_flag);

        enforce_first_round_plus_absorb(&mut result[C_OMEGA_M_ROUND_FIRST.begin()..C_OMEGA_M_ROUND_FIRST.end()], next.id_slice(T_OMEGA_M), current.id_slice(T_OMEGA_M_HASH), next.id_slice(T_OMEGA_M_HASH), round_constants, hash_first_flag);
        enforce_round(&mut result[C_OMEGA_M_ROUND_REMAINING.begin()..C_OMEGA_M_ROUND_REMAINING.end()], current.id_slice(T_OMEGA_M_HASH), next.id_slice(T_OMEGA_M_HASH), round_constants, hash_remaining_flag);
        enforce_copy(&mut result[C_OMEGA_M_COPY.begin()..C_OMEGA_M_COPY.end()], current.id_slice(T_OMEGA_M), next.id_slice(T_OMEGA_M), copy_flag);

        enforce_first_round_plus_absorb(&mut result[C_S_ROUND_FIRST_1.begin()..C_S_ROUND_FIRST_1.end()], &next[T_S.begin()..T_S.begin() + 8], current.id_slice(T_S_HASH_1), next.id_slice(T_S_HASH_1), round_constants, hash_first_flag);
        enforce_first_round_plus_absorb(&mut result[C_S_ROUND_FIRST_2.begin()..C_S_ROUND_FIRST_2.end()], &next[T_S.begin()+8..T_S.begin() + 8*2], current.id_slice(T_S_HASH_2), next.id_slice(T_S_HASH_2), round_constants, hash_first_flag);
        enforce_first_round_plus_absorb(&mut result[C_S_ROUND_FIRST_3.begin()..C_S_ROUND_FIRST_3.end()], &next[T_S.begin()+8*2..T_S.begin() + 8*3], current.id_slice(T_S_HASH_3), next.id_slice(T_S_HASH_3), round_constants, hash_first_flag);
        enforce_first_round_plus_absorb(&mut result[C_S_ROUND_FIRST_4.begin()..C_S_ROUND_FIRST_4.end()], &next[T_S.begin()+8*3..T_S.begin() + 8*4], current.id_slice(T_S_HASH_4), next.id_slice(T_S_HASH_4), round_constants, hash_first_flag);
        enforce_first_round_plus_absorb(&mut result[C_S_ROUND_FIRST_5.begin()..C_S_ROUND_FIRST_5.end()], &next[T_S.begin()+8*4..T_S.begin() + 8*5], current.id_slice(T_S_HASH_5), next.id_slice(T_S_HASH_5), round_constants, hash_first_flag);
        enforce_round(&mut result[C_S_ROUND_REMAINING_1.begin()..C_S_ROUND_REMAINING_1.end()], current.id_slice(T_S_HASH_1), next.id_slice(T_S_HASH_1), round_constants, hash_remaining_flag);
        enforce_round(&mut result[C_S_ROUND_REMAINING_2.begin()..C_S_ROUND_REMAINING_2.end()], current.id_slice(T_S_HASH_2), next.id_slice(T_S_HASH_2), round_constants, hash_remaining_flag);
        enforce_round(&mut result[C_S_ROUND_REMAINING_3.begin()..C_S_ROUND_REMAINING_3.end()], current.id_slice(T_S_HASH_3), next.id_slice(T_S_HASH_3), round_constants, hash_remaining_flag);
        enforce_round(&mut result[C_S_ROUND_REMAINING_4.begin()..C_S_ROUND_REMAINING_4.end()], current.id_slice(T_S_HASH_4), next.id_slice(T_S_HASH_4), round_constants, hash_remaining_flag);
        enforce_round(&mut result[C_S_ROUND_REMAINING_5.begin()..C_S_ROUND_REMAINING_5.end()], current.id_slice(T_S_HASH_5), next.id_slice(T_S_HASH_5), round_constants, hash_remaining_flag);
        enforce_copy(&mut result[C_S_COPY.begin()..C_S_COPY.end()], current.id_slice(T_S), next.id_slice(T_S), copy_flag);

        enforce_sum(&mut result[C_SUM.begin()..C_SUM.end()], next.id_slice(T_PIXELS), identity, current.id_slice(T_SUM)[0], next.id_slice(T_SUM)[0], next.id_slice(T_F_F)[0]);
        enforce_var(&mut result[C_VAR.begin()..C_VAR.end()], next.id_slice(T_PIXELS), identity, current.id_slice(T_VAR)[0], next.id_slice(T_VAR)[0], E::from(self.avg_rounded), next.id_slice(T_F_F)[0]);

        result[C_F_L_U.begin()] += helper_u(next.id_slice(T_F_L)[0]);
        result[C_F_H_U.begin()] += helper_u(next.id_slice(T_F_H)[0]);

        enforce_min(&mut result[C_MIN.begin()..C_MIN.end()], next.id_slice(T_PIXELS), identity, current.id_slice(T_MIN)[0], next.id_slice(T_MIN)[0], next.id_slice(T_F_L)[0], next.id_slice(T_F_F)[0]);
        enforce_max(&mut result[C_MAX.begin()..C_MAX.end()], next.id_slice(T_PIXELS), identity, current.id_slice(T_MAX)[0], next.id_slice(T_MAX)[0], next.id_slice(T_F_H)[0], next.id_slice(T_F_F)[0]);
        enforce_z(&mut result[C_Z.begin()..C_Z.end()], next.id_slice(T_PIXELS), next.id_slice(T_MED), identity, current.id_slice(T_Z)[0], next.id_slice(T_Z)[0], E::from(self.lambda), next.id_slice(T_F_F)[0]);

        enforce_omega_l(&mut result[C_OMEGA_L.begin()..C_OMEGA_L.end()], next.id_slice(T_PIXELS), identity, current.id_slice(T_MIN)[0], next.id_slice(T_OMEGA_L), next.id_slice(T_F_L)[0], next.id_slice(T_F_F)[0]);
        enforce_omega_h(&mut result[C_OMEGA_H.begin()..C_OMEGA_H.end()], next.id_slice(T_PIXELS), identity, current.id_slice(T_MAX)[0], next.id_slice(T_OMEGA_H), next.id_slice(T_F_H)[0], next.id_slice(T_F_F)[0]);
        enforce_omega_m(&mut result[C_OMEGA_M.begin()..C_OMEGA_M.end()], identity, current.id_slice(T_MED), next.id_slice(T_MED), next.id_slice(T_OMEGA_M), next.id_slice(T_F_F)[0]);

        result[C_F_F_U.begin()] += helper_u(next.id_slice(T_F_F)[0]);
        result[C_F_S_U.begin()] += helper_u(next.id_slice(T_F_S)[0]);
        result[C_F_F_T.begin()] += helper_t(current.id_slice(T_F_F)[0], next.id_slice(T_F_F)[0]);
        result[C_F_S_T.begin()] += helper_t(current.id_slice(T_F_S)[0], next.id_slice(T_F_S)[0]);

        enforce_f(&mut result[C_F.begin()..C_F.end()], identity, E::from(self.beta), E::from(self.gamma), next.id_slice(T_PIXELS), next.id_slice(T_OMEGA_L), next.id_slice(T_OMEGA_H), next.id_slice(T_OMEGA_M), current.id_slice(T_F), next.id_slice(T_F), next.id_slice(T_F_F)[0]);
        enforce_g(&mut result[C_G.begin()..C_G.end()], identity, E::from(self.beta), E::from(self.gamma), current.id_slice(T_S), next.id_slice(T_S), current.id_slice(T_G), next.id_slice(T_G), next.id_slice(T_F_S)[0]);


        result[C_R.begin()] += next.id_slice(T_F)[2] - next.id_slice(T_G)[2] - next.id_slice(T_R)[0];
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let idx_result = CYCLE_LENGTH * (self.input_length.as_int() as usize) / NUM_ELEMS_PER_CYCLE;
        let idx_result_s = (4 * self.input_length.as_int() as usize + SIZE_OF_T - 1) / 5;

        let (beta, gamma) = get_beta_gamma(&self.pub_inputs);

        #[allow(non_snake_case)]
        let mut F_t = Self::BaseField::ONE;
        for i in 1..SIZE_OF_T {
            F_t *= gamma * (Self::BaseField::ONE + beta) + Self::BaseField::new((i - 1) as u128) + beta * Self::BaseField::new(i as u128);
        }

        let mut result = vec![];

        for c in 0..STATE_WIDTH {
            result.push(Assertion::single(T_PIXELS_HASH.begin() + c, 0, Self::BaseField::ZERO));
            result.push(Assertion::single(T_OMEGA_L_HASH.begin() + c, 0, Self::BaseField::ZERO));
            result.push(Assertion::single(T_OMEGA_H_HASH.begin() + c, 0, Self::BaseField::ZERO));
            result.push(Assertion::single(T_MED_HASH.begin() + c, 0, Self::BaseField::ZERO));
            result.push(Assertion::single(T_OMEGA_M_HASH.begin() + c, 0, Self::BaseField::ZERO));
            result.push(Assertion::single(T_S_HASH_1.begin() + c, 0, Self::BaseField::ZERO));
            result.push(Assertion::single(T_S_HASH_2.begin() + c, 0, Self::BaseField::ZERO));
            result.push(Assertion::single(T_S_HASH_3.begin() + c, 0, Self::BaseField::ZERO));
            result.push(Assertion::single(T_S_HASH_4.begin() + c, 0, Self::BaseField::ZERO));
            result.push(Assertion::single(T_S_HASH_5.begin() + c, 0, Self::BaseField::ZERO));
        }
        for c in 0..RATE {
            result.push(Assertion::single(T_PIXELS_HASH.begin() + c, idx_result, self.hash_pixels[c]));
            result.push(Assertion::single(T_OMEGA_L_HASH.begin() + c, idx_result, self.hash_omega_l[c]));
            result.push(Assertion::single(T_OMEGA_H_HASH.begin() + c, idx_result, self.hash_omega_h[c]));
            result.push(Assertion::single(T_MED_HASH.begin() + c, idx_result, self.hash_med[c]));
            result.push(Assertion::single(T_OMEGA_M_HASH.begin() + c, idx_result, self.hash_omega_m[c]));
            result.push(Assertion::single(T_S_HASH_1.begin() + c, idx_result_s, self.hash_s[0 * RATE + c]));
            result.push(Assertion::single(T_S_HASH_2.begin() + c, idx_result_s, self.hash_s[1 * RATE + c]));
            result.push(Assertion::single(T_S_HASH_3.begin() + c, idx_result_s, self.hash_s[2 * RATE + c]));
            result.push(Assertion::single(T_S_HASH_4.begin() + c, idx_result_s, self.hash_s[3 * RATE + c]));
            result.push(Assertion::single(T_S_HASH_5.begin() + c, idx_result_s, self.hash_s[4 * RATE + c]));
        }
        result.push(Assertion::single(T_SUM.begin(), 0, Self::BaseField::ZERO));
        result.push(Assertion::single(T_VAR.begin(), 0, Self::BaseField::ZERO));
        result.push(Assertion::single(T_SUM.begin(), idx_result, self.sum));
        result.push(Assertion::single(T_VAR.begin(), idx_result, self.variance));

        result.push(Assertion::single(T_MIN.begin(), 0, Self::BaseField::new((SIZE_OF_T - 1) as u128)));
        result.push(Assertion::single(T_MAX.begin(), 0, Self::BaseField::ZERO));
        result.push(Assertion::single(T_MIN.begin(), idx_result, self.min));
        result.push(Assertion::single(T_MAX.begin(), idx_result, self.max));

        result.push(Assertion::single(T_MED.end() - 1, 0, Self::BaseField::ZERO)); // not required

        result.push(Assertion::single(T_MED.begin() + (idx_result / 2 - 1) % CYCLE_LENGTH, idx_result / 2, self.med_low));
        result.push(Assertion::single(T_MED.begin() + (idx_result / 2) % CYCLE_LENGTH, idx_result / 2 + 1, self.med_high));

        result.push(Assertion::single(T_Z.begin(), 0, Self::BaseField::ONE));
        result.push(Assertion::single(T_Z.begin(), idx_result, Self::BaseField::ONE));

        result.push(Assertion::single(T_S.end() - 1, 0, Self::BaseField::ZERO));
        result.push(Assertion::single(T_F.end() - 1, 0, F_t));
        result.push(Assertion::single(T_G.end() - 1, 0, Self::BaseField::ONE));
        result.push(Assertion::single(T_R.begin(), idx_result, Self::BaseField::ZERO));

        result.push(Assertion::single(T_F_F.begin(), idx_result, Self::BaseField::ONE)); // implies that it is always 1 before
        result.push(Assertion::single(T_F_F.begin(), idx_result + 1, Self::BaseField::ZERO));

        result.push(Assertion::single(T_F_S.begin(), idx_result_s, Self::BaseField::ONE)); // implies that it is always 1 before
        result.push(Assertion::single(T_F_S.begin(), idx_result_s + 1, Self::BaseField::ZERO));

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

fn enforce_min<E: FieldElement + From<BaseElement>>(
    result_slice: &mut [E],
    pixels: &[E],
    identity: &[E],
    current_min: E,
    next_min: E,
    flag_l: E,
    flag: E,
) {
    let mut sum_part = E::ZERO;
    for i in 0..pixels.len() {
        sum_part += identity[i % CYCLE_LENGTH] * pixels[i];
    }
    result_slice[0] += flag * (- next_min + flag_l * current_min + (E::ONE - flag_l) * sum_part);
}

fn enforce_max<E: FieldElement + From<BaseElement>>(
    result_slice: &mut [E],
    pixels: &[E],
    identity: &[E],
    current_max: E,
    next_max: E,
    flag_h: E,
    flag: E,
) {
    let mut sum_part = E::ZERO;
    for i in 0..pixels.len() {
        sum_part += identity[i % CYCLE_LENGTH] * pixels[i];
    }
    result_slice[0] += flag * (- next_max + (E::ONE - flag_h) * current_max + flag_h * sum_part);
}

fn enforce_omega_l<E: FieldElement + From<BaseElement>>(
    result_slice: &mut [E],
    pixels: &[E],
    identity: &[E],
    current_min: E,
    next_omega_l: &[E],
    flag_l: E,
    flag: E,
) {
    let mut sum_part_pixels = E::ZERO;
    for i in 0..pixels.len() {
        sum_part_pixels += identity[i % CYCLE_LENGTH] * pixels[i];
    }
    let mut sum_part_omega_l = E::ZERO;
    for i in 0..next_omega_l.len() {
        sum_part_omega_l += identity[i % CYCLE_LENGTH] * next_omega_l[i];
    }
    result_slice[0] += flag * (flag_l * (- current_min + sum_part_pixels) + (E::ONE - flag_l) * (current_min - sum_part_pixels) - sum_part_omega_l);
}

fn enforce_omega_h<E: FieldElement + From<BaseElement>>(
    result_slice: &mut [E],
    pixels: &[E],
    identity: &[E],
    current_max: E,
    next_omega_h: &[E],
    flag_h: E,
    flag: E,
) {
    let mut sum_part_pixels = E::ZERO;
    for i in 0..pixels.len() {
        sum_part_pixels += identity[i % CYCLE_LENGTH] * pixels[i];
    }
    let mut sum_part_omega_h = E::ZERO;
    for i in 0..next_omega_h.len() {
        sum_part_omega_h += identity[i % CYCLE_LENGTH] * next_omega_h[i];
    }
    result_slice[0] += flag * (flag_h * (- current_max + sum_part_pixels) + (E::ONE - flag_h) * (current_max - sum_part_pixels) - sum_part_omega_h);
}

fn enforce_omega_m<E: FieldElement + From<BaseElement>>(
    result_slice: &mut [E],
    identity: &[E],
    current_med: &[E],
    next_med: &[E],
    next_omega_m: &[E],
    flag: E,
) {
    let mut sum_part_next_med = E::ZERO;
    for i in 0..next_med.len() {
        sum_part_next_med += identity[i % CYCLE_LENGTH] * next_med[i];
    }
    let mut sum_part_current_med = E::ZERO;
    for i in 0..current_med.len() {
        sum_part_current_med += identity[i % CYCLE_LENGTH] * current_med[(i + CYCLE_LENGTH - 1) % CYCLE_LENGTH];
    }
    let mut sum_part_next_omega_m = E::ZERO;
    for i in 0..next_omega_m.len() {
        sum_part_next_omega_m += identity[i % CYCLE_LENGTH] * next_omega_m[i];
    }
    result_slice[0] += flag * (sum_part_next_med - sum_part_current_med - sum_part_next_omega_m);
}

fn enforce_z<E: FieldElement + From<BaseElement>>(
    result_slice: &mut [E],
    pixels: &[E],
    median: &[E],
    identity: &[E],
    current_z: E,
    next_z: E,
    lambda: E,
    flag: E,
) {
    let mut sum_part_pixels = E::ZERO;
    for i in 0..pixels.len() {
        sum_part_pixels += identity[i % CYCLE_LENGTH] * pixels[i];
    }
    let mut sum_part_med = E::ZERO;
    for i in 0..median.len() {
        sum_part_med += identity[i % CYCLE_LENGTH] * median[i];
    }
    result_slice[0] += flag * (current_z * (lambda + sum_part_pixels) - next_z * (lambda + sum_part_med));
}

fn enforce_g<E: FieldElement + From<BaseElement>>(
    result_slice: &mut [E],
    identity: &[E],
    beta: E,
    gamma: E,
    current_s: &[E],
    next_s: &[E],
    current_g: &[E],
    next_g: &[E],
    flag: E,
) {
    let mut sum_part_next_s_m1 = identity[0] * current_s[39];
    for i in 1..CYCLE_LENGTH {
        sum_part_next_s_m1 += identity[i] * next_s[5 * i - 1];
    }
    let mut sum_part_next_s_0 = E::ZERO;
    for i in 0..CYCLE_LENGTH {
        sum_part_next_s_0 += identity[i] * next_s[5 * i];
    }
    let mut sum_part_next_s_1 = E::ZERO;
    for i in 0..CYCLE_LENGTH {
        sum_part_next_s_1 += identity[i] * next_s[5 * i + 1];
    }
    let mut sum_part_next_s_2 = E::ZERO;
    for i in 0..CYCLE_LENGTH {
        sum_part_next_s_2 += identity[i] * next_s[5 * i + 2];
    }
    let mut sum_part_next_s_3 = E::ZERO;
    for i in 0..CYCLE_LENGTH {
        sum_part_next_s_3 += identity[i] * next_s[5 * i + 3];
    }
    let mut sum_part_next_s_4 = E::ZERO;
    for i in 0..CYCLE_LENGTH {
        sum_part_next_s_4 += identity[i] * next_s[5 * i + 4];
    }
    result_slice[0] += - next_g[0] + (gamma * (E::ONE + beta) + sum_part_next_s_m1 + beta * sum_part_next_s_0) * (gamma * (E::ONE + beta) + sum_part_next_s_0 + beta * sum_part_next_s_1) * (gamma * (E::ONE + beta) + sum_part_next_s_1 + beta * sum_part_next_s_2);
    result_slice[1] += - next_g[1] + next_g[0] * (gamma * (E::ONE + beta) + sum_part_next_s_2 + beta * sum_part_next_s_3) * (gamma * (E::ONE + beta) + sum_part_next_s_3 + beta * sum_part_next_s_4);
    result_slice[2] += - next_g[2] + current_g[2] * helper_s(next_g[1], flag);
}

fn enforce_f<E: FieldElement + From<BaseElement>>(
    result_slice: &mut [E],
    identity: &[E],
    beta: E,
    gamma: E,
    next_pixels: &[E],
    next_omega_l: &[E],
    next_omega_h: &[E],
    next_omega_m: &[E],
    current_f: &[E],
    next_f: &[E],
    flag: E,
) {
    let mut sum_part_next_next_pixels = E::ZERO;
    for i in 0..next_pixels.len() {
        sum_part_next_next_pixels += identity[i % CYCLE_LENGTH] * next_pixels[i];
    }
    let mut sum_part_next_next_omega_l = E::ZERO;
    for i in 0..next_omega_l.len() {
        sum_part_next_next_omega_l += identity[i % CYCLE_LENGTH] * next_omega_l[i];
    }
    let mut sum_part_next_next_omega_h = E::ZERO;
    for i in 0..next_omega_h.len() {
        sum_part_next_next_omega_h += identity[i % CYCLE_LENGTH] * next_omega_h[i];
    }
    let mut sum_part_next_next_next_omega_m = E::ZERO;
    for i in 0..next_omega_m.len() {
        sum_part_next_next_next_omega_m += identity[i % CYCLE_LENGTH] * next_omega_m[i];
    }
    result_slice[0] += (E::ONE + beta) * (gamma + sum_part_next_next_pixels)
                     * (E::ONE + beta) * (gamma + sum_part_next_next_omega_l)
                     * (E::ONE + beta) * (gamma + sum_part_next_next_omega_h) - next_f[0];
    result_slice[1] += helper_s(next_f[0] * (E::ONE + beta) * (gamma + sum_part_next_next_next_omega_m), flag) - next_f[1];
    result_slice[2] += current_f[2] * next_f[1] - next_f[2];
}

pub fn build_trace(pixels: &Vec<u16>) -> TraceTable<BaseElement> {
    assert_eq!(0, pixels.len() % (5 * CYCLE_LENGTH));
    assert_eq!(0, (SIZE_OF_T - 1) % (5 * CYCLE_LENGTH));
    assert!(pixels.len() >= (SIZE_OF_T - 1));
    let trace_len = next_power_of_two(pixels.len() + 2);
    let sum = pixels.iter().map(|e| {*e as u128}).sum::<u128>();
    let avg = BaseElement::new(sum.rounded_div(pixels.len() as u128));
    let mut pixels_sorted = pixels.clone();
    pixels_sorted.sort();
    let mut omega_l = vec![0u16; pixels.len()];
    let mut omega_h = vec![0u16; pixels.len()];
    let mut omega_m = vec![0u16; pixels.len()];
    let mut min = (SIZE_OF_T - 1) as u16;
    let mut max = 0u16;
    for step in 0..pixels.len() {
        // med
        if step == 0 {
            omega_m[step] = pixels_sorted[step];
        } else {
            omega_m[step] = pixels_sorted[step] - pixels_sorted[step - 1];
        }
        // min
        if pixels[step] < min {
            omega_l[step] = min - pixels[step];
            min = pixels[step];
        } else {
            omega_l[step] = pixels[step] - min;
        }
        // max
        if pixels[step] > max {
            omega_h[step] = pixels[step] - max;
            max = pixels[step];
        } else {
            omega_h[step] = max - pixels[step];
        }
    }
    let mut s: Vec<u16> = vec![];
    s.append(&mut pixels.clone());
    s.append(&mut omega_l.clone());
    s.append(&mut omega_h.clone());
    s.append(&mut omega_m.clone());
    for t in 1..SIZE_OF_T {
        s.push(t as u16);
    }
    s.sort();
    let mut table = TraceTable::<BaseElement>::with_meta(TRACE_WIDTH, trace_len, create_meta(pixels.len()));
    table.fill(
        |state| {
            for i in 0..TRACE_WIDTH {
                state[i] = BaseElement::ZERO;
            }
            state[T_MIN.begin()] = BaseElement::new((SIZE_OF_T - 1) as u128);
            state[T_F_F.begin()] = BaseElement::ONE;
            state[T_F_S.begin()] = BaseElement::ONE;
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
                            state[T_OMEGA_L.idx + c] = BaseElement::from(omega_l[next_pixel_idx]);
                            state[T_OMEGA_H.idx + c] = BaseElement::from(omega_h[next_pixel_idx]);
                            state[T_MED.idx + c] = BaseElement::from(pixels_sorted[next_pixel_idx]);
                            state[T_OMEGA_M.idx + c] = BaseElement::from(omega_m[next_pixel_idx]);
                        }
                    }
                    // read elements into trace: s
                    for c in 0..(5 * NUM_ELEMS_PER_CYCLE) {
                        let next_pixel_idx = (5 * NUM_ELEMS_PER_CYCLE) * step / CYCLE_LENGTH + c;
                        if next_pixel_idx < s.len() {
                            state[T_S.idx + c] = BaseElement::from(s[next_pixel_idx]);
                        }
                    }
                    // absorb elements into hash state
                    for c in 0..RATE {
                        for d in 0..NUM_PIXELS_PER_ELEM {
                            state[T_PIXELS_HASH.idx + c] += state[T_PIXELS.idx + NUM_PIXELS_PER_ELEM * c + d] * COMPRESSOR[d];
                            state[T_OMEGA_L_HASH.idx + c] += state[T_OMEGA_L.idx + NUM_PIXELS_PER_ELEM * c + d] * COMPRESSOR[d];
                            state[T_OMEGA_H_HASH.idx + c] += state[T_OMEGA_H.idx + NUM_PIXELS_PER_ELEM * c + d] * COMPRESSOR[d];
                            state[T_MED_HASH.idx + c] += state[T_MED.idx + NUM_PIXELS_PER_ELEM * c + d] * COMPRESSOR[d];
                            state[T_OMEGA_M_HASH.idx + c] += state[T_OMEGA_M.idx + NUM_PIXELS_PER_ELEM * c + d] * COMPRESSOR[d];
                            state[T_S_HASH_1.idx + c] += state[T_S.idx + 0 * CYCLE_LENGTH + NUM_PIXELS_PER_ELEM * c + d] * COMPRESSOR[d];
                            state[T_S_HASH_2.idx + c] += state[T_S.idx + 1 * CYCLE_LENGTH + NUM_PIXELS_PER_ELEM * c + d] * COMPRESSOR[d];
                            state[T_S_HASH_3.idx + c] += state[T_S.idx + 2 * CYCLE_LENGTH + NUM_PIXELS_PER_ELEM * c + d] * COMPRESSOR[d];
                            state[T_S_HASH_4.idx + c] += state[T_S.idx + 3 * CYCLE_LENGTH + NUM_PIXELS_PER_ELEM * c + d] * COMPRESSOR[d];
                            state[T_S_HASH_5.idx + c] += state[T_S.idx + 4 * CYCLE_LENGTH + NUM_PIXELS_PER_ELEM * c + d] * COMPRESSOR[d];
                        }
                    }
                }
                _ => {}
            }
            // transition of flags f_f and f_s
            if step == (SIZE_OF_T + 4 * pixels.len() - 1) / 5 {
                state[T_F_S.begin()] = BaseElement::ZERO;
            }
            if step == pixels.len() {
                state[T_F_F.begin()] = BaseElement::ZERO;
            }
            // hashing
            apply_round(&mut state[T_PIXELS_HASH.begin()..T_PIXELS_HASH.end()], cyclic_step);
            apply_round(&mut state[T_OMEGA_L_HASH.begin()..T_OMEGA_L_HASH.end()], cyclic_step);
            apply_round(&mut state[T_OMEGA_H_HASH.begin()..T_OMEGA_H_HASH.end()], cyclic_step);
            apply_round(&mut state[T_MED_HASH.begin()..T_MED_HASH.end()], cyclic_step);
            apply_round(&mut state[T_OMEGA_M_HASH.begin()..T_OMEGA_M_HASH.end()], cyclic_step);
            apply_round(&mut state[T_S_HASH_1.begin()..T_S_HASH_1.end()], cyclic_step);
            apply_round(&mut state[T_S_HASH_2.begin()..T_S_HASH_2.end()], cyclic_step);
            apply_round(&mut state[T_S_HASH_3.begin()..T_S_HASH_3.end()], cyclic_step);
            apply_round(&mut state[T_S_HASH_4.begin()..T_S_HASH_4.end()], cyclic_step);
            apply_round(&mut state[T_S_HASH_5.begin()..T_S_HASH_5.end()], cyclic_step);
            // simple stats: sum, var
            for i in 0..FACTOR_M {
                state[T_SUM.begin()] += state[T_PIXELS.begin() + i * CYCLE_LENGTH + cyclic_step];
                state[T_VAR.begin()] += (state[T_PIXELS.begin() + i * CYCLE_LENGTH + cyclic_step] - avg) * (state[T_PIXELS.begin() + i * CYCLE_LENGTH + cyclic_step] - avg);
            }
            // min, max, and f_l, f_h
            if state[T_PIXELS.begin() + cyclic_step].as_int() < state[T_MIN.begin()].as_int() {
                state[T_F_L.begin()] = BaseElement::ZERO;
                state[T_MIN.begin()] = state[T_PIXELS.begin() + cyclic_step];
            } else {
                state[T_F_L.begin()] = BaseElement::ONE;
            }
            if state[T_PIXELS.begin() + cyclic_step].as_int() > state[T_MAX.begin()].as_int() {
                state[T_F_H.begin()] = BaseElement::ONE;
                state[T_MAX.begin()] = state[T_PIXELS.begin() + cyclic_step];
            } else {
                state[T_F_H.begin()] = BaseElement::ZERO;
            }
        }
    );
    // derive randomness and fill columns dependent on randomness: z, F, G, R
    let mut hash_pixels = [BaseElement::ZERO; RATE];
    let mut hash_omega_l = [BaseElement::ZERO; RATE];
    let mut hash_omega_h = [BaseElement::ZERO; RATE];
    let mut hash_med = [BaseElement::ZERO; RATE];
    let mut hash_omega_m = [BaseElement::ZERO; RATE];
    let mut hash_s = [BaseElement::ZERO; 5 * RATE];
    for i in 0..RATE {
        hash_pixels[i] = table.get(T_PIXELS_HASH.idx + i, pixels.len());
        hash_omega_l[i] = table.get(T_OMEGA_L_HASH.idx + i, pixels.len());
        hash_omega_h[i] = table.get(T_OMEGA_H_HASH.idx + i, pixels.len());
        hash_med[i] = table.get(T_MED_HASH.idx + i, pixels.len());
        hash_omega_m[i] = table.get(T_OMEGA_M_HASH.idx + i, pixels.len());
        hash_s[RATE * 0 + i] = table.get(T_S_HASH_1.idx + i, (4 * pixels.len() + SIZE_OF_T - 1) / 5);
        hash_s[RATE * 1 + i] = table.get(T_S_HASH_2.idx + i, (4 * pixels.len() + SIZE_OF_T - 1) / 5);
        hash_s[RATE * 2 + i] = table.get(T_S_HASH_3.idx + i, (4 * pixels.len() + SIZE_OF_T - 1) / 5);
        hash_s[RATE * 3 + i] = table.get(T_S_HASH_4.idx + i, (4 * pixels.len() + SIZE_OF_T - 1) / 5);
        hash_s[RATE * 4 + i] = table.get(T_S_HASH_5.idx + i, (4 * pixels.len() + SIZE_OF_T - 1) / 5);
    }
    let pub_inp = PubInputs {
        hash_pixels,
        hash_omega_l,
        hash_omega_h,
        hash_med,
        hash_omega_m,
        hash_s,
        input_length: BaseElement::new(pixels.len() as u128),
        sum: BaseElement::new(sum),
        avg_rounded: avg,
        variance: table.get(T_VAR.begin(), pixels.len()),
        min: table.get(T_MIN.begin(), pixels.len()),
        max: table.get(T_MAX.begin(), pixels.len()),
        med_low: table.get(T_MED.begin() + (pixels.len() / 2 - 1) % CYCLE_LENGTH, pixels.len() / 2),
        med_high: table.get(T_MED.begin() + (pixels.len() / 2) % CYCLE_LENGTH, pixels.len() / 2 + 1),
    };
    let (beta, gamma) = get_beta_gamma(&pub_inp);
    #[allow(non_snake_case)]
    let mut F_t = BaseElement::ONE;
    for i in 1..SIZE_OF_T {
        F_t *= gamma * (BaseElement::ONE + beta) + BaseElement::new((i - 1) as u128) + beta * BaseElement::new(i as u128);
    }
    let lambda = get_lambda(&pub_inp);
    table.set(T_Z.begin(), 0, BaseElement::ONE);
    table.set(T_F.end() - 1, 0, F_t);
    table.set(T_G.end() - 1, 0, BaseElement::ONE);
    for step in 0..(table.length() - 1) {
        let cyclic_step = step % CYCLE_LENGTH; // cyclic_step can be considered the index in the MASKs
        let current_idx = step;
        let next_idx = step + 1;

        let current_z = table.get(T_Z.begin(), current_idx);
        let next_pixel = table.get(T_PIXELS.begin() + cyclic_step, next_idx);
        let next_med = table.get(T_MED.begin() + cyclic_step, next_idx);
        let next_z = current_z * (lambda + next_pixel) / (lambda + next_med);
        table.set(T_Z.begin(), next_idx, next_z);

        let current_f = table.get(T_F.end() - 1, current_idx);
        let next_omega_l = table.get(T_OMEGA_L.begin() + cyclic_step, next_idx);
        let next_omega_h = table.get(T_OMEGA_H.begin() + cyclic_step, next_idx);
        let next_omega_m = table.get(T_OMEGA_M.begin() + cyclic_step, next_idx);
        let next_f_f = table.get(T_F_F.begin(), next_idx);
        let next_f_0 = (BaseElement::ONE + beta) * (gamma + next_pixel) * (BaseElement::ONE + beta) * (gamma + next_omega_l) * (BaseElement::ONE + beta) * (gamma + next_omega_h);
        let next_f_1 = helper_s(next_f_0 * (BaseElement::ONE + beta) * (gamma + next_omega_m), next_f_f);
        let next_f_2 = current_f * next_f_1;
        table.set(T_F.begin() + 0, next_idx, next_f_0);
        table.set(T_F.begin() + 1, next_idx, next_f_1);
        table.set(T_F.begin() + 2, next_idx, next_f_2);

        let current_g = table.get(T_G.end() - 1, current_idx);
        let next_f_s = table.get(T_F_S.begin(), next_idx);
        #[allow(unused_assignments)]
        let mut next_s_m1 = BaseElement::from(42u128);
        if cyclic_step == 0 {
            next_s_m1 = table.get(T_S.end() - 1, current_idx);
        } else {
            next_s_m1 = table.get(T_S.begin() + cyclic_step * 5 - 1, next_idx);
        }
        let next_s_0 = table.get(T_S.begin() + cyclic_step * 5 + 0, next_idx);
        let next_s_1 = table.get(T_S.begin() + cyclic_step * 5 + 1, next_idx);
        let next_s_2 = table.get(T_S.begin() + cyclic_step * 5 + 2, next_idx);
        let next_s_3 = table.get(T_S.begin() + cyclic_step * 5 + 3, next_idx);
        let next_s_4 = table.get(T_S.begin() + cyclic_step * 5 + 4, next_idx);
        let next_g_0 = (gamma * (BaseElement::ONE + beta) + next_s_m1 + beta * next_s_0) * (gamma * (BaseElement::ONE + beta) + next_s_0 + beta * next_s_1) * (gamma * (BaseElement::ONE + beta) + next_s_1 + beta * next_s_2);
        let next_g_1 = next_g_0 * (gamma * (BaseElement::ONE + beta) + next_s_2 + beta * next_s_3) * (gamma * (BaseElement::ONE + beta) + next_s_3 + beta * next_s_4);
        let next_g_2 = current_g * helper_s(next_g_1, next_f_s);
        table.set(T_G.begin() + 0, next_idx, next_g_0);
        table.set(T_G.begin() + 1, next_idx, next_g_1);
        table.set(T_G.begin() + 2, next_idx, next_g_2);

        table.set(T_R.begin(), next_idx, next_f_2 - next_g_2);
    }

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
        let result_step_a = input_length * CYCLE_LENGTH / NUM_ELEMS_PER_CYCLE;
        let result_step_b = (4 * input_length + SIZE_OF_T - 1) / 5;
        let mut hash_pixels = [BaseElement::ZERO; RATE];
        let mut hash_omega_l = [BaseElement::ZERO; RATE];
        let mut hash_omega_h = [BaseElement::ZERO; RATE];
        let mut hash_med = [BaseElement::ZERO; RATE];
        let mut hash_omega_m = [BaseElement::ZERO; RATE];
        let mut hash_s = [BaseElement::ZERO; 5 * RATE];
        for c in 0..RATE {
            hash_pixels[c] = trace.get(T_PIXELS_HASH.idx + c, result_step_a);
            hash_omega_l[c] = trace.get(T_OMEGA_L_HASH.idx + c, result_step_a);
            hash_omega_h[c] = trace.get(T_OMEGA_H_HASH.idx + c, result_step_a);
            hash_med[c] = trace.get(T_MED_HASH.idx + c, result_step_a);
            hash_omega_m[c] = trace.get(T_OMEGA_M_HASH.idx + c, result_step_a);
            hash_s[RATE * 0 + c] = trace.get(T_S_HASH_1.idx + c, result_step_b);
            hash_s[RATE * 1 + c] = trace.get(T_S_HASH_2.idx + c, result_step_b);
            hash_s[RATE * 2 + c] = trace.get(T_S_HASH_3.idx + c, result_step_b);
            hash_s[RATE * 3 + c] = trace.get(T_S_HASH_4.idx + c, result_step_b);
            hash_s[RATE * 4 + c] = trace.get(T_S_HASH_5.idx + c, result_step_b);
        }
        let sum = trace.get(T_SUM.begin(), result_step_a);
        let variance = trace.get(T_VAR.begin(), result_step_a);
        let min = trace.get(T_MIN.begin(), result_step_a);
        let max = trace.get(T_MAX.begin(), result_step_a);
        let med_low = trace.get(T_MED.begin() + (result_step_a / 2 - 1) % CYCLE_LENGTH, result_step_a / 2);
        let med_high = trace.get(T_MED.begin() + (result_step_a / 2) % CYCLE_LENGTH, result_step_a / 2 + 1);
        PubInputs {
            hash_pixels,
            hash_omega_l,
            hash_omega_h,
            hash_med,
            hash_omega_m,
            hash_s,
            input_length: BaseElement::new(input_length as u128),
            sum,
            avg_rounded: BaseElement::new((sum.as_int()).rounded_div(input_length as u128)),
            variance,
            min,
            max,
            med_low,
            med_high,
        }
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}