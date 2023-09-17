// Rescue-Prime definition according to https://eprint.iacr.org/2020/1143
// Taken and adapted from https://github.com/novifinancial/winterfell/blob/main/examples/src/rescue/rescue.rs
// Parameters derived using https://github.com/KULeuven-COSIC/Marvellous

// Parameters
//   p   .. 340282366920938463463374557953744961537 (prime from winterfell::math::fields::f128)
//   m   .. 9
//   c_p .. 1
//   security at least 128 bit

use winterfell::math::{fields::f128::BaseElement, FieldElement};

pub type Elem = BaseElement;

// RESCUE CONSTANTS
// ================================================================================================

pub const NUM_ROUNDS: usize = 8;
pub const STATE_WIDTH: usize = 9;
pub const CAPACITY: usize = 1;
pub const RATE: usize = STATE_WIDTH - CAPACITY;
const ALPHA: u32 = 3;
const INV_ALPHA: u128 = 226854911280625642308916371969163307691;

const MDS: [Elem; STATE_WIDTH * STATE_WIDTH] = [
    Elem::new(150094635296999121),
    Elem::new(340282366920938463463149427439195690336),
    Elem::new(84411075413992860),
    Elem::new(340282366920938463463364822667744381797),
    Elem::new(364572438704838),
    Elem::new(340282366920938463463374553452850656539),
    Elem::new(18318658140),
    Elem::new(340282366920938463463374557953720752677),
    Elem::new(9841),
    Elem::new(1477081305957768349761),
    Elem::new(340282366920938461248015258909664071617),
    Elem::new(830464262634554464059),
    Elem::new(340282366920938463367654019497453733057),
    Elem::new(3578022083293731018),
    Elem::new(340282366920938463463330629225328181057),
    Elem::new(175773020450742),
    Elem::new(340282366920938463463374557733824228417),
    Elem::new(72636421),
    Elem::new(10902337119274288189585941),
    Elem::new(340282366920922112265621116423314299677),
    Elem::new(6129103051534490589064140),
    Elem::new(340282366920937757157506327072060715056),
    Elem::new(26385516604305016476318),
    Elem::new(340282366920938463140023726422701560397),
    Elem::new(1286673036395336460),
    Elem::new(340282366920938463463372975281818522219),
    Elem::new(494894285941),
    Elem::new(74280977358883194447399657861),
    Elem::new(340282366809533560559102349761084062337),
    Elem::new(41758207694766455878941719400),
    Elem::new(340282366916126655152737768305002590337),
    Elem::new(179718510858368911187836077),
    Elem::new(340282366920936262382017994360084004737),
    Elem::new(8742448408061543808600),
    Elem::new(340282366920938463452680404506994660737),
    Elem::new(3287582741506063),
    Elem::new(493448532595061060714075927170623),
    Elem::new(340281626860025222258639632111371827735),
    Elem::new(277396989820105406282268067810980),
    Elem::new(340282334957138432026603756253961717317),
    Elem::new(1193750249204165634400502061594),
    Elem::new(340282366906321119535778575950744594740),
    Elem::new(58023022992049963173346020),
    Elem::new(340282366920938392617192638478820451957),
    Elem::new(21658948312410865183),
    Elem::new(3250891947867863390247053555200504143),
    Elem::new(335406770191301931314646938843156637377),
    Elem::new(1827515058473445723489323454811459578),
    Elem::new(340071788261635189888022255808831580097),
    Elem::new(7864291806006227817276975764611134),
    Elem::new(340282270630051601091434166691585938497),
    Elem::new(382145525879388576506005172823),
    Elem::new(340282366920472148038924217019224186177),
    Elem::new(142299528422960399756323),
    Elem::new(260889072481016946483921979409357576789),
    Elem::new(294109695457304815011263569543330296235),
    Elem::new(96897786123648020966790891596799875825),
    Elem::new(317912742718020576130300337690805571243),
    Elem::new(51667907444403809555847542124007809234),
    Elem::new(339649756075661666195744923271868570317),
    Elem::new(2510445527801052949502558181396180),
    Elem::new(340282363858174627685294031214892512580),
    Elem::new(934054234760012359481199283),
    Elem::new(194542495007434960394582255983923586243),
    Elem::new(104404390561882821492740793700399886259),
    Elem::new(31015733125887000676791684053161446567),
    Elem::new(177473572991922008668894803954934702656),
    Elem::new(225693703897588175263197025163593303860),
    Elem::new(271259291625999272530694212816871392781),
    Elem::new(16478009365511174087881242740613722400),
    Elem::new(340262264978264552243515110485852110337),
    Elem::new(6129263888495201102915629695046),
    Elem::new(188693918348033658130796882009222201279),
    Elem::new(314243109041589500728057538205852247049),
    Elem::new(274553328639948771850735264514343203581),
    Elem::new(310813334522360543261976075967969844628),
    Elem::new(58189218743095556616922042146577748196),
    Elem::new(88532793352623448815797177624516430079),
    Elem::new(257968030906974364479467734461159660011),
    Elem::new(208377884906813703378925729595147376377),
    Elem::new(40216143252770054194345243936096486),
];

