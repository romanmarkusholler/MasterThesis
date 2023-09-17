// Rescue-Prime definition according to https://eprint.iacr.org/2020/1143
// Taken and adapted from https://github.com/novifinancial/winterfell/blob/main/examples/src/rescue/rescue.rs
// Parameters derived using https://github.com/KULeuven-COSIC/Marvellous

// Parameters
//   p   .. 4611624995532046337 (prime from winterfell::math::fields::f62)
//   m   .. 4
//   c_p .. 3
//   security at least 128 bit

use winterfell::math::{fields::f62::BaseElement, FieldElement};

pub type Elem = BaseElement;

// RESCUE CONSTANTS
// ================================================================================================

pub const NUM_ROUNDS: usize = 15;
pub const STATE_WIDTH: usize = 4;
pub const CAPACITY: usize = 3;
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
    Elem::new(701104025118976884),
    Elem::new(3829713303167962021),
    Elem::new(3493944243763378989),
    Elem::new(2252961374134939104),
    Elem::new(966764098212384772),
    Elem::new(4287820704531755629),
    Elem::new(2166403522297030610),
    Elem::new(618002652599374859),
    Elem::new(3536254835601188773),
    Elem::new(2861149515806932376),
    Elem::new(1516959689842592780),
    Elem::new(456557379546570520),
    Elem::new(2607995063063254379),
    Elem::new(1097665560591934335),
    Elem::new(3874014569716568621),
    Elem::new(4328552626972127492),
    Elem::new(3117515282420604315),
    Elem::new(519891142352399260),
    Elem::new(3483850909183648329),
    Elem::new(900205189715881671),
    Elem::new(1481905392709594888),
    Elem::new(3052845865309885592),
    Elem::new(1375806108908699034),
    Elem::new(459747437070800832),
    Elem::new(2660675196195863879),
    Elem::new(2473961292001812919),
    Elem::new(1688424447542423205),
    Elem::new(1072879623692832709),
    Elem::new(2043624929897786422),
    Elem::new(2924069510687746780),
    Elem::new(3918159506389054584),
    Elem::new(3774335076707280751),
    Elem::new(1752492718010628726),
    Elem::new(1073415166620835594),
    Elem::new(726159715901766514),
    Elem::new(2832104204500498498),
    Elem::new(1478223063348339174),
    Elem::new(1289244337861918911),
    Elem::new(3823587875630140191),
    Elem::new(3588039810137281697),
    Elem::new(3463987949232753759),
    Elem::new(1797932357542146820),
    Elem::new(1315812089826815911),
    Elem::new(2852221434136956255),
    Elem::new(127452407719209377),
    Elem::new(2808984078052786515),
    Elem::new(841583956540965906),
    Elem::new(4422855586625390271),
    Elem::new(3611388390062903382),
    Elem::new(2031922030059909696),
    Elem::new(2697829724397558788),
    Elem::new(2267583763654229653),
    Elem::new(1280756594131679795),
    Elem::new(1370848993366626326),
    Elem::new(2467993517698147583),
    Elem::new(3957167007821752654),
    Elem::new(3968552685888324323),
    Elem::new(1077597098581876493),
    Elem::new(3532806674550843103),
    Elem::new(1380430401468485690),
    Elem::new(775669586176670288),
    Elem::new(2530435515917871822),
    Elem::new(3868902280832932300),
    Elem::new(705390454674938413),
    Elem::new(4422929781402517350),
    Elem::new(2892417706985188713),
    Elem::new(4348168646885837690),
    Elem::new(2538981446171348305),
    Elem::new(2034740708531478292),
    Elem::new(305737397904337515),
    Elem::new(3192833973243637715),
    Elem::new(4421690400022691196),
    Elem::new(3760337393237790454),
    Elem::new(4371382887156124577),
    Elem::new(516988938743947622),
    Elem::new(4351062387731842131),
    Elem::new(665905576053960907),
    Elem::new(1747121705445084414),
    Elem::new(1808644441886091899),
    Elem::new(483058335268190401),
    Elem::new(1558263247006248009),
    Elem::new(1275376746065912363),
    Elem::new(4457599726575435044),
    Elem::new(871915951082478040),
    Elem::new(2685409183107778460),
    Elem::new(4550507981018864744),
    Elem::new(1260288007192469457),
    Elem::new(4025665905376751681),
    Elem::new(160018679881676489),
    Elem::new(1525322003903609074),
    Elem::new(4451140015131640929),
    Elem::new(3999641557131347821),
    Elem::new(2269466241244753090),
    Elem::new(3075446558169756042),
    Elem::new(3659040822406101045),
    Elem::new(3504624307026803629),
    Elem::new(4553458974677283679),
    Elem::new(3614487929967415309),
    Elem::new(2820728715312404135),
    Elem::new(2508645214701419950),
    Elem::new(4511767726020150650),
    Elem::new(4148028150874685199),
    Elem::new(3874565705990269238),
    Elem::new(3787893980853325736),
    Elem::new(821847811480125713),
    Elem::new(3621230732181582318),
    Elem::new(1801780046903573595),
    Elem::new(3174993236396752667),
    Elem::new(302309678139414184),
    Elem::new(2474070870877720658),
    Elem::new(2139081438680603181),
    Elem::new(817021037372976679),
    Elem::new(73452920264340984),
    Elem::new(2248383741259597603),
    Elem::new(1524155271118572611),
    Elem::new(2157883594944041856),
    Elem::new(388555251834568743),
    Elem::new(3359700906391244644),
    Elem::new(1085426295298952844),
    Elem::new(3890091597351953114),
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
