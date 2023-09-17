// Rescue-Prime definition according to https://eprint.iacr.org/2020/1143
// Taken and adapted from https://github.com/novifinancial/winterfell/blob/main/examples/src/rescue/rescue.rs
// Parameters derived using https://github.com/KULeuven-COSIC/Marvellous

// Parameters
//   p   .. 18446744069414584321 (prime from winterfell::math::fields::f64)
//   m   .. 9
//   c_p .. 1
//   security at least 128 bit

use winterfell::math::{fields::f64::BaseElement, FieldElement};

pub type Elem = BaseElement;

// RESCUE CONSTANTS
// ================================================================================================

pub const NUM_ROUNDS: usize = 8;
pub const STATE_WIDTH: usize = 9;
pub const CAPACITY: usize = 1;
pub const RATE: usize = STATE_WIDTH - CAPACITY;
const ALPHA: u32 = 7;
const INV_ALPHA: u64 = 10540996611094048183;

const MDS: [Elem; STATE_WIDTH * STATE_WIDTH] = [
    Elem::new(15911754940807515092),
    Elem::new(2711892286355972158),
    Elem::new(17150989262161941497),
    Elem::new(1745852583430594377),
    Elem::new(565355451240421748),
    Elem::new(16587236605753788891),
    Elem::new(667156732699554300),
    Elem::new(18446738415201823621),
    Elem::new(6725601),
    Elem::new(4000831318486715337),
    Elem::new(13866593699749449226),
    Elem::new(12844960428353252001),
    Elem::new(582714116362056623),
    Elem::new(3366018464571614479),
    Elem::new(15938247878153142367),
    Elem::new(5208160849119430188),
    Elem::new(17979409943366626563),
    Elem::new(39579496050501),
    Elem::new(12880740554289902608),
    Elem::new(10736257011154062521),
    Elem::new(9684835675499922946),
    Elem::new(17726787349382971757),
    Elem::new(3129880678747094218),
    Elem::new(10717533211705047281),
    Elem::new(6323700978479141929),
    Elem::new(13559837768921340497),
    Elem::new(7474147118893437849),
    Elem::new(8444280502348543928),
    Elem::new(18063596406833757604),
    Elem::new(1023367824000313667),
    Elem::new(17098225262737701178),
    Elem::new(13698883994234477975),
    Elem::new(11759502291953144483),
    Elem::new(16538072702756198364),
    Elem::new(13063382426484038143),
    Elem::new(10991153005139330585),
    Elem::new(14334129791615145359),
    Elem::new(2684393531687652704),
    Elem::new(9102699125972580966),
    Elem::new(6899282244058680396),
    Elem::new(7336899618183924238),
    Elem::new(12811728231465824079),
    Elem::new(3414522606789347434),
    Elem::new(11622131483008073027),
    Elem::new(5581189644877109082),
    Elem::new(12274638267902752934),
    Elem::new(10320521705305529889),
    Elem::new(534382282294584005),
    Elem::new(9894417991995566363),
    Elem::new(18390808178506636504),
    Elem::new(8691994234740331347),
    Elem::new(6296450013617354001),
    Elem::new(14783339201082448771),
    Elem::new(11047168471627717792),
    Elem::new(8221443041727100385),
    Elem::new(393559796193809889),
    Elem::new(17551880528192194831),
    Elem::new(11029403667917004080),
    Elem::new(9527404663882352754),
    Elem::new(15125710629080112088),
    Elem::new(3464609272919550073),
    Elem::new(1348518148838494851),
    Elem::new(7124446528907718334),
    Elem::new(16372941063200117221),
    Elem::new(14810743068666274131),
    Elem::new(9540729115791359379),
    Elem::new(236400869707253861),
    Elem::new(3328476643285920097),
    Elem::new(1394376324654775198),
    Elem::new(3244018084586040045),
    Elem::new(12848315908364988429),
    Elem::new(12010975199401608924),
    Elem::new(12859139447927245406),
    Elem::new(11343755749076409967),
    Elem::new(12009542032810910684),
    Elem::new(10555036868816319109),
    Elem::new(2382032143833076836),
    Elem::new(1483676889877496082),
    Elem::new(16425558919997587467),
    Elem::new(7912680464088162810),
    Elem::new(17262297830645713245),
];

