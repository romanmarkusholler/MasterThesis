// Rescue-Prime definition according to https://eprint.iacr.org/2020/1143
// Taken and adapted from https://github.com/novifinancial/winterfell/blob/main/examples/src/rescue/rescue.rs
// Parameters derived using https://github.com/KULeuven-COSIC/Marvellous

// Parameters
//   p   .. 4611624995532046337 (prime from winterfell::math::fields::f62)
//   m   .. 4
//   c_p .. 2
//   security at least 128 bit

use winterfell::math::{fields::f62::BaseElement, FieldElement};

pub type Elem = BaseElement;

// RESCUE CONSTANTS
// ================================================================================================

pub const NUM_ROUNDS: usize = 14;
pub const STATE_WIDTH: usize = 4;
pub const CAPACITY: usize = 2;
pub const RATE: usize = STATE_WIDTH - CAPACITY;
const ALPHA: u32 = 3;
const INV_ALPHA: u64 = 3074416663688030891;

const MDS: [Elem; STATE_WIDTH * STATE_WIDTH] = [
    Elem::new(4611624995532045608),
    Elem::new(1080),
    Elem::new(4611624995532045947),
    Elem::new(40),
    Elem::new(4611624995532017177),
    Elem::new(42471),
    Elem::new(4611624995532031817),
    Elem::new(1210),
    Elem::new(4611624995531164247),
    Elem::new(1277640),
    Elem::new(4611624995531616908),
    Elem::new(33880),
    Elem::new(4611624995507347817),
    Elem::new(35708310),
    Elem::new(4611624995520110777),
    Elem::new(925771),
];

const INV_MDS: [Elem; STATE_WIDTH * STATE_WIDTH] = [
    Elem::new(3835753351312808841),
    Elem::new(505991969560173536),
    Elem::new(3204166179701427449),
    Elem::new(1677338490489682849),
    Elem::new(3912489244545528721),
    Elem::new(2992699836076075320),
    Elem::new(1194730044134442280),
    Elem::new(1123330866308046354),
    Elem::new(1961047666138455920),
    Elem::new(4252451032307404410),
    Elem::new(2998036551482399275),
    Elem::new(11714741135833070),
    Elem::new(683203703041784644),
    Elem::new(778093106242032509),
    Elem::new(4295326984864553447),
    Elem::new(3466626196915722075),
];

const ROUND_CONSTANTS: [Elem; STATE_WIDTH * 2 * NUM_ROUNDS] = [
    Elem::new(84059200413209450),
    Elem::new(373178937564870477),
    Elem::new(3634665217539531222),
    Elem::new(1818526052796649294),
    Elem::new(43002828310905347),
    Elem::new(1339785607435899452),
    Elem::new(3327414099846103536),
    Elem::new(3720267036147955407),
    Elem::new(1212405412276462983),
    Elem::new(2466189979681890486),
    Elem::new(3281929273804089803),
    Elem::new(2765007764398338029),
    Elem::new(3860595181968282485),
    Elem::new(1700923066901328573),
    Elem::new(1822808759769232537),
    Elem::new(2626543261588181859),
    Elem::new(1180785654043706125),
    Elem::new(3278507323242511379),
    Elem::new(2247861773607994080),
    Elem::new(888978770346910833),
    Elem::new(4065117358798607593),
    Elem::new(2535691992117626933),
    Elem::new(1892086820688304873),
    Elem::new(3667546902495623291),
    Elem::new(3667562026480151801),
    Elem::new(1900600439264387015),
    Elem::new(3743472215158074923),
    Elem::new(374156173151790171),
    Elem::new(400784247678292935),
    Elem::new(485831602057389304),
    Elem::new(688571586707975441),
    Elem::new(2014042310608406449),
    Elem::new(1901799904671064373),
    Elem::new(3778005880135162580),
    Elem::new(2391930266556619031),
    Elem::new(832601436562668997),
    Elem::new(4214057760921055958),
    Elem::new(658692901801137352),
    Elem::new(1954112702930448136),
    Elem::new(2998795451098641832),
    Elem::new(4456530904183667625),
    Elem::new(342629764430205425),
    Elem::new(3492755002973900683),
    Elem::new(3814835056106218482),
    Elem::new(607170086553088030),
    Elem::new(795069255518443540),
    Elem::new(1919302892442085635),
    Elem::new(3556741158917451700),
    Elem::new(3561926676429326404),
    Elem::new(2767297584682563727),
    Elem::new(4173772503566563981),
    Elem::new(3636870786946711035),
    Elem::new(3150131705229414069),
    Elem::new(4376594263245035840),
    Elem::new(453430431573410085),
    Elem::new(57461235190982874),
    Elem::new(1010715261332251889),
    Elem::new(3814226295063661614),
    Elem::new(612783221392610123),
    Elem::new(274680007677058177),
    Elem::new(4590496723747560349),
    Elem::new(3589444804033441211),
    Elem::new(2810438166424592924),
    Elem::new(4344573364555470373),
    Elem::new(892997045795553014),
    Elem::new(1808709039791092904),
    Elem::new(4542836651138703729),
    Elem::new(3019149084362551708),
    Elem::new(2904712339388229319),
    Elem::new(885603324699348123),
    Elem::new(2655024237486468326),
    Elem::new(589339913251683230),
    Elem::new(1641967306908921355),
    Elem::new(2209618786454888003),
    Elem::new(3506691578385905661),
    Elem::new(21251929053485279),
    Elem::new(3442460353589681627),
    Elem::new(3720862489098581928),
    Elem::new(1150646531154045107),
    Elem::new(4575835837757565626),
    Elem::new(2946269058019272865),
    Elem::new(4556767058423040792),
    Elem::new(3423759454234576830),
    Elem::new(4352253608578664076),
    Elem::new(731551570890522135),
    Elem::new(4109944482420570488),
    Elem::new(1785316767441539800),
    Elem::new(4202149893859497949),
    Elem::new(4515940521830299618),
    Elem::new(509427395813016816),
    Elem::new(2703455222057663874),
    Elem::new(2358933959583288586),
    Elem::new(4587265030045200994),
    Elem::new(437929932013931358),
    Elem::new(157878995536006837),
    Elem::new(9188722667849804),
    Elem::new(3528060750917340760),
    Elem::new(2120338204854229159),
    Elem::new(1850197494439346282),
    Elem::new(3455441796492337339),
    Elem::new(3914056536964108377),
    Elem::new(2271623193895877944),
    Elem::new(3680193581756190987),
    Elem::new(3123247226226873029),
    Elem::new(2609694948293632651),
    Elem::new(426698706492066394),
    Elem::new(698555533963097770),
    Elem::new(242609722274523402),
    Elem::new(1706096316215143515),
    Elem::new(394685350925065643),
    Elem::new(2770607924709542204),
    Elem::new(1787028432509679680),
];



