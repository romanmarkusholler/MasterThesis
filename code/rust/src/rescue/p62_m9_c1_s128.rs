// Rescue-Prime definition according to https://eprint.iacr.org/2020/1143
// Taken and adapted from https://github.com/novifinancial/winterfell/blob/main/examples/src/rescue/rescue.rs
// Parameters derived using https://github.com/KULeuven-COSIC/Marvellous

// Parameters
//   p   .. 4611624995532046337 (prime from winterfell::math::fields::f62)
//   m   .. 9
//   c_p .. 1
//   security at least 128 bit

use winterfell::math::{fields::f62::BaseElement, FieldElement};

pub type Elem = BaseElement;

// RESCUE CONSTANTS
// ================================================================================================

pub const NUM_ROUNDS: usize = 8;
pub const STATE_WIDTH: usize = 9;
pub const CAPACITY: usize = 1;
pub const RATE: usize = STATE_WIDTH - CAPACITY;
const ALPHA: u32 = 3;
const INV_ALPHA: u64 = 3074416663688030891;

const MDS: [Elem; STATE_WIDTH * STATE_WIDTH] = [
    Elem::new(150094635296999121),
    Elem::new(4386494480982775136),
    Elem::new(84411075413992860),
    Elem::new(4601889709531466597),
    Elem::new(364572438704838),
    Elem::new(4611620494637741339),
    Elem::new(18318658140),
    Elem::new(4611624995507837477),
    Elem::new(9841),
    Elem::new(1361307387513521921),
    Elem::new(2832323806833398177),
    Elem::new(371763438786123399),
    Elem::new(1123586449881744597),
    Elem::new(3578022083293731018),
    Elem::new(4567696267115265857),
    Elem::new(175773020450742),
    Elem::new(4611624775611313217),
    Elem::new(72636421),
    Elem::new(3690586968508376915),
    Elem::new(1188716678600029516),
    Elem::new(4404722642276688942),
    Elem::new(1392834815468635765),
    Elem::new(2410004866179382341),
    Elem::new(4074543151731888787),
    Elem::new(1286673036395336460),
    Elem::new(4610042323605607019),
    Elem::new(494894285941),
    Elem::new(1460884875119705544),
    Elem::new(3386615783772897896),
    Elem::new(201971421999830434),
    Elem::new(2690809914857215644),
    Elem::new(3005613438739961642),
    Elem::new(1137553896735229930),
    Elem::new(3419041528315999985),
    Elem::new(3140721539845838211),
    Elem::new(3287582741506063),
    Elem::new(1799234820778435744),
    Elem::new(2072052720368877933),
    Elem::new(1556994839688241047),
    Elem::new(781837143061117873),
    Elem::new(3574339685331375704),
    Elem::new(3222440231476431090),
    Elem::new(14265327237660372),
    Elem::new(2212886883903365751),
    Elem::new(3212448330282679835),
    Elem::new(418351083517431213),
    Elem::new(1604014025070032746),
    Elem::new(4416269077327681940),
    Elem::new(681286285019270020),
    Elem::new(2805410846266053252),
    Elem::new(2410684455530809505),
    Elem::new(4077427075299802133),
    Elem::new(3417121306051169026),
    Elem::new(3227560823577981851),
    Elem::new(3691476875300647805),
    Elem::new(1874414663873269341),
    Elem::new(1418653137253019787),
    Elem::new(2559323521522078511),
    Elem::new(1382789646784900202),
    Elem::new(2739816669015464653),
    Elem::new(3129816858893153678),
    Elem::new(679991697250255911),
    Elem::new(970216912235395461),
    Elem::new(2928319956530473610),
    Elem::new(2247820445494298710),
    Elem::new(4005627978492551095),
    Elem::new(1162150878861062419),
    Elem::new(4269347266999601819),
    Elem::new(1020076390998853272),
    Elem::new(1754026023882377187),
    Elem::new(3149871781959943552),
    Elem::new(2520884254441070022),
    Elem::new(3605823366462788878),
    Elem::new(1932568390090888418),
    Elem::new(210979528524628337),
    Elem::new(4080931779665215284),
    Elem::new(2254354572900271594),
    Elem::new(3743460493495557051),
    Elem::new(2371731759139912191),
    Elem::new(4228931313260232939),
    Elem::new(629343774120736994),
];