const INV_MDS: [Elem; STATE_WIDTH * STATE_WIDTH] = [
    Elem::new(12539207436065832329),
    Elem::new(12077697399935358608),
    Elem::new(11489157039865439957),
    Elem::new(4022295415787053861),
    Elem::new(3826851271344202700),
    Elem::new(2762205602142483652),
    Elem::new(4011688063434574022),
    Elem::new(11145195700700186855),
    Elem::new(11912678348383205301),
    Elem::new(4727195465870018641),
    Elem::new(9920359389887667425),
    Elem::new(5882185432840443093),
    Elem::new(17741064004776248473),
    Elem::new(7911297895002287941),
    Elem::new(10171341733175619366),
    Elem::new(2083657376480454673),
    Elem::new(10776243903197042427),
    Elem::new(4573631076428555246),
    Elem::new(736023694432405199),
    Elem::new(13757068485645773643),
    Elem::new(8441959758169971861),
    Elem::new(7555928964926312149),
    Elem::new(12026351406134197362),
    Elem::new(7821482727890945600),
    Elem::new(12585455291860344),
    Elem::new(1822423469098651379),
    Elem::new(3166408246653635427),
    Elem::new(8986700519619826502),
    Elem::new(9279896563366898378),
    Elem::new(13003781105377603755),
    Elem::new(13654842869503478429),
    Elem::new(6040531618942127052),
    Elem::new(10790028874759868378),
    Elem::new(13470778785726456026),
    Elem::new(7979522825782598098),
    Elem::new(9027637183994064988),
    Elem::new(4903771519896946287),
    Elem::new(4880363692783577635),
    Elem::new(14022866956744736042),
    Elem::new(1482704328144708807),
    Elem::new(15556761641939977419),
    Elem::new(4165101601570367338),
    Elem::new(5642321549502873499),
    Elem::new(6994112155574369328),
    Elem::new(16138972831500780930),
    Elem::new(9097855105810106273),
    Elem::new(11304812409172332644),
    Elem::new(5495002765127845140),
    Elem::new(17349450637489595112),
    Elem::new(5711132269931307421),
    Elem::new(12428043081259626558),
    Elem::new(1120990037650345611),
    Elem::new(7030824490151679826),
    Elem::new(4248865481065498700),
    Elem::new(3935402967907460435),
    Elem::new(6216932751277754590),
    Elem::new(9244633293430859140),
    Elem::new(2315497648650709885),
    Elem::new(11837871652379070246),
    Elem::new(4914401822520226103),
    Elem::new(4327473378007705992),
    Elem::new(14868197986830246925),
    Elem::new(16126564776654303969),
    Elem::new(14340817518001988765),
    Elem::new(10854148653441472529),
    Elem::new(17665873453485278662),
    Elem::new(9485609402492606939),
    Elem::new(15595726145931404890),
    Elem::new(16357593446793026710),
    Elem::new(15060277806204701300),
    Elem::new(16200124235946144513),
    Elem::new(13567037823605465940),
    Elem::new(7153203116068863772),
    Elem::new(5026931394711878012),
    Elem::new(7906190408160118526),
    Elem::new(11498192696621955873),
    Elem::new(7824129118715884826),
    Elem::new(361143884638050460),
    Elem::new(15105352412984880743),
    Elem::new(7056196256360339342),
    Elem::new(11855636989396365731),
];