pub fn hash(input_sequence: &Vec<Elem>) -> [Elem; RATE] {
    rescue_prime_hash(input_sequence)
}

// ALGORITHMS FROM https://eprint.iacr.org/2020/1143
// ================================================================================================

// Algorithm 1
pub fn rescue_prime_hash(input_sequence: &Vec<Elem>) -> [Elem; RATE] {
    assert_eq!(0, input_sequence.len() % RATE);
    let mut state = [Elem::ZERO; STATE_WIDTH];
    let mut absorb_index = 0;
    while absorb_index < input_sequence.len() {
        for i in 0..RATE {
            state[i] += input_sequence[absorb_index];
            absorb_index += 1;
        }
        rescue_xlix_permutation(&mut state);
    }
    let mut output_sequence = [Elem::ZERO; RATE];
    output_sequence.copy_from_slice(&state[..RATE]);
    output_sequence
}

// Algorithm 2
#[allow(dead_code)]
pub fn rescue_prime_wrapper(input_sequence: &Vec<Elem>) -> [Elem; RATE] {
    let mut padded_input = input_sequence.clone();
    padded_input.push(Elem::ONE);
    while (padded_input.len() % RATE) != 0 {
        padded_input.push(Elem::ZERO);
    }
    rescue_prime_hash(&padded_input)
}

// Algorithm 3
pub fn rescue_xlix_permutation(state: &mut [Elem; STATE_WIDTH]) {
    for round in 0..NUM_ROUNDS {
        let round_const = &ROUND_CONSTANTS[round * STATE_WIDTH * 2..(round + 1) * STATE_WIDTH * 2];
        apply_sbox(state);
        matrix_mul(MDS, state);
        add_constants(state, &round_const, 0);
        apply_inv_sbox(state);
        matrix_mul(MDS, state);
        add_constants(state, &round_const, STATE_WIDTH);
    }
}

// TRACE CONSTRUCTION
// ================================================================================================

pub fn apply_round(state: &mut [Elem], round: usize) {
    // determine which round constants to use
    let tmp_round = round % NUM_ROUNDS;
    let round_constants = &ROUND_CONSTANTS[tmp_round * STATE_WIDTH * 2..(tmp_round + 1) * STATE_WIDTH * 2];

    // apply first half of Rescue round
    apply_sbox(state);
    matrix_mul(MDS, state);
    add_constants(state, &round_constants, 0);

    // apply second half of Rescue round
    apply_inv_sbox(state);
    matrix_mul(MDS, state);
    add_constants(state, &round_constants, STATE_WIDTH);
}

