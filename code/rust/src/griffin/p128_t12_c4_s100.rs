// Griffin definition according to the paper
// Inspired by https://github.com/novifinancial/winterfell/blob/main/examples/src/rescue/rescue.rs
// Parameters derived using sage script in thesis repo

// Parameters
//   p   .. 340282366920938463463374557953744961537 (prime from winterfell::math::fields::f128)
//   t   .. 12
//   security at least 100 bit
//   rate and capacity do not influence parameter generation and can be chosen freely.

use winterfell::math::{fields::f128::BaseElement, FieldElement};

pub type Elem = BaseElement;

// Griffin CONSTANTS
// ================================================================================================

pub const NUM_ROUNDS: usize = 8;
pub const STATE_WIDTH: usize = 12;
pub const CAPACITY: usize = 4;
pub const RATE: usize = STATE_WIDTH - CAPACITY;
const D: u32 = 3;
const INV_D: u128 = 226854911280625642308916371969163307691;

const MAT: [Elem; STATE_WIDTH * STATE_WIDTH] = [
    Elem::new(6),
    Elem::new(4),
    Elem::new(2),
    Elem::new(2),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(2),
    Elem::new(6),
    Elem::new(4),
    Elem::new(2),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(2),
    Elem::new(2),
    Elem::new(6),
    Elem::new(4),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(4),
    Elem::new(2),
    Elem::new(2),
    Elem::new(6),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(6),
    Elem::new(4),
    Elem::new(2),
    Elem::new(2),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(2),
    Elem::new(6),
    Elem::new(4),
    Elem::new(2),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(2),
    Elem::new(2),
    Elem::new(6),
    Elem::new(4),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(4),
    Elem::new(2),
    Elem::new(2),
    Elem::new(6),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(6),
    Elem::new(4),
    Elem::new(2),
    Elem::new(2),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(2),
    Elem::new(6),
    Elem::new(4),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(2),
    Elem::new(2),
    Elem::new(6),
    Elem::new(4),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(2),
    Elem::new(1),
    Elem::new(1),
    Elem::new(3),
    Elem::new(4),
    Elem::new(2),
    Elem::new(2),
    Elem::new(6),
];

