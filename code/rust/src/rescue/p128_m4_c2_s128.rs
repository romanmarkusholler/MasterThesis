// Rescue-Prime definition according to https://eprint.iacr.org/2020/1143
// Taken and adapted from https://github.com/novifinancial/winterfell/blob/main/examples/src/rescue/rescue.rs
// Parameters derived using https://github.com/KULeuven-COSIC/Marvellous

// Parameters
//   p   .. 340282366920938463463374557953744961537 (prime from winterfell::math::fields::f128)
//   m   .. 4
//   c_p .. 2
//   security at least 128 bit

use winterfell::math::{fields::f128::BaseElement, FieldElement};

pub type Elem = BaseElement;

// RESCUE CONSTANTS
// ================================================================================================

pub const NUM_ROUNDS: usize = 14;
pub const STATE_WIDTH: usize = 4;
pub const CAPACITY: usize = 2;
pub const RATE: usize = STATE_WIDTH - CAPACITY;
const ALPHA: u32 = 3;
const INV_ALPHA: u128 = 226854911280625642308916371969163307691;

const MDS: [Elem; STATE_WIDTH * STATE_WIDTH] = [
    Elem::new(340282366920938463463374557953744960808),
    Elem::new(1080),
    Elem::new(340282366920938463463374557953744961147),
    Elem::new(40),
    Elem::new(340282366920938463463374557953744932377),
    Elem::new(42471),
    Elem::new(340282366920938463463374557953744947017),
    Elem::new(1210),
    Elem::new(340282366920938463463374557953744079447),
    Elem::new(1277640),
    Elem::new(340282366920938463463374557953744532108),
    Elem::new(33880),
    Elem::new(340282366920938463463374557953720263017),
    Elem::new(35708310),
    Elem::new(340282366920938463463374557953733025977),
    Elem::new(925771),
];

const INV_MDS: [Elem; STATE_WIDTH * STATE_WIDTH] = [
    Elem::new(18020639985667067681479625318803400939),
    Elem::new(119196285838491236328880430704594968577),
    Elem::new(231409255903369280423951003551679307334),
    Elem::new(311938552114349342492438056332412246225),
    Elem::new(245698978747161380010236204726851770228),
    Elem::new(32113671753878130773768090116517402309),
    Elem::new(284248318938217584166130208504515171073),
    Elem::new(118503764402619831976614612559605579465),
    Elem::new(42476948408512208745085164298752800413),
    Elem::new(283594571303717652525183978492772054516),
    Elem::new(94047455979774690913009073579656179991),
    Elem::new(260445758149872374743470899536308888155),
    Elem::new(12603050626701424572717576220509072651),
    Elem::new(250660673575506110946271793719013778251),
    Elem::new(113894235293153614657151429548304212092),
    Elem::new(303406774346515776750608316419662860081),
];