// TRANSITION CONSTRAINTS
// ================================================================================================

/// when flag = 1, enforces constraints for a single round of Rescue hash functions
pub fn enforce_round<E: FieldElement + From<Elem>>(
    result_slice: &mut [E],
    current_slice: &[E],
    next_slice: &[E],
    round_constants: &[E],
    flag: E,
) {
    // compute the state that should result from applying the first half of Rescue round
    // to the current state of the computation
    let mut step1 = [E::ZERO; STATE_WIDTH];
    step1.copy_from_slice(current_slice);
    apply_sbox(&mut step1);
    matrix_mul(MDS, &mut step1);
    for i in 0..STATE_WIDTH {
        step1[i] += round_constants[i];
    }

    // compute the state that should result from applying the inverse for the second
    // half for Rescue round to the next step of the computation
    let mut step2 = [E::ZERO; STATE_WIDTH];
    step2.copy_from_slice(next_slice);
    for i in 0..STATE_WIDTH {
        step2[i] -= round_constants[STATE_WIDTH + i];
    }
    matrix_mul(INV_MDS, &mut step2);
    apply_sbox(&mut step2);

    // make sure that the results are equal
    for i in 0..STATE_WIDTH {
        result_slice[i] += flag * (step2[i] - step1[i]);
    }
}

// ROUND CONSTANTS
// ================================================================================================

/// when flag = 1, enforces constraints for a single round of Rescue hash functions
#[allow(dead_code)]
pub fn enforce_first_round<E: FieldElement + From<Elem>>(
    result_slice: &mut [E],
    pixels: &[E],
    current_slice: &[E],
    next_slice: &[E],
    round_constants: &[E],
    flag: E,
) {
    // compute the state that should result from applying the first half of Rescue round
    // to the current state of the computation
    let mut step1 = [E::ZERO; STATE_WIDTH];
    step1.copy_from_slice(current_slice);
    for i in 0..RATE {
        step1[i] += pixels[i];
    }
    apply_sbox(&mut step1);
    matrix_mul(MDS, &mut step1);
    for i in 0..STATE_WIDTH {
        step1[i] += round_constants[i];
    }

    // compute the state that should result from applying the inverse for the second
    // half for Rescue round to the next step of the computation
    let mut step2 = [E::ZERO; STATE_WIDTH];
    step2.copy_from_slice(next_slice);
    for i in 0..STATE_WIDTH {
        step2[i] -= round_constants[STATE_WIDTH + i];
    }
    matrix_mul(INV_MDS, &mut step2);
    apply_sbox(&mut step2);

    // make sure that the results are equal
    for i in 0..STATE_WIDTH {
        result_slice[i] += flag * (step2[i] - step1[i]);
    }
}

/// returns round constants arranged in column-major form for periodic columns
pub fn get_round_constants_periodic(cycle_length: usize, shift: usize) -> Vec<Vec<Elem>> {
    let mut constants = Vec::new();
    for _ in 0..(STATE_WIDTH * 2) {
        constants.push(vec![Elem::ZERO; cycle_length]);
    }
    for i in 0..NUM_ROUNDS {
        for j in 0..(STATE_WIDTH * 2) {
            constants[j][(i + shift) % cycle_length] = ROUND_CONSTANTS[i * STATE_WIDTH * 2 + j];
        }
    }
    constants
}

// HELPER FUNCTIONS
// ================================================================================================

fn add_constants(state: &mut [Elem], round_constants: &[Elem], offset: usize) {
    for i in 0..STATE_WIDTH {
        state[i] += round_constants[offset + i];
    }
}

fn apply_sbox<E: FieldElement>(state: &mut [E]) {
    for i in 0..STATE_WIDTH {
        state[i] = state[i].exp(ALPHA.into());
    }
}

fn apply_inv_sbox(state: &mut [Elem]) {
    for i in 0..STATE_WIDTH {
        state[i] = state[i].exp(INV_ALPHA);
    }
}

fn matrix_mul<E: FieldElement + From<Elem>>(matrix: [Elem; STATE_WIDTH * STATE_WIDTH], state: &mut [E]) {
    let mut result = [E::ZERO; STATE_WIDTH];
    for row in 0..STATE_WIDTH {
        for col in 0..STATE_WIDTH {
            result[row] += E::from(matrix[row * STATE_WIDTH + col]) * state[col];
        }
    }
    state.copy_from_slice(&result);
}