const INV_MDS: [Elem; STATE_WIDTH * STATE_WIDTH] = [
    Elem::new(3535209814333908612),
    Elem::new(2192294647579404765),
    Elem::new(2247283712851777654),
    Elem::new(1016653726681019018),
    Elem::new(4084320113214072343),
    Elem::new(677515556938913667),
    Elem::new(2421791421881488327),
    Elem::new(3644426067838966381),
    Elem::new(3238629916340680919),
    Elem::new(1855209548528077732),
    Elem::new(1472993453010258945),
    Elem::new(1795205275729888080),
    Elem::new(3589415998546501003),
    Elem::new(3294978760228179936),
    Elem::new(1778846966858158585),
    Elem::new(3997809095230362131),
    Elem::new(1212577225021150021),
    Elem::new(4061088654507655253),
    Elem::new(754377492363537555),
    Elem::new(1274059338065041488),
    Elem::new(1711978099802673295),
    Elem::new(3853263153951502696),
    Elem::new(1022446755313492063),
    Elem::new(2814266961258360083),
    Elem::new(2869424750903044655),
    Elem::new(2102868476327595162),
    Elem::new(2043814954142938352),
    Elem::new(743870189666828265),
    Elem::new(2360900441789594331),
    Elem::new(697008895183019540),
    Elem::new(275384929271023918),
    Elem::new(4322677085037880573),
    Elem::new(1358253373117268608),
    Elem::new(2976193419717151400),
    Elem::new(1722984966615577177),
    Elem::new(3989226681729841537),
    Elem::new(3574605258651933052),
    Elem::new(3776081033478750867),
    Elem::new(4197195984439072521),
    Elem::new(2323160270091604467),
    Elem::new(2968178383345228859),
    Elem::new(3207693524924509149),
    Elem::new(3189047413106611221),
    Elem::new(3494615199478891974),
    Elem::new(939172905675675913),
    Elem::new(105602560538833690),
    Elem::new(3377550819844293227),
    Elem::new(2405537603155391432),
    Elem::new(4494434426780066202),
    Elem::new(3399461679221988058),
    Elem::new(3410359846988759564),
    Elem::new(1139390336191412558),
    Elem::new(527098797354794496),
    Elem::new(4198688907584692459),
    Elem::new(2470160527543053102),
    Elem::new(4596885798055676665),
    Elem::new(1067306248085340780),
    Elem::new(681737992013306965),
    Elem::new(1520629548906454474),
    Elem::new(4404047232332091755),
    Elem::new(2694913477905427615),
    Elem::new(1346140786108729998),
    Elem::new(4276303366710150332),
    Elem::new(1404066629609082371),
    Elem::new(1041748211388490239),
    Elem::new(4525166593910048957),
    Elem::new(977282067546530790),
    Elem::new(4499663276559992087),
    Elem::new(3124570284534635445),
    Elem::new(2912864986169238494),
    Elem::new(1203274577166891218),
    Elem::new(3369488350775322085),
    Elem::new(1729798676117111118),
    Elem::new(977196139524086241),
    Elem::new(4046373689597190438),
    Elem::new(2740698142978392711),
    Elem::new(478566997738395362),
    Elem::new(1317536305202134563),
    Elem::new(3484754205530568350),
    Elem::new(917159143741939893),
    Elem::new(2754416681698366673),
];