const ROUND_CONSTANTS: [Elem; STATE_WIDTH * 2 * NUM_ROUNDS] = [
    Elem::new(11634186502472574198),
    Elem::new(6160739341619519953),
    Elem::new(7323081760153609453),
    Elem::new(8963628711597965530),
    Elem::new(4149892941148131853),
    Elem::new(13392526775580837002),
    Elem::new(2902166722686047941),
    Elem::new(13991326508314867409),
    Elem::new(2488560454014152603),
    Elem::new(6589508082742749233),
    Elem::new(18247813120147042662),
    Elem::new(442740224846270413),
    Elem::new(3415151924891236790),
    Elem::new(12310116918197892841),
    Elem::new(741097377643492758),
    Elem::new(3158731613958111555),
    Elem::new(306461993853536522),
    Elem::new(7531121335350040140),
    Elem::new(13362828541400729095),
    Elem::new(9746547419839901949),
    Elem::new(17056922490939767838),
    Elem::new(6275088288299434625),
    Elem::new(7253464592685389224),
    Elem::new(3552311482074555071),
    Elem::new(8083799438757397172),
    Elem::new(142272000890307341),
    Elem::new(7791470101890884594),
    Elem::new(6242683846232322859),
    Elem::new(13748086603176625504),
    Elem::new(12484293929015308531),
    Elem::new(11682377363663629729),
    Elem::new(190940869500880458),
    Elem::new(7961520526967218160),
    Elem::new(3683055571769776652),
    Elem::new(5885113286812764018),
    Elem::new(8245362076693448234),
    Elem::new(13079775779943421440),
    Elem::new(1620961428577843121),
    Elem::new(778444694079784823),
    Elem::new(8367294517619394630),
    Elem::new(16913583215255148593),
    Elem::new(12027672443150261680),
    Elem::new(8772625586926615905),
    Elem::new(2718347592145780756),
    Elem::new(8335873057761546064),
    Elem::new(6227944134271018258),
    Elem::new(9322670578358126770),
    Elem::new(2168048942895109980),
    Elem::new(15979654620270583959),
    Elem::new(9141602852296911340),
    Elem::new(8817805750132634127),
    Elem::new(1968964494180478117),
    Elem::new(17622397526528660742),
    Elem::new(16503556592707099435),
    Elem::new(18273895031829889270),
    Elem::new(1321087486765436820),
    Elem::new(78415818672121230),
    Elem::new(14882430102630261210),
    Elem::new(6717740389368810766),
    Elem::new(7479247088245631537),
    Elem::new(3721110060940407688),
    Elem::new(9821066463506441256),
    Elem::new(10662804994061016896),
    Elem::new(317926704037002797),
    Elem::new(13528559436893401630),
    Elem::new(5270291099396640797),
    Elem::new(12525510962600419874),
    Elem::new(12046075271534831525),
    Elem::new(15939813417791268403),
    Elem::new(2770151940319237050),
    Elem::new(14670520714632457748),
    Elem::new(7277811732510184389),
    Elem::new(11272445397465126994),
    Elem::new(13273744148558875758),
    Elem::new(15473733302614861003),
    Elem::new(1282074552648985116),
    Elem::new(3193633016975683389),
    Elem::new(808011392535991319),
    Elem::new(7141445196898187372),
    Elem::new(3099529342507611584),
    Elem::new(14494529111209225649),
    Elem::new(3695655934617541015),
    Elem::new(7161875658083699289),
    Elem::new(14033176761059849714),
    Elem::new(2476015945196789679),
    Elem::new(1886591589154523073),
    Elem::new(18009875213995788887),
    Elem::new(9941219996580288856),
    Elem::new(1707234801820779879),
    Elem::new(10588896186592712331),
    Elem::new(15136457836148607367),
    Elem::new(578858167205168541),
    Elem::new(14003127434110554212),
    Elem::new(11384857834810805347),
    Elem::new(8281278113059682180),
    Elem::new(4289437737596482772),
    Elem::new(1887616044283307726),
    Elem::new(7638372657279702042),
    Elem::new(823583252690255073),
    Elem::new(5802242146970684498),
    Elem::new(12685402610805034584),
    Elem::new(5984639629755047919),
    Elem::new(3143150313580957224),
    Elem::new(3808295989256224180),
    Elem::new(9031456236375675106),
    Elem::new(16648113137820323784),
    Elem::new(8478631596329143300),
    Elem::new(3337421942438363467),
    Elem::new(18332541066180775839),
    Elem::new(12887003281194749607),
    Elem::new(8848749007283380124),
    Elem::new(5043933798289008119),
    Elem::new(13183428172828960524),
    Elem::new(501154734860702338),
    Elem::new(2587123205704457440),
    Elem::new(11927154311983495633),
    Elem::new(14399814432187306933),
    Elem::new(15455258618667509979),
    Elem::new(39908091910793913),
    Elem::new(3110538457242353741),
    Elem::new(14325316117082168709),
    Elem::new(13134098071182217256),
    Elem::new(9263004538761520364),
    Elem::new(9517009767905172353),
    Elem::new(12676592586192779073),
    Elem::new(8320405991577206743),
    Elem::new(1852118676601291827),
    Elem::new(3424540211247367133),
    Elem::new(16185557941545874892),
    Elem::new(6290116232463382221),
    Elem::new(17157033980965369412),
    Elem::new(7244137642311645728),
    Elem::new(17567374927212900180),
    Elem::new(756080945628154301),
    Elem::new(14703759431979323173),
    Elem::new(12063154632571008285),
    Elem::new(4230140130386427821),
    Elem::new(6674034109838897821),
    Elem::new(378453408288985050),
    Elem::new(8579368219637036690),
    Elem::new(12240816202420880976),
    Elem::new(7285297108732611743),
    Elem::new(2563102255109144961),
    Elem::new(76962020379598151),
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