const ROUND_CONSTANTS: [Elem; STATE_WIDTH * 2 * NUM_ROUNDS] = [
    Elem::new(252554749905287426032410435339236177993),
    Elem::new(326313117077202227382608658363071725971),
    Elem::new(283436127564251164996053541356642980485),
    Elem::new(148409698559084596343094545913152351005),
    Elem::new(129784633637387118109348657545411451832),
    Elem::new(334721381970987361377153087104020782186),
    Elem::new(256906306164171297257581429422597033315),
    Elem::new(126103441273649638990848377073385978477),
    Elem::new(242992562558979132725356692446337670218),
    Elem::new(323111873747203350364563034066016472972),
    Elem::new(167854256969530291341842495670018536864),
    Elem::new(66261431183329449572401016560613956112),
    Elem::new(332707503643181957831913123159820045201),
    Elem::new(309431953717330355442665105913755372275),
    Elem::new(200997638640820198206847315341410077181),
    Elem::new(214851145820078726772477610841901771962),
    Elem::new(254262885370163821084623485726191783139),
    Elem::new(244630091243487942074377043025872238416),
    Elem::new(304150242027274232418240825909989817206),
    Elem::new(28472731445103490784808196409880389309),
    Elem::new(119755787943390712002295837101264640690),
    Elem::new(281506597661283550002516931496480836292),
    Elem::new(300519887407532809019699703173411177603),
    Elem::new(221876949225176456507505419920912685558),
    Elem::new(134925289027258548508365472694670781908),
    Elem::new(143129342219798203971727336900106145546),
    Elem::new(256223654898024207432846499826346471683),
    Elem::new(31147186782100244372647587606474806379),
    Elem::new(102147260311686449008947017489626036358),
    Elem::new(226358108901936519406611136752297497421),
    Elem::new(277784951628500852597040010728915204379),
    Elem::new(74812516211160484832953917084826478651),
    Elem::new(289125478451405676337998900307230992347),
    Elem::new(150380234798373118503578886303215289586),
    Elem::new(112645792522461629112483247211125876711),
    Elem::new(14349422390996266495769401955912647925),
    Elem::new(265433461947929523091688102515797359010),
    Elem::new(34639103763714380233747269792456160784),
    Elem::new(274099847563679426470048467000057277742),
    Elem::new(15131952275289278100892381186648210923),
    Elem::new(88806147481272114692619820444421514468),
    Elem::new(304700557699119246585301177411749419066),
    Elem::new(305393228640902983712889121879772644357),
    Elem::new(161236819399049949062608078730468876565),
    Elem::new(11623924323161638284425097755890320483),
    Elem::new(126290704653152881987149890880912009420),
    Elem::new(164321971104799189713207082253065756234),
    Elem::new(114260190697194069069649025399711832059),
    Elem::new(295078652738328625131732993422177908851),
    Elem::new(310225988826227175492206819890429911472),
    Elem::new(229694140216567581506612698579342432888),
    Elem::new(6024461837415388340984639392074522557),
    Elem::new(174971522559056730282246944493707705439),
    Elem::new(184429873644341360575372569143525803379),
    Elem::new(172106130068281439730805308027868268865),
    Elem::new(271011430738159773088563793909157184864),
    Elem::new(212355960719655401151112133350608141156),
    Elem::new(334861330271011001104101243069327615496),
    Elem::new(114987273218437742948286485011847147433),
    Elem::new(299346572414977350269412670517092081201),
    Elem::new(72152171826418673130231459130517351768),
    Elem::new(148069753571804990671521650057633476899),
    Elem::new(318314073996603838332906284599689793644),
    Elem::new(43025894537963204475589357644712683670),
    Elem::new(246535545714498010355502669126117210625),
    Elem::new(112342531944971796645058733124393941508),
    Elem::new(206935906552041964063932016246986229762),
    Elem::new(335303402285754816036591264246220221463),
    Elem::new(334925895094789316422990446773073311393),
    Elem::new(108333887072772182646372099844591643355),
    Elem::new(236134832215051014416895379687397270708),
    Elem::new(297449264994507974018631187250118491511),
    Elem::new(30379737549359223786488243446446766973),
    Elem::new(256535737313118438633342733298096741462),
    Elem::new(13655871679538759115242897169354997798),
    Elem::new(253363973060526401423666741515121567068),
    Elem::new(249714211032619846408037334481310851880),
    Elem::new(237063379341638115915427978928338765308),
    Elem::new(198856864913077475563358268410837759958),
    Elem::new(38448680615233864931328658611445367057),
    Elem::new(186055732619837617570046914087752001702),
    Elem::new(291510259293633339466148466467294184832),
    Elem::new(156593891116260296604857920915516384535),
    Elem::new(116129548136504618117164850160356084413),
    Elem::new(219011051537172400451585089136543096830),
    Elem::new(293495723116265534657859444852644396488),
    Elem::new(27506443760631949181321956878146663691),
    Elem::new(162807279332329907266627497483493840619),
    Elem::new(100414852229420541724145466241233536391),
    Elem::new(283135891066578111315210050113160136149),
    Elem::new(326504060055957940147837581714532493571),
    Elem::new(166646293299020376357720186197358333021),
    Elem::new(336167580062685375761670820946705832786),
    Elem::new(135900773996623087107557195621759472758),
    Elem::new(247941487216893449028146949238966575419),
    Elem::new(136192418998892393624948720647409976303),
    Elem::new(81197823229356799292644365664225101911),
    Elem::new(173517215498293659248208316844286432666),
    Elem::new(306085143195055178019159023892318087140),
    Elem::new(40715214845438591101823501649671766417),
    Elem::new(40382657613840957120644301680346007829),
    Elem::new(59655470284994118058435081913180362996),
    Elem::new(278383168781587211622114600298757695213),
    Elem::new(127915200899541531717885467334976942941),
    Elem::new(30915989239601216080136785371529662463),
    Elem::new(189106292065473123859980813727027305214),
    Elem::new(21088543385967000723713613177008000704),
    Elem::new(136117707900977832229762391977904797990),
    Elem::new(338838747860515573126904718627739111490),
    Elem::new(233220207137347084357953242388702244929),
    Elem::new(283973885625150422291177874970526876720),
    Elem::new(325874450051047835515849454872276760139),
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
