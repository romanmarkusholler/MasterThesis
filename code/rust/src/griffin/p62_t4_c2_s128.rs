// Griffin definition according to the paper
// Inspired by https://github.com/novifinancial/winterfell/blob/main/examples/src/rescue/rescue.rs
// Parameters derived using sage script in thesis repo

// Parameters
//   p   .. 4611624995532046337 (prime from winterfell::math::fields::f62)
//   t   .. 4
//   security at least 128 bit
//   rate and capacity do not influence parameter generation and can be chosen freely.

use winterfell::math::{fields::f62::BaseElement, FieldElement};

pub type Elem = BaseElement;

// Griffin CONSTANTS
// ================================================================================================

pub const NUM_ROUNDS: usize = 12;
pub const STATE_WIDTH: usize = 4;
pub const CAPACITY: usize = 2;
pub const RATE: usize = STATE_WIDTH - CAPACITY;
const D: u32 = 3;
const INV_D: u64 = 3074416663688030891;

const MAT: [Elem; STATE_WIDTH * STATE_WIDTH] = [
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
];

const INV_MAT: [Elem; STATE_WIDTH * STATE_WIDTH] = [
    Elem::new(3162257139793403203),
    Elem::new(395282142474175400),
    Elem::new(4084582138899812470),
    Elem::new(2239932140686993935),
    Elem::new(2239932140686993935),
    Elem::new(3162257139793403203),
    Elem::new(395282142474175400),
    Elem::new(4084582138899812470),
    Elem::new(4084582138899812470),
    Elem::new(2239932140686993935),
    Elem::new(3162257139793403203),
    Elem::new(395282142474175400),
    Elem::new(395282142474175400),
    Elem::new(4084582138899812470),
    Elem::new(2239932140686993935),
    Elem::new(3162257139793403203),
];

const ROUND_CONSTANTS: [Elem; STATE_WIDTH * NUM_ROUNDS] = [
    Elem::new(3158844352288017369),
    Elem::new(1195317911825411486),
    Elem::new(4608205121057886279),
    Elem::new(4008181692568088186),
    Elem::new(1017954942371112343),
    Elem::new(1040939723347347663),
    Elem::new(3439273042787950109),
    Elem::new(2938341972827614785),
    Elem::new(1054982074439128355),
    Elem::new(3378941598864555180),
    Elem::new(2655968627444521873),
    Elem::new(3496395394053647545),
    Elem::new(3224656810721129303),
    Elem::new(1409457225689559228),
    Elem::new(399784346908802669),
    Elem::new(2246468338867711618),
    Elem::new(2416421687084831339),
    Elem::new(3428136709650469831),
    Elem::new(4074697826219704129),
    Elem::new(1883685086274227575),
    Elem::new(4534624711249925081),
    Elem::new(2432723871866042896),
    Elem::new(2124532709347037506),
    Elem::new(2661205267268058692),
    Elem::new(4131330586801945043),
    Elem::new(4317709368381444920),
    Elem::new(1311980128349755585),
    Elem::new(3894459409720878477),
    Elem::new(1273089151069727973),
    Elem::new(2754117850078380637),
    Elem::new(4155398919935078309),
    Elem::new(1602817949587807433),
    Elem::new(3483922029577050900),
    Elem::new(4086381296415431721),
    Elem::new(3694431514100124504),
    Elem::new(284397335042238801),
    Elem::new(1914738171044430233),
    Elem::new(4367396245930867973),
    Elem::new(1228756722162514976),
    Elem::new(3985334418783874519),
    Elem::new(1881353522970783983),
    Elem::new(1439406162131278831),
    Elem::new(4032157944401576016),
    Elem::new(2333963439210464894),
    Elem::new(0),
    Elem::new(0),
    Elem::new(0),
    Elem::new(0),
];

const ALPHAS: [Elem; STATE_WIDTH] = [
    Elem::new(0),
    Elem::new(0),
    Elem::new(3778590617795793098),
    Elem::new(2945556240059539859),
];

const BETAS: [Elem; STATE_WIDTH] = [
    Elem::new(0),
    Elem::new(0),
    Elem::new(1936179684747344016),
    Elem::new(3133093743457329727),
];

const LAMBDAS: [Elem; 2] = [
    Elem::new(1342015051341116620),
    Elem::new(2353927619244796484),
];


pub fn hash(input_sequence: &Vec<Elem>) -> [Elem; RATE] {
    griffin_hash(input_sequence)
}

// ALGORITHMS for computing Griffin
// ================================================================================================

pub fn griffin_hash(input_sequence: &Vec<Elem>) -> [Elem; RATE] {
    assert_eq!(0, input_sequence.len() % RATE);
    let mut state = [Elem::ZERO; STATE_WIDTH]; // TODO
    let mut absorb_index = 0;
    while absorb_index < input_sequence.len() {
        for i in 0..RATE {
            state[i] += input_sequence[absorb_index];
            absorb_index += 1;
        }
        griffin_permutation(&mut state);
    }
    let mut output_sequence = [Elem::ZERO; RATE];
    output_sequence.copy_from_slice(&state[..RATE]);
    output_sequence
}

