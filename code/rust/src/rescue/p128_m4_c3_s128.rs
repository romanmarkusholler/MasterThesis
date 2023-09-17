// Rescue-Prime definition according to https://eprint.iacr.org/2020/1143
// Taken and adapted from https://github.com/novifinancial/winterfell/blob/main/examples/src/rescue/rescue.rs
// Parameters derived using https://github.com/KULeuven-COSIC/Marvellous

// Parameters
//   p   .. 340282366920938463463374557953744961537 (prime from winterfell::math::fields::f128)
//   m   .. 4
//   c_p .. 3
//   security at least 128 bit

use winterfell::math::{fields::f128::BaseElement, FieldElement};

pub type Elem = BaseElement;

// RESCUE CONSTANTS
// ================================================================================================

pub const NUM_ROUNDS: usize = 15;
pub const STATE_WIDTH: usize = 4;
pub const CAPACITY: usize = 3;
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
    Elem::new(304951987256627212945226722807003071239),
    Elem::new(84911614596503551494291714127141828970),
    Elem::new(11561348560794067007719869811382615669),
    Elem::new(65919352163409885087785995489812321454),
    Elem::new(32660298840119058254262587324518399221),
    Elem::new(237711376965396221552693414809189728601),
    Elem::new(72778421407291181665038091501953341727),
    Elem::new(248309033140501538884584081724858513657),
    Elem::new(223846896486161413801104013569055640867),
    Elem::new(308040861752156729284022588712000739649),
    Elem::new(314463876347231824998581295575506520218),
    Elem::new(221538389327339527032813766516758441868),
    Elem::new(70182308720548255768662095322618106918),
    Elem::new(102335399546756935818752090635377713525),
    Elem::new(292718382975171368356856825473002978403),
    Elem::new(318101491130971117519077339651209044302),
    Elem::new(218206413975961082221958532776019274625),
    Elem::new(11585332615296475936609054499458186339),
    Elem::new(239395796458790481892003321790309067124),
    Elem::new(299903708294933413028887537132545991509),
    Elem::new(246506347505328559463215818456241282725),
    Elem::new(235431562085806566110278801185614767175),
    Elem::new(233205602081227265454555636727449703555),
    Elem::new(211809254020284552105438082247253376665),
    Elem::new(264287843380834921106081385446141257552),
    Elem::new(33083178659224623579681702087023189949),
    Elem::new(93769638098124504386885411203296620725),
    Elem::new(10102248436484647111892346234540618333),
    Elem::new(108750227988946912093181951665845726595),
    Elem::new(113425463334799679360369538882421190008),
    Elem::new(282993639445851589314479321747725451698),
    Elem::new(106540587996860496220786790708387221939),
    Elem::new(280494931135093623457664706203854877668),
    Elem::new(220469971472938830446512736414483571558),
    Elem::new(29008815697004726304338365147952346441),
    Elem::new(146146686227806472697734430080270200754),
    Elem::new(136533279466359980855667487305236555941),
    Elem::new(43445533626804694034591627695425495140),
    Elem::new(21777595346275634699513140328366187209),
    Elem::new(149243077154527223082961226209030646045),
    Elem::new(40115084110863292885435542445301497191),
    Elem::new(25327249030127169289715198073885677810),
    Elem::new(183877208946997729685289397222890711148),
    Elem::new(307277958655402823001670211783172877188),
    Elem::new(259922040525193377321830065568912050136),
    Elem::new(229856174561892012502785270556019066792),
    Elem::new(193254457239599541372318673923811481361),
    Elem::new(135344074663049243279458096450496994091),
    Elem::new(28945209381724471491792174215709638277),
    Elem::new(325185139312249201718352642288523535328),
    Elem::new(259793749627395919119875415344098664561),
    Elem::new(320081200062023237218336188414238655207),
    Elem::new(6817322811354073268882142986619627668),
    Elem::new(52634094269354318760073507731583596831),
    Elem::new(303809314622484064544060067702091594925),
    Elem::new(160761631975164562269735843400590445667),
    Elem::new(1268954196847509381909398834855944380),
    Elem::new(278657487617164477839246121514101566311),
    Elem::new(304856618624174984580190722145299692197),
    Elem::new(34777907247995431108046617780813012161),
    Elem::new(142590557303334219473970614722763359994),
    Elem::new(39322463746626042912808822085519458295),
    Elem::new(217380525917361619308202117393161661104),
    Elem::new(49511932371883905572032490975677308612),
    Elem::new(42043556749112331268112039619577931511),
    Elem::new(116525055142101732952762331675616279079),
    Elem::new(63271291675690282979424192628843499877),
    Elem::new(56527918472106331710531435416737959661),
    Elem::new(170876974620942906772990523152399798432),
    Elem::new(159818662552768960442647437917141511859),
    Elem::new(108367623603656523127942547640625422790),
    Elem::new(51954897262069550333953652742010354927),
    Elem::new(289924163746592231680265871272860731632),
    Elem::new(67754380420996190527112111175964520353),
    Elem::new(334637205826370282357006076432326648060),
    Elem::new(238864540742801072403246665338017925137),
    Elem::new(209398403794237752215147496778770145338),
    Elem::new(280900959687155406242251592928187699962),
    Elem::new(170284624253991044795714062882973451180),
    Elem::new(263643851508952009054777223910562391993),
    Elem::new(309363000330034365304110227758551597194),
    Elem::new(214260365868735247436030332393098538583),
    Elem::new(207091926594245032333676990450767146919),
    Elem::new(154435902261971118778071443673366228467),
    Elem::new(58109843336629176358589072565913101133),
    Elem::new(67764592864961438741631571309959071145),
    Elem::new(8975940334910366895987394162210444861),
    Elem::new(156237146375548191937443163157930222515),
    Elem::new(6411239282569124219015898048575274644),
    Elem::new(303328848175807617187203799857566039631),
    Elem::new(156609342102480383298518213031752215526),
    Elem::new(267989182688715697291427602055008615181),
    Elem::new(175434644975868287006026496950302014301),
    Elem::new(171072594275000554314901349912337057985),
    Elem::new(208003398619889496473712859905048752985),
    Elem::new(300529686518807160276844237893693659938),
    Elem::new(105888310676981525813420106755522757410),
    Elem::new(58236010421601669421523505733485142793),
    Elem::new(283672243278015269390712567024291611053),
    Elem::new(182122577399963064468849486081725847520),
    Elem::new(16848357750869920478570088122575084710),
    Elem::new(201076298158746622838531461731629928347),
    Elem::new(49126909857788840900094259944655468350),
    Elem::new(197333202057684732428377225420324177582),
    Elem::new(290584072783120203004886943513748916397),
    Elem::new(91336726835559582860290097256942507305),
    Elem::new(293540413610524854173346181930946267973),
    Elem::new(229108314490361528464866780861589436793),
    Elem::new(122899203467161829433613746558034317239),
    Elem::new(323451165908731978217674238506630389913),
    Elem::new(189126458935624262496071313307212119843),
    Elem::new(243066713300100395681479008306088032768),
    Elem::new(165321805201577550151889507524905117783),
    Elem::new(259943289065294679240426317610232394495),
    Elem::new(212944382979854236076911427647438013584),
    Elem::new(208068613587060033613366562961602860986),
    Elem::new(236161452711595455046215999026446547850),
    Elem::new(221635179983576774019304182090066971283),
    Elem::new(161438027834981232604795121322109025031),
    Elem::new(309394962702090072893203216019145110838),
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
