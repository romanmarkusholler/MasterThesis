// Griffin definition according to the paper
// Inspired by https://github.com/novifinancial/winterfell/blob/main/examples/src/rescue/rescue.rs
// Parameters derived using sage script in thesis repo

// Parameters
//   p   .. 340282366920938463463374557953744961537 (prime from winterfell::math::fields::f128)
//   t   .. 4
//   security at least 128 bit
//   rate and capacity do not influence parameter generation and can be chosen freely.

use winterfell::math::{fields::f128::BaseElement, FieldElement};

pub type Elem = BaseElement;

// Griffin CONSTANTS
// ================================================================================================

pub const NUM_ROUNDS: usize = 12;
pub const STATE_WIDTH: usize = 4;
pub const CAPACITY: usize = 2;
pub const RATE: usize = STATE_WIDTH - CAPACITY;
const D: u32 = 3;
const INV_D: u128 = 226854911280625642308916371969163307691;

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
    Elem::new(233336480174357803517742554025425116483),
    Elem::new(29167060021794725439717819253178139560),
    Elem::new(301392953558545496210417465616174108790),
    Elem::new(165280006790170110825067642434676124175),
    Elem::new(165280006790170110825067642434676124175),
    Elem::new(233336480174357803517742554025425116483),
    Elem::new(29167060021794725439717819253178139560),
    Elem::new(301392953558545496210417465616174108790),
    Elem::new(301392953558545496210417465616174108790),
    Elem::new(165280006790170110825067642434676124175),
    Elem::new(233336480174357803517742554025425116483),
    Elem::new(29167060021794725439717819253178139560),
    Elem::new(29167060021794725439717819253178139560),
    Elem::new(301392953558545496210417465616174108790),
    Elem::new(165280006790170110825067642434676124175),
    Elem::new(233336480174357803517742554025425116483),
];

const ROUND_CONSTANTS: [Elem; STATE_WIDTH * NUM_ROUNDS] = [
    Elem::new(294970790111306351373636413874686151821),
    Elem::new(160189431472011418998427183027739354479),
    Elem::new(55550889145373399671553726121092858466),
    Elem::new(275164185752383030818525310739461573884),
    Elem::new(38201251074942853819690530188837782157),
    Elem::new(62576778657652102143243695940893274138),
    Elem::new(329588424345411619796209026549647545760),
    Elem::new(277833782588935353313288187278579011179),
    Elem::new(44021280959178045642836381478997810870),
    Elem::new(326406391134286162613079794749493302223),
    Elem::new(330511757441020095780992868358439323843),
    Elem::new(82652686758316037556524046407399965496),
    Elem::new(228675171350738287686838730130561276916),
    Elem::new(224112638573379126418625935393785693428),
    Elem::new(64778613048988911329800247205141078159),
    Elem::new(217603433766828919995349954570396030405),
    Elem::new(80200422234881574089220840511278754356),
    Elem::new(75105765571716273279949541773384489182),
    Elem::new(259359742366690112701507630935577873933),
    Elem::new(83404028401302237313089142501002104123),
    Elem::new(13161178151423257454210838033394546966),
    Elem::new(70905281362196007540683767767251187423),
    Elem::new(302444071825868031003598979999714239976),
    Elem::new(196838326858397795372394750033112491530),
    Elem::new(111507700541055941613666035111517070703),
    Elem::new(193569732598610094044858811120409129368),
    Elem::new(191504056519622663260520654355159891695),
    Elem::new(284561537657629588804458055018962719733),
    Elem::new(117866270732042728177782916119453810877),
    Elem::new(1542213147082308645284977730431520992),
    Elem::new(88222772647598367445363102670909661349),
    Elem::new(197970947411142477399843311169063498117),
    Elem::new(243651713504449003950365473150839077311),
    Elem::new(125703797731008009672060499092197912223),
    Elem::new(135304452039729263621279398651147114084),
    Elem::new(150400300947788252961050964854718156122),
    Elem::new(78967423199903199132380881249370128518),
    Elem::new(202714971907111937486825382877502551762),
    Elem::new(194669266240218440979603488210730556215),
    Elem::new(19590513170902684529383267231746178395),
    Elem::new(256879369894401793037417086346116265484),
    Elem::new(223444523763436029506954371502782034526),
    Elem::new(268822433894440182928616124776557842222),
    Elem::new(93659545806798837818243112419862471821),
    Elem::new(0),
    Elem::new(0),
    Elem::new(0),
    Elem::new(0),
];

const ALPHAS: [Elem; STATE_WIDTH] = [
    Elem::new(0),
    Elem::new(0),
    Elem::new(140179816034872192279383539457689534325),
    Elem::new(280359632069744384558767078915379068650),
];

const BETAS: [Elem; STATE_WIDTH] = [
    Elem::new(0),
    Elem::new(0),
    Elem::new(13144622592979658941386704330615843177),
    Elem::new(52578490371918635765546817322463372708),
];

const LAMBDAS: [Elem; 2] = [
    Elem::new(11074109588650505088167938287378412180),
    Elem::new(139341286121071490113612867272958857580),
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