pub fn griffin_permutation(state: &mut [Elem; STATE_WIDTH]) {
    for i in 0..NUM_ROUNDS {
        apply_round(state, i);
    }
}

// TRACE CONSTRUCTION
// ================================================================================================

pub fn apply_round(state: &mut [Elem], round: usize) {
    // determine which round constants to use
    let tmp_round = round % NUM_ROUNDS;
    let round_constants = &ROUND_CONSTANTS[tmp_round * STATE_WIDTH..(tmp_round + 1) * STATE_WIDTH];

    // the first round multiplies the state additionally with the matrix before getting started
    if tmp_round == 0 {
        matrix_mul(MAT, state);
    }

    // now one round
    // non linear layer
    state[0] = state[0].exp(D.into());
    state[1] = state[1].exp(INV_D);
    let l: Elem = LAMBDAS[0] * state[0] + LAMBDAS[1] * state[1];
    for i in 2..STATE_WIDTH {
        state[i] = state[i] * (l * l + ALPHAS[i] * l + BETAS[i]);
    }
    // matrix multiplication
    matrix_mul(MAT, state);
    // adding round constants
    for i in 0..STATE_WIDTH {
        state[i] += round_constants[i];
    }
}


// TRANSITION CONSTRAINTS
// ================================================================================================

/// when flag = 1, enforces constraints for the first round of Griffin-π
/// performs M S M c
#[allow(dead_code)]
pub fn enforce_first_round_plus_absorb<E: FieldElement + From<Elem>>(
    result_slice: &mut [E],
    pixels: &[E],
    current_slice: &[E],
    next_slice: &[E],
    round_constants: &[E],
    flag: E,
) {
    // approach from the left side: multiply by MAT
    let mut left = [E::ZERO; STATE_WIDTH];
    left.copy_from_slice(current_slice);
    for i in 0..RATE {
        left[i] += pixels[i];
    }
    matrix_mul(MAT, &mut left);

    // now we can enforce one round
    enforce_round(result_slice, &left, next_slice, round_constants, flag);
}

/// when flag = 1, enforces constraints for the first round of Griffin-π
/// performs M S M c
#[allow(dead_code)]
pub fn enforce_first_round<E: FieldElement + From<Elem>>(
    result_slice: &mut [E],
    current_slice: &[E],
    next_slice: &[E],
    round_constants: &[E],
    flag: E,
) {
    // approach from the left side: multiply by MAT
    let mut left = [E::ZERO; STATE_WIDTH];
    left.copy_from_slice(current_slice);
    matrix_mul(MAT, &mut left);

    // now we can enforce one round
    enforce_round(result_slice, &left, next_slice, round_constants, flag);
}

/// when flag = 1, enforces constraints for one other round of Griffin-π
/// performs S M c
pub fn enforce_round<E: FieldElement + From<Elem>>(
    result_slice: &mut [E],
    current_slice: &[E],
    next_slice: &[E],
    round_constants: &[E],
    flag: E,
) {
    // approach from the right side: subtract constants, then multiply by INV_MAT
    let mut right = [E::ZERO; STATE_WIDTH];
    right.copy_from_slice(next_slice);
    for i in 0..STATE_WIDTH {
        right[i] -= round_constants[i];
    }
    matrix_mul(INV_MAT, &mut right);

    // now we can enforce the nonlinear layer
    enforce_non_linear(result_slice, current_slice, &right, flag);
}

fn enforce_non_linear<E: FieldElement + From<Elem>>(
    result_slice: &mut [E],
    x: &[E],
    y: &[E],
    flag: E,
) {
    result_slice[0] += flag * (y[0] - x[0].exp(D.into()));
    result_slice[1] += flag * (x[1] - y[1].exp(D.into()));
    let l: E = E::from(LAMBDAS[0]) * y[0] + E::from(LAMBDAS[1]) * y[1]; // function L(x[0], x[1])
    for i in 2..STATE_WIDTH {
        result_slice[i] += flag * (x[i] * (l * l + E::from(ALPHAS[i]) * l + E::from(BETAS[i])) - y[i]);
    }
}

// ROUND CONSTANTS
// ================================================================================================

/// returns round constants arranged in column-major form for periodic columns
pub fn get_round_constants_periodic(cycle_length: usize, shift: usize) -> Vec<Vec<Elem>> {
    let mut constants = Vec::new();
    for _ in 0..STATE_WIDTH {
        constants.push(vec![Elem::ZERO; cycle_length]);
    }
    for i in 0..NUM_ROUNDS {
        for j in 0..STATE_WIDTH {
            constants[j][(i + shift) % cycle_length] = ROUND_CONSTANTS[i * STATE_WIDTH + j];
        }
    }
    constants
}

// HELPER FUNCTIONS
// ================================================================================================

fn matrix_mul<E: FieldElement + From<Elem>>(matrix: [Elem; STATE_WIDTH * STATE_WIDTH], state: &mut [E]) {
    let mut result = [E::ZERO; STATE_WIDTH];
    for row in 0..STATE_WIDTH {
        for col in 0..STATE_WIDTH {
            result[row] += E::from(matrix[row * STATE_WIDTH + col]) * state[col];
        }
    }
    state.copy_from_slice(&result);
}