const INV_MAT: [Elem; STATE_WIDTH * STATE_WIDTH] = [
    Elem::new(89931768400533736772463276030632596978),
    Elem::new(21875295016346044079788364439883604670),
    Elem::new(55903531708439890426125820235258100824),
    Elem::new(38889413362392967252957092337570852747),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(38889413362392967252957092337570852747),
    Elem::new(89931768400533736772463276030632596978),
    Elem::new(21875295016346044079788364439883604670),
    Elem::new(55903531708439890426125820235258100824),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(55903531708439890426125820235258100824),
    Elem::new(38889413362392967252957092337570852747),
    Elem::new(89931768400533736772463276030632596978),
    Elem::new(21875295016346044079788364439883604670),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(21875295016346044079788364439883604670),
    Elem::new(55903531708439890426125820235258100824),
    Elem::new(38889413362392967252957092337570852747),
    Elem::new(89931768400533736772463276030632596978),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(89931768400533736772463276030632596978),
    Elem::new(21875295016346044079788364439883604670),
    Elem::new(55903531708439890426125820235258100824),
    Elem::new(38889413362392967252957092337570852747),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(38889413362392967252957092337570852747),
    Elem::new(89931768400533736772463276030632596978),
    Elem::new(21875295016346044079788364439883604670),
    Elem::new(55903531708439890426125820235258100824),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(55903531708439890426125820235258100824),
    Elem::new(38889413362392967252957092337570852747),
    Elem::new(89931768400533736772463276030632596978),
    Elem::new(21875295016346044079788364439883604670),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(21875295016346044079788364439883604670),
    Elem::new(55903531708439890426125820235258100824),
    Elem::new(38889413362392967252957092337570852747),
    Elem::new(89931768400533736772463276030632596978),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(89931768400533736772463276030632596978),
    Elem::new(21875295016346044079788364439883604670),
    Elem::new(55903531708439890426125820235258100824),
    Elem::new(38889413362392967252957092337570852747),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(38889413362392967252957092337570852747),
    Elem::new(89931768400533736772463276030632596978),
    Elem::new(21875295016346044079788364439883604670),
    Elem::new(55903531708439890426125820235258100824),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(55903531708439890426125820235258100824),
    Elem::new(38889413362392967252957092337570852747),
    Elem::new(89931768400533736772463276030632596978),
    Elem::new(21875295016346044079788364439883604670),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(332990601915489782103445103140450426647),
    Elem::new(94792945070832857679082912572828953571),
    Elem::new(213891773493161319891264007856639690109),
    Elem::new(196877655147114396718095279958952442032),
    Elem::new(21875295016346044079788364439883604670),
    Elem::new(55903531708439890426125820235258100824),
    Elem::new(38889413362392967252957092337570852747),
    Elem::new(89931768400533736772463276030632596978),
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
    Elem::new(219992706639538776780574143249166236681),
    Elem::new(111868858717085401553214800625171294575),
    Elem::new(166091519019485208309832438187507502719),
    Elem::new(92605075528987592980973916705586477301),
    Elem::new(162976312484747503748482767371543480314),
    Elem::new(140179816034872192279383539457689534325),
    Elem::new(13144622592979658941386704330615843177),
    Elem::new(11074109588650505088167938287378412180),
    Elem::new(139341286121071490113612867272958857580),
    Elem::new(195002700985738333464910523344343957374),
    Elem::new(280983741404709444276092498132561963812),
    Elem::new(225048000834685228495757343025182068979),
    Elem::new(257408489283183051791230771924657635638),
    Elem::new(107236053947440707423527082907737044702),
    Elem::new(316657034674253128635021959205228552337),
    Elem::new(176859630605189644528805717300052383048),
    Elem::new(48331190367757835427450033392495333309),
    Elem::new(207217732320205864631342798465164968777),
    Elem::new(319389813447757101056014023659946806025),
    Elem::new(231864646469071852554332000210448761813),
    Elem::new(157548870756329323166980311901551012527),
    Elem::new(87722431110855579846828452796792907973),
    Elem::new(134007644785157191154419975031833397313),
    Elem::new(27414468045295873469343738945928600457),
    Elem::new(130705933749559877557147976998525934827),
    Elem::new(53376489101686661851640555890414118773),
    Elem::new(184898988800842731320447197653326153636),
    Elem::new(277140707779146391085188715485690994782),
    Elem::new(122816216618203200592723691201301521403),
    Elem::new(239240784910490472755633920499696630377),
    Elem::new(220822091823919075263526243864228147809),
    Elem::new(297717934532291881777781447334036107202),
    Elem::new(2530195196949751370553446745542216125),
    Elem::new(120635107065269767580066365765460879324),
    Elem::new(80841447230111294415845548863262916094),
    Elem::new(192224850642576066657649241279040198945),
    Elem::new(320940461271767132288233287619754516010),
    Elem::new(303476200563940515872415710292096348756),
    Elem::new(38029322414115807923199058019760698083),
    Elem::new(115575251597041011128741656732196935877),
    Elem::new(0),
    Elem::new(0),
    Elem::new(0),
    Elem::new(0),
    Elem::new(0),
    Elem::new(0),
    Elem::new(0),
    Elem::new(0),
    Elem::new(0),
    Elem::new(0),
    Elem::new(0),
    Elem::new(0),
];

const ALPHAS: [Elem; STATE_WIDTH] = [
    Elem::new(0),
    Elem::new(0),
    Elem::new(249179594608364989167094770975337358530),
    Elem::new(158076822295791514870814983996929755523),
    Elem::new(66974049983218040574535197018522152516),
    Elem::new(316153644591583029741629967993859511046),
    Elem::new(225050872279009555445350181015451908039),
    Elem::new(133948099966436081149070394037044305032),
    Elem::new(42845327653862606852790607058636702025),
    Elem::new(292024922262227596019885378033974060555),
    Elem::new(200922149949654121723605591055566457548),
    Elem::new(109819377637080647427325804077158854541),
];

const BETAS: [Elem; STATE_WIDTH] = [
    Elem::new(0),
    Elem::new(0),
    Elem::new(128557895541040233617729607877348657930),
    Elem::new(173949215243222471007543873555649670183),
    Elem::new(136173959106546712169442797034903036759),
    Elem::new(15232127131012957103426378315108757658),
    Elem::new(151406086237559669272869175350011794417),
    Elem::new(204413469505248385214396630185867185499),
    Elem::new(174254276934079104928008742822674930904),
    Elem::new(60928508524051828413705513260435030632),
    Elem::new(204718531196105019134861499452892446220),
    Elem::new(265341978029300213628102143446302216131),
];

const LAMBDAS: [Elem; 2] = [
    Elem::new(76071573668718817050189117827732608134),
    Elem::new(32310758015644895296242377566125942448),
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