const INV_MDS: [Elem; STATE_WIDTH * STATE_WIDTH] = [
    Elem::new(207704205238170807874062389651402246184),
    Elem::new(338985877705667148435373284629855945642),
    Elem::new(129028267600780770615661928550545766843),
    Elem::new(308035008707470404458798574304191182997),
    Elem::new(128520708499505858811967733849558211851),
    Elem::new(75192627449100971018093805348251269149),
    Elem::new(304723430247806995709152718418888185473),
    Elem::new(247138038315326480697662666345484012828),
    Elem::new(302366037761801343159474246624292948256),
    Elem::new(127826805904477456371474222478224881465),
    Elem::new(58658143825271814927663889085592315510),
    Elem::new(312249633317719663349408437894842488201),
    Elem::new(214300933317574560218555665032290192921),
    Elem::new(284396721402661659389038897151481097747),
    Elem::new(250642028598419737645395164192907767034),
    Elem::new(326756605713292613148743956415561946579),
    Elem::new(64564033929793286393378167005124959170),
    Elem::new(62016928595481525873214390512699159059),
    Elem::new(248581119490725734937433971221259595049),
    Elem::new(205001520256672409877405228205435574405),
    Elem::new(200096610221992325040847115543314292955),
    Elem::new(15223170475099380041360367767094442338),
    Elem::new(235366553140646608237869232108973717383),
    Elem::new(162307162630682801506851542241168907512),
    Elem::new(52675509994437226630257244779552696631),
    Elem::new(335285735573611416179235296489092358465),
    Elem::new(246874452820824414865612791412833222948),
    Elem::new(283673471255946562146645390311833638442),
    Elem::new(209223032476035404560919156087419693012),
    Elem::new(211512112805934564584850722873108293522),
    Elem::new(239776030660594538269138961758799208508),
    Elem::new(68210418817622944190939851321990599837),
    Elem::new(222109338954160942132440798066799260073),
    Elem::new(258338197395874177917585304146894183425),
    Elem::new(338175469353412666797355362713810876851),
    Elem::new(210676129806048980180371800441814015553),
    Elem::new(220783014807171136331164688074748627144),
    Elem::new(309965259718335944489975028792798283342),
    Elem::new(96673590909058413519603421380578045352),
    Elem::new(205785556561014960463687902712791399561),
    Elem::new(72930044192120504138776809082161800375),
    Elem::new(40863127682090904559911953136810632891),
    Elem::new(147156319027505131468688770623137082831),
    Elem::new(5465313383088119386627759574466685670),
    Elem::new(261507241403368739495061898437487288983),
    Elem::new(47112215418871085025936967894656035967),
    Elem::new(257010822502178578101896171521076674054),
    Elem::new(20670720855140215404881506561419650370),
    Elem::new(275063314552737150020455701571996505359),
    Elem::new(154434305573626427596859079056551184503),
    Elem::new(119775195884931758842035438232130131140),
    Elem::new(110335483825478749319799953283019030264),
    Elem::new(93758101272597109289270377026652209913),
    Elem::new(282969307798192780252363036667478424579),
    Elem::new(52011660187227698998307018524445124622),
    Elem::new(191078345965907944922431974561774457545),
    Elem::new(86336838853676487241056288607338298724),
    Elem::new(125093328226800820985177164342949053500),
    Elem::new(10689792024438114156388383608175598304),
    Elem::new(329939868120376749051950735773432225615),
    Elem::new(23499294867058318465853361897065892690),
    Elem::new(278372710189966668032426818312990960568),
    Elem::new(264107629248301051999906486186808234581),
    Elem::new(302228698991084466216875753830938701241),
    Elem::new(280345530278773498915828360047963227132),
    Elem::new(333099072682993183200689623504181423558),
    Elem::new(27875343830661393369872307155840999075),
    Elem::new(47883408063394345577170731401514016906),
    Elem::new(20045034666478649987681661989026523732),
    Elem::new(282120995912747862095020676645287935430),
    Elem::new(122980149585923452096449943205160728029),
    Elem::new(284833600592635465857283731988811252583),
    Elem::new(243710843188765407683950167325811244364),
    Elem::new(164728799513981287864597798438362590906),
    Elem::new(203691322335641954496554143160915473208),
    Elem::new(207184049598426413618777837396461507381),
    Elem::new(182286914752134147886045712500843417287),
    Elem::new(104791328630945774391628390734939042132),
    Elem::new(292925681130628074351389456735241751204),
    Elem::new(37505641512749851214506703376700564141),
    Elem::new(264587253941419405809422580099449217063),
];