const ROUND_CONSTANTS: [Elem; STATE_WIDTH * 2 * NUM_ROUNDS] = [
    Elem::new(3479116555304757703),
    Elem::new(1155047556099996899),
    Elem::new(427089052771468700),
    Elem::new(431082298481139051),
    Elem::new(2726498217431892440),
    Elem::new(915414314571502871),
    Elem::new(385454225924539962),
    Elem::new(1710614897861079827),
    Elem::new(4347799284751404615),
    Elem::new(34937339639775767),
    Elem::new(2874834615961943844),
    Elem::new(2872617912924433034),
    Elem::new(1916052099798880866),
    Elem::new(4506017753736519466),
    Elem::new(3679345309200506099),
    Elem::new(3787869391481781358),
    Elem::new(3151990861101109689),
    Elem::new(2641757974136818946),
    Elem::new(585302176139640508),
    Elem::new(4530630730124285737),
    Elem::new(790382751707103995),
    Elem::new(447829000152397049),
    Elem::new(4493234438415407782),
    Elem::new(3366379033989222412),
    Elem::new(4323203038464976976),
    Elem::new(2251855181029153658),
    Elem::new(3641618157117543589),
    Elem::new(2445404810245899183),
    Elem::new(3647581731494330069),
    Elem::new(2699899663630189088),
    Elem::new(61061156597312719),
    Elem::new(4322596539765352055),
    Elem::new(1704050299014426007),
    Elem::new(1534019659729860169),
    Elem::new(271049182915723608),
    Elem::new(892409126058879910),
    Elem::new(3760196303927541285),
    Elem::new(1627107491318596387),
    Elem::new(2198487769678679154),
    Elem::new(1252420745438363900),
    Elem::new(3594398923378085704),
    Elem::new(4400047412641461419),
    Elem::new(3672377192467211158),
    Elem::new(1707242180970982129),
    Elem::new(953371898520095440),
    Elem::new(4372800870710691850),
    Elem::new(510235433697610969),
    Elem::new(830761040016512618),
    Elem::new(3398847627590885621),
    Elem::new(3616434751203478813),
    Elem::new(2791550780096776652),
    Elem::new(2021550064953257921),
    Elem::new(4216432564045218516),
    Elem::new(1824234198542854533),
    Elem::new(2727190399348246265),
    Elem::new(4566134796419452876),
    Elem::new(1019957349222139381),
    Elem::new(495185720818776281),
    Elem::new(2736845784884018801),
    Elem::new(3849624882169502674),
    Elem::new(514956422266925707),
    Elem::new(795128726631560559),
    Elem::new(771930910409126772),
    Elem::new(2466065049589759591),
    Elem::new(3195115050860391041),
    Elem::new(454814501665496408),
    Elem::new(2182840498016086271),
    Elem::new(3731003969258903129),
    Elem::new(1861478041664471778),
    Elem::new(1265011680690258047),
    Elem::new(3377760548330501396),
    Elem::new(137124217307397454),
    Elem::new(4238051543850859398),
    Elem::new(4228164356574848340),
    Elem::new(3111464094934540653),
    Elem::new(2310185438666398574),
    Elem::new(4018834869962017275),
    Elem::new(1506212896309752191),
    Elem::new(3792026699561553893),
    Elem::new(489031579437078393),
    Elem::new(2912210071683438594),
    Elem::new(873787270059905174),
    Elem::new(3929361943043323257),
    Elem::new(1554269851059712578),
    Elem::new(47466377919336006),
    Elem::new(3671251071867893516),
    Elem::new(3866590107907068071),
    Elem::new(1835697137246678725),
    Elem::new(1083725543437090272),
    Elem::new(4328061113903858854),
    Elem::new(2721927690464478750),
    Elem::new(3371805806350466094),
    Elem::new(1058542491559648837),
    Elem::new(4398538541017561354),
    Elem::new(753975563407725049),
    Elem::new(1938764573704295159),
    Elem::new(216368030155626380),
    Elem::new(4396695461512595540),
    Elem::new(2464541797907241195),
    Elem::new(4262128344402760777),
    Elem::new(1300493554523041996),
    Elem::new(2962920660188935891),
    Elem::new(13336570907352833),
    Elem::new(41847148224576177),
    Elem::new(1596975541576424170),
    Elem::new(1328864207092924714),
    Elem::new(1155575769136949372),
    Elem::new(4387704409368927478),
    Elem::new(2740122345842038061),
    Elem::new(3611890152369092683),
    Elem::new(98668625291075489),
    Elem::new(4199344593106719112),
    Elem::new(3064875571082143007),
    Elem::new(1781155977771418002),
    Elem::new(1823965356338223150),
    Elem::new(3791033480382087253),
    Elem::new(4480973711277962650),
    Elem::new(3959408393141961496),
    Elem::new(3875195580749786218),
    Elem::new(1369071271077738431),
    Elem::new(2018844369623837531),
    Elem::new(590323244619354786),
    Elem::new(2762768879534493426),
    Elem::new(3743873834386968842),
    Elem::new(433353121638315937),
    Elem::new(4463469203132372635),
    Elem::new(4220524760201799584),
    Elem::new(1226700721619629859),
    Elem::new(2113671285658160646),
    Elem::new(2689242256911910282),
    Elem::new(1430053932056773788),
    Elem::new(4016942347453846629),
    Elem::new(3276633055713004461),
    Elem::new(4467812854216799073),
    Elem::new(3251806235982809425),
    Elem::new(1244686195760883321),
    Elem::new(108350848265307164),
    Elem::new(3947878545256552800),
    Elem::new(1695490117910788076),
    Elem::new(3676104303027886927),
    Elem::new(1061293990612388670),
    Elem::new(1871880748515671840),
    Elem::new(1860982292795661158),
    Elem::new(3586852323547489415),
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