const ROUND_CONSTANTS: [Elem; STATE_WIDTH * 2 * NUM_ROUNDS] = [
    Elem::new(193550505184713801886850691874302114155),
    Elem::new(73477471125177237583945581530965976084),
    Elem::new(336279502154193780034454449733345567343),
    Elem::new(135317725925888010323160512940118246176),
    Elem::new(266931630451858520653088100069771445550),
    Elem::new(237770002109414465779793104379907619059),
    Elem::new(251299408841043447171430588873060041328),
    Elem::new(141995952081615579885284935062985598220),
    Elem::new(300182467202507410331878748551511513542),
    Elem::new(320075451354510020983792900244985957861),
    Elem::new(269779363768581274492460063332673414165),
    Elem::new(45036748174771534644141297594097519625),
    Elem::new(250152852415518823160110342920383101614),
    Elem::new(141204021396194811524568050260616313985),
    Elem::new(226023795345908938822698683664768836998),
    Elem::new(68016657657960555088899086457404615157),
    Elem::new(310568148306309576007272421615723419953),
    Elem::new(188238717735419854450969626086205873984),
    Elem::new(315595579688700958781450462853941751036),
    Elem::new(335071301308265324057006302408599545095),
    Elem::new(168532892142917869495326788262302221826),
    Elem::new(180570938030506022348466096605944041175),
    Elem::new(307369499718017023049634848822129441903),
    Elem::new(77433643948225191453041092242433595722),
    Elem::new(295139306307645914685855337322600842658),
    Elem::new(260740346631472864827766948555925279892),
    Elem::new(227769852515038471447868885629776840281),
    Elem::new(158092478799051831795002055728021774322),
    Elem::new(145476093056051208877954288316354978488),
    Elem::new(17087304111100405288854472099043738307),
    Elem::new(43712851500629936729849843799210141537),
    Elem::new(237973631083374254468453358942325264572),
    Elem::new(95356111157200790184678721109726594773),
    Elem::new(313628495808117313950512297675264414711),
    Elem::new(191361409201358755306323073245227334035),
    Elem::new(153440992947256037399888310080271077427),
    Elem::new(15960277761427441017887125485411281),
    Elem::new(57168887688442182170290344460444447591),
    Elem::new(185826864243918899927217370730769532505),
    Elem::new(287295756955166317899809526630322446088),
    Elem::new(194889633971151288348446096082000980044),
    Elem::new(22605793917940761417126970213938380422),
    Elem::new(43699897651006894914437247202949025048),
    Elem::new(155566768136717140391381802258505284397),
    Elem::new(165418844653189269018278743958064921361),
    Elem::new(203042806373456641562425845014451542206),
    Elem::new(152758758047131086963036262682364247867),
    Elem::new(261784292711457601981723392807367423774),
    Elem::new(304854676002186534236029361165930015543),
    Elem::new(295087140369911151771911697710015989208),
    Elem::new(27653551232270576790010432608837629840),
    Elem::new(37022801367124200084104469615560914433),
    Elem::new(317995363281387557804319316265333522279),
    Elem::new(65212356154145455566939913864291691380),
    Elem::new(114691326552306785056627924680445704724),
    Elem::new(191337756613793289172887461630309921006),
    Elem::new(88004990029374170396718749007641869717),
    Elem::new(326138670852305840334678546615580089969),
    Elem::new(271480491365462262541047594537666004010),
    Elem::new(219627364852196098654764946574258486102),
    Elem::new(233896236396084120802563751766537905221),
    Elem::new(118134270279607534612442961185326279297),
    Elem::new(283453207805304338212072415238889822731),
    Elem::new(118235992913371147329895186106061565774),
    Elem::new(286697263734248175984000373401131137170),
    Elem::new(323200633015677677275905168265824590852),
    Elem::new(270053995341857523358084297722289778071),
    Elem::new(168254528849346269673536240973072362171),
    Elem::new(137168394972367619622732475799400341810),
    Elem::new(181396420613869665795859231670167012791),
    Elem::new(125480190248470542733915754619417623908),
    Elem::new(209540598410783219143106900915581086038),
    Elem::new(178015299389255883855450166509097465823),
    Elem::new(108736975777524029774284482822913826551),
    Elem::new(236460719442438694231436904919188186413),
    Elem::new(58473661628550723390576068213295477751),
    Elem::new(257433767838565916336653738415416514484),
    Elem::new(71930874131050356271165403674128573332),
    Elem::new(116409631965320410106403759694457172751),
    Elem::new(321294830390145659590996497887302633473),
    Elem::new(48073895084039246638758426384352165275),
    Elem::new(55643885269061530488734525445099673569),
    Elem::new(218904762300971489500615110839221901210),
    Elem::new(291644728922626812350703865769572378388),
    Elem::new(127088497571914851114698133221781372914),
    Elem::new(2042766001497629287763795750477059730),
    Elem::new(219962855902011599948930380336935037298),
    Elem::new(299710250149476252588079372202509572601),
    Elem::new(193277381713935058022930042387316542548),
    Elem::new(231544127198435424976457948012344126871),
    Elem::new(60414249221498063949014112904592751920),
    Elem::new(302534992199339033015928282560319502807),
    Elem::new(287958778872602760982876452470171615575),
    Elem::new(134926437774889708740045054339327623908),
    Elem::new(277344739895293442598275471922252409459),
    Elem::new(46140626862756811178697328217023129604),
    Elem::new(159086202487453376964532191170574795444),
    Elem::new(282866053414122647673098008729264237341),
    Elem::new(9505658592294727050118478368790279302),
    Elem::new(81285690940412475278287003216055903990),
    Elem::new(20431536313829434124352340324414703575),
    Elem::new(332902448231951464058387130954171315155),
    Elem::new(252783574092979305808931803255011551975),
    Elem::new(238673557328966250754937103812214330248),
    Elem::new(118796454793018655046171316985950485994),
    Elem::new(144806800324129634820388140281133723524),
    Elem::new(39061450683982795482411733080045240978),
    Elem::new(6766709017455661249877844369625589699),
    Elem::new(47243398502023709561203841818983889525),
    Elem::new(1211597052303231193366953643850706555),
    Elem::new(216496156370583631179243160868765267650),
    Elem::new(274361455172322532664248290600283112978),
    Elem::new(267899031289768887245208642957993979116),
    Elem::new(244088540335495286891751959418496719297),
    Elem::new(307730252804032331155034396119361085086),
    Elem::new(5845478755806413322737307560365976161),
    Elem::new(168544962438612944841331177514181484018),
    Elem::new(108114550579344139140073746534211426815),
    Elem::new(61355972534812124874044553516558648510),
    Elem::new(292761851292906878914859288975500552122),
    Elem::new(41946796760335350153057001864041828129),
    Elem::new(219595744768650904522415500717872937996),
    Elem::new(79163316353927915665753448613005063427),
    Elem::new(144000348026294000025757522806933549036),
    Elem::new(208193561387214219856091326796853687346),
    Elem::new(103560216192929029666967779239122703533),
    Elem::new(235887698224035659840065062683678263012),
    Elem::new(49295426143183858666956417915184783181),
    Elem::new(170622597836750128882820411551888612378),
    Elem::new(35978614060576926512893658345082837780),
    Elem::new(261412476991616383155241488270042436171),
    Elem::new(155179388630344909841612436936947578169),
    Elem::new(113822527646172163099663271501464039937),
    Elem::new(64034108438888126189070374275272955554),
    Elem::new(99983747311974585075113704172665920346),
    Elem::new(236571709759562543627976319115372069084),
    Elem::new(35609137364752911607352034261087274173),
    Elem::new(228028974548576657500203199277385411785),
    Elem::new(216040467167869205703694915283267973916),
    Elem::new(175067555634716185072466973757966184336),
    Elem::new(31170133915143757482014992805434957728),
    Elem::new(225750543609266217407860197641642327152),
    Elem::new(37431248622767659835894854121017678028),
    Elem::new(230724458517123175241757004227676051370),
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
