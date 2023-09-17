#[macro_use]
extern crate lazy_static;
use clap::Parser;
use std::collections::HashMap;
use log::{info, trace};
use env_logger;

use std::time::Instant;

mod utils;
mod stark;
mod rescue;
mod griffin;

use winterfell::math::{FieldElement};
use winterfell::{FieldExtension, Prover, Trace};

#[cfg(feature = "master_thesis_full")]
const COMPILE_VARIANT: &str = "master_thesis_full";
#[cfg(feature = "master_thesis_half")]
const COMPILE_VARIANT: &str = "master_thesis_half";
#[cfg(feature = "master_thesis_quarter")]
const COMPILE_VARIANT: &str = "master_thesis_quarter";
#[cfg(feature = "master_thesis_test")]
const COMPILE_VARIANT: &str = "master_thesis_test";


/// STARKs Benchmark
/// https://github.com/romanmarkusholler/MasterThesis
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// STARK variant to benchmark
    #[clap(short, long, validator=stark_choice_validator)]
    pub stark: String,

    /// Length of the input pixel vector
    #[clap(short, long, default_value_t = 110016)]
    pub length: usize,

    /// Input random number generation: lower bound (inclusive)
    #[clap(short, long, default_value_t = 0)]
    pub begin: u16,

    /// Input random number generation: upper bound (exclusive)
    #[clap(short, long, default_value_t = 20000)]
    pub end: u16,
}

pub fn stark_choice_validator(v: &str) -> Result<(), String> {
    if STARK_OPTIONS.contains_key(v) { return Ok(()); }
    let mut keys = vec![];
    for elem in STARK_OPTIONS.keys() {
        keys.push(String::from(*elem));
    }
    keys.sort();
    let the_options = keys.join("\n    ");
    Err(String::from("The value must be one of:\n\n    ") + &the_options)
}

lazy_static! {
    static ref STARK_OPTIONS: HashMap<&'static str, fn(&Args)> = {
        let mut m = HashMap::new();
        m.insert("stark_a", stark_a as fn(&Args));
        m.insert("stark_a_62", stark_a_62 as fn(&Args));
        m.insert("stark_a_griffin", stark_a_griffin as fn(&Args));
        m.insert("stark_a_griffin_62", stark_a_griffin_62 as fn(&Args));
        m.insert("stark_b", stark_b as fn(&Args));
        m.insert("stark_b_62", stark_b_62 as fn(&Args));
        m.insert("stark_b_griffin", stark_b_griffin as fn(&Args));
        m.insert("stark_b_griffin_62", stark_b_griffin_62 as fn(&Args));
        m.insert("stark_c", stark_c as fn(&Args));
        m.insert("stark_c_griffin", stark_c_griffin as fn(&Args));
        m.insert("stark_d", stark_d as fn(&Args));
        m.insert("stark_d_griffin", stark_d_griffin as fn(&Args));
        m.insert("stark_e", stark_e as fn(&Args));
        m.insert("stark_e_62", stark_e_62 as fn(&Args));
        m.insert("stark_e_griffin", stark_e_griffin as fn(&Args));
        m.insert("stark_e_griffin_62", stark_e_griffin_62 as fn(&Args));
        m.insert("stark_e_opt", stark_e_opt as fn(&Args));
        m.insert("stark_e_opt_62", stark_e_opt_62 as fn(&Args));
        m.insert("stark_e_opt_griffin", stark_e_opt_griffin as fn(&Args));
        m.insert("stark_e_opt_griffin_62", stark_e_opt_griffin_62 as fn(&Args));
        m.insert("stark_f", stark_f as fn(&Args));
        m.insert("stark_f_62", stark_f_62 as fn(&Args));
        m.insert("stark_f_64", stark_f_64 as fn(&Args));
        m.insert("stark_f_griffin", stark_f_griffin as fn(&Args));
        m.insert("stark_f_griffin_62", stark_f_griffin_62 as fn(&Args));
        m.insert("stark_f_opt_m2", stark_f_opt_m2 as fn(&Args));
        m.insert("stark_f_opt_m2_62", stark_f_opt_m2_62 as fn(&Args));
        m.insert("stark_f_opt_m2_griffin", stark_f_opt_m2_griffin as fn(&Args));
        m.insert("stark_f_opt_m2_griffin_62", stark_f_opt_m2_griffin_62 as fn(&Args));
        m.insert("stark_f_opt_m4", stark_f_opt_m4 as fn(&Args));
        m.insert("stark_f_opt_m4_62", stark_f_opt_m4_62 as fn(&Args));
        m.insert("stark_f_opt_m4_griffin", stark_f_opt_m4_griffin as fn(&Args));
        m.insert("stark_f_opt_m4_griffin_62", stark_f_opt_m4_griffin_62 as fn(&Args));
        m.insert("stark_f_opt_m8", stark_f_opt_m8 as fn(&Args));
        m.insert("stark_f_opt_m8_62", stark_f_opt_m8_62 as fn(&Args));
        m.insert("stark_f_opt_m8_griffin", stark_f_opt_m8_griffin as fn(&Args));
        m.insert("stark_f_opt_m8_griffin_62", stark_f_opt_m8_griffin_62 as fn(&Args));
        m.insert("stark_g", stark_g as fn(&Args));
        m.insert("stark_g_62", stark_g_62 as fn(&Args));
        m.insert("stark_g_griffin", stark_g_griffin as fn(&Args));
        m.insert("stark_g_griffin_62", stark_g_griffin_62 as fn(&Args));
        m
    };
}

fn main() {
    env_logger::init();
    let args: Args = Args::parse();
    STARK_OPTIONS[&*args.stark](&args);
}

pub fn stark_a(args: &Args) {
    let name = "STARK A";
    trace!("BEGIN scenario {}", name);
    use stark::stark_a as Stark;
    use rescue::p128_m4_c3_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128) };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_a_62(args: &Args) {
    let name = "STARK A 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_a_62 as Stark;
    use rescue::p62_m4_c3_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u64) };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_a_griffin(args: &Args) {
    let name = "STARK A (Griffin)";
    trace!("BEGIN scenario {}", name);
    use stark::stark_a_griffin as Stark;
    use griffin::p128_t4_c3_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128) };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_a_griffin_62(args: &Args) {
    let name = "STARK A (Griffin) 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_a_griffin_62 as Stark;
    use griffin::p62_t4_c3_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u64) };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_b(args: &Args) {
    let name = "STARK B";
    trace!("BEGIN scenario {}", name);
    use stark::stark_b as Stark;
    use rescue::p128_m4_c2_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128) };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_b_62(args: &Args) {
    let name = "STARK B 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_b_62 as Stark;
    use rescue::p62_m4_c2_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u64) };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_b_griffin(args: &Args) {
    let name = "STARK B (Griffin)";
    trace!("BEGIN scenario {}", name);
    use stark::stark_b_griffin as Stark;
    use griffin::p128_t4_c2_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128) };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_b_griffin_62(args: &Args) {
    let name = "STARK B (Griffin) 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_b_griffin_62 as Stark;
    use griffin::p62_t4_c2_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u64) };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_c(args: &Args) {
    let name = "STARK C";
    trace!("BEGIN scenario {}", name);
    use stark::stark_c as Stark;
    use rescue::p128_m4_c2_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, _) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let mut pixels_base = vec![];
    for i in 0..(args.length / 8) {
        let mut result: u128 = 0;
        for j in 0..8 {
            result |= (pixels_u16[i * 8 + j] as u128) << (16u128 * j as u128);
        }
        pixels_base.push(BaseElement::new(result));
    }
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128) };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_c_griffin(args: &Args) {
    let name = "STARK C (Griffin)";
    trace!("BEGIN scenario {}", name);
    use stark::stark_c_griffin as Stark;
    use griffin::p128_t4_c2_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, _) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let mut pixels_base = vec![];
    for i in 0..(args.length / 8) {
        let mut result: u128 = 0;
        for j in 0..8 {
            result |= (pixels_u16[i * 8 + j] as u128) << (16u128 * j as u128);
        }
        pixels_base.push(BaseElement::new(result));
    }
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128) };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_d(args: &Args) {
    let name = "STARK D";
    trace!("BEGIN scenario {}", name);
    use stark::stark_d as Stark;
    use rescue::p128_m4_c2_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, _) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let mut pixels_base = vec![];
    for i in 0..(args.length / 8) {
        let mut result: u128 = 0;
        for j in 0..8 {
            result |= (pixels_u16[i * 8 + j] as u128) << (16u128 * j as u128);
        }
        pixels_base.push(BaseElement::new(result));
    }
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16, &hash);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128) };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_d_griffin(args: &Args) {
    let name = "STARK D (Griffin)";
    trace!("BEGIN scenario {}", name);
    use stark::stark_d_griffin as Stark;
    use griffin::p128_t4_c2_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, _) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let mut pixels_base = vec![];
    for i in 0..(args.length / 8) {
        let mut result: u128 = 0;
        for j in 0..8 {
            result |= (pixels_u16[i * 8 + j] as u128) << (16u128 * j as u128);
        }
        pixels_base.push(BaseElement::new(result));
    }
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16, &hash);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128) };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_e(args: &Args) {
    let name = "STARK E";
    trace!("BEGIN scenario {}", name);
    use stark::stark_e as Stark;
    use rescue::p128_m9_c1_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
    let stat_result_step = hash_result_step;
    let hash_s_result_step = 4 * args.length + Stark::SIZE_OF_T - 1;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_pixels_trace = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_l = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_h = [BaseElement::ZERO; Hash::RATE];
    let mut hash_med = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_m = [BaseElement::ZERO; Hash::RATE];
    let mut hash_s = [BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_pixels_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        hash_omega_l[i] = trace.get(Stark::T_OMEGA_L_HASH.idx + i, hash_result_step);
        hash_omega_h[i] = trace.get(Stark::T_OMEGA_H_HASH.idx + i, hash_result_step);
        hash_med[i] = trace.get(Stark::T_MED_HASH.idx + i, hash_result_step);
        hash_omega_m[i] = trace.get(Stark::T_OMEGA_M_HASH.idx + i, hash_result_step);
        hash_s[i] = trace.get(Stark::T_S_HASH.idx + i, hash_s_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_pixels_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    let manual_stats = utils::get_plain_statistics_u128::<BaseElement>(pixels_u16);
    #[allow(unused_assignments)]
    let mut med_high_trace = BaseElement::ZERO;
    #[allow(unused_assignments)]
    let mut med_low_trace = BaseElement::ZERO;
    med_low_trace = trace.get(Stark::T_MED.begin() + (stat_result_step / 2 - 1) % Stark::CYCLE_LENGTH, stat_result_step / 2);
    med_high_trace = trace.get(Stark::T_MED.begin() + (stat_result_step / 2) % Stark::CYCLE_LENGTH, stat_result_step / 2 + 1);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    let min_trace = trace.get(Stark::T_MIN.begin(), stat_result_step);
    let max_trace = trace.get(Stark::T_MAX.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    assert_eq!(min_trace, manual_stats.min_e);
    assert_eq!(max_trace, manual_stats.max_e);
    assert_eq!(med_low_trace, manual_stats.med_low_e);
    assert_eq!(med_high_trace, manual_stats.med_high_e);
    trace!("{}", utils::get_stats_string_u128(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(16, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs {
        hash_pixels: hash,
        hash_omega_l,
        hash_omega_h,
        hash_med,
        hash_omega_m,
        hash_s,
        input_length: BaseElement::new(args.length as u128),
        sum: manual_stats.sum_e,
        avg_rounded: manual_stats.avg_rounded_e,
        variance: manual_stats.var_e,
        min: min_trace,
        max: max_trace,
        med_low: med_low_trace,
        med_high: med_high_trace,
    };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_e_62(args: &Args) {
    let name = "STARK E 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_e_62 as Stark;
    use rescue::p62_m9_c1_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
    let stat_result_step = hash_result_step;
    let hash_s_result_step = 4 * args.length + Stark::SIZE_OF_T - 1;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_pixels_trace = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_l = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_h = [BaseElement::ZERO; Hash::RATE];
    let mut hash_med = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_m = [BaseElement::ZERO; Hash::RATE];
    let mut hash_s = [BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_pixels_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        hash_omega_l[i] = trace.get(Stark::T_OMEGA_L_HASH.idx + i, hash_result_step);
        hash_omega_h[i] = trace.get(Stark::T_OMEGA_H_HASH.idx + i, hash_result_step);
        hash_med[i] = trace.get(Stark::T_MED_HASH.idx + i, hash_result_step);
        hash_omega_m[i] = trace.get(Stark::T_OMEGA_M_HASH.idx + i, hash_result_step);
        hash_s[i] = trace.get(Stark::T_S_HASH.idx + i, hash_s_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_pixels_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    let manual_stats = utils::get_plain_statistics_u64::<BaseElement>(pixels_u16);
    #[allow(unused_assignments)]
    let mut med_high_trace = BaseElement::ZERO;
    #[allow(unused_assignments)]
    let mut med_low_trace = BaseElement::ZERO;
    med_low_trace = trace.get(Stark::T_MED.begin() + (stat_result_step / 2 - 1) % Stark::CYCLE_LENGTH, stat_result_step / 2);
    med_high_trace = trace.get(Stark::T_MED.begin() + (stat_result_step / 2) % Stark::CYCLE_LENGTH, stat_result_step / 2 + 1);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    let min_trace = trace.get(Stark::T_MIN.begin(), stat_result_step);
    let max_trace = trace.get(Stark::T_MAX.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    assert_eq!(min_trace, manual_stats.min_e);
    assert_eq!(max_trace, manual_stats.max_e);
    assert_eq!(med_low_trace, manual_stats.med_low_e);
    assert_eq!(med_high_trace, manual_stats.med_high_e);
    trace!("{}", utils::get_stats_string_u64(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(16, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs {
        hash_pixels: hash,
        hash_omega_l,
        hash_omega_h,
        hash_med,
        hash_omega_m,
        hash_s,
        input_length: BaseElement::new(args.length as u64),
        sum: manual_stats.sum_e,
        avg_rounded: manual_stats.avg_rounded_e,
        variance: manual_stats.var_e,
        min: min_trace,
        max: max_trace,
        med_low: med_low_trace,
        med_high: med_high_trace,
    };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_e_griffin(args: &Args) {
    let name = "STARK E (Griffin)";
    trace!("BEGIN scenario {}", name);
    use stark::stark_e_griffin as Stark;
    use griffin::p128_t12_c4_s100 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
    let stat_result_step = hash_result_step;
    let hash_s_result_step = 4 * args.length + Stark::SIZE_OF_T - 1;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_pixels_trace = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_l = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_h = [BaseElement::ZERO; Hash::RATE];
    let mut hash_med = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_m = [BaseElement::ZERO; Hash::RATE];
    let mut hash_s = [BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_pixels_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        hash_omega_l[i] = trace.get(Stark::T_OMEGA_L_HASH.idx + i, hash_result_step);
        hash_omega_h[i] = trace.get(Stark::T_OMEGA_H_HASH.idx + i, hash_result_step);
        hash_med[i] = trace.get(Stark::T_MED_HASH.idx + i, hash_result_step);
        hash_omega_m[i] = trace.get(Stark::T_OMEGA_M_HASH.idx + i, hash_result_step);
        hash_s[i] = trace.get(Stark::T_S_HASH.idx + i, hash_s_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_pixels_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    let manual_stats = utils::get_plain_statistics_u128::<BaseElement>(pixels_u16);
    #[allow(unused_assignments)]
    let mut med_high_trace = BaseElement::ZERO;
    #[allow(unused_assignments)]
    let mut med_low_trace = BaseElement::ZERO;
    med_low_trace = trace.get(Stark::T_MED.begin() + (stat_result_step / 2 - 1) % Stark::CYCLE_LENGTH, stat_result_step / 2);
    med_high_trace = trace.get(Stark::T_MED.begin() + (stat_result_step / 2) % Stark::CYCLE_LENGTH, stat_result_step / 2 + 1);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    let min_trace = trace.get(Stark::T_MIN.begin(), stat_result_step);
    let max_trace = trace.get(Stark::T_MAX.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    assert_eq!(min_trace, manual_stats.min_e);
    assert_eq!(max_trace, manual_stats.max_e);
    assert_eq!(med_low_trace, manual_stats.med_low_e);
    assert_eq!(med_high_trace, manual_stats.med_high_e);
    trace!("{}", utils::get_stats_string_u128(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(16, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs {
        hash_pixels: hash,
        hash_omega_l,
        hash_omega_h,
        hash_med,
        hash_omega_m,
        hash_s,
        input_length: BaseElement::new(args.length as u128),
        sum: manual_stats.sum_e,
        avg_rounded: manual_stats.avg_rounded_e,
        variance: manual_stats.var_e,
        min: min_trace,
        max: max_trace,
        med_low: med_low_trace,
        med_high: med_high_trace,
    };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_e_griffin_62(args: &Args) {
    let name = "STARK E (Griffin) 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_e_griffin_62 as Stark;
    use griffin::p62_t12_c4_s100 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
    let stat_result_step = hash_result_step;
    let hash_s_result_step = 4 * args.length + Stark::SIZE_OF_T - 1;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_pixels_trace = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_l = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_h = [BaseElement::ZERO; Hash::RATE];
    let mut hash_med = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_m = [BaseElement::ZERO; Hash::RATE];
    let mut hash_s = [BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_pixels_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        hash_omega_l[i] = trace.get(Stark::T_OMEGA_L_HASH.idx + i, hash_result_step);
        hash_omega_h[i] = trace.get(Stark::T_OMEGA_H_HASH.idx + i, hash_result_step);
        hash_med[i] = trace.get(Stark::T_MED_HASH.idx + i, hash_result_step);
        hash_omega_m[i] = trace.get(Stark::T_OMEGA_M_HASH.idx + i, hash_result_step);
        hash_s[i] = trace.get(Stark::T_S_HASH.idx + i, hash_s_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_pixels_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    let manual_stats = utils::get_plain_statistics_u64::<BaseElement>(pixels_u16);
    #[allow(unused_assignments)]
    let mut med_high_trace = BaseElement::ZERO;
    #[allow(unused_assignments)]
    let mut med_low_trace = BaseElement::ZERO;
    med_low_trace = trace.get(Stark::T_MED.begin() + (stat_result_step / 2 - 1) % Stark::CYCLE_LENGTH, stat_result_step / 2);
    med_high_trace = trace.get(Stark::T_MED.begin() + (stat_result_step / 2) % Stark::CYCLE_LENGTH, stat_result_step / 2 + 1);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    let min_trace = trace.get(Stark::T_MIN.begin(), stat_result_step);
    let max_trace = trace.get(Stark::T_MAX.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    assert_eq!(min_trace, manual_stats.min_e);
    assert_eq!(max_trace, manual_stats.max_e);
    assert_eq!(med_low_trace, manual_stats.med_low_e);
    assert_eq!(med_high_trace, manual_stats.med_high_e);
    trace!("{}", utils::get_stats_string_u64(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(16, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs {
        hash_pixels: hash,
        hash_omega_l,
        hash_omega_h,
        hash_med,
        hash_omega_m,
        hash_s,
        input_length: BaseElement::new(args.length as u64),
        sum: manual_stats.sum_e,
        avg_rounded: manual_stats.avg_rounded_e,
        variance: manual_stats.var_e,
        min: min_trace,
        max: max_trace,
        med_low: med_low_trace,
        med_high: med_high_trace,
    };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_e_opt(args: &Args) {
    let name = "STARK E (opt)";
    trace!("BEGIN scenario {}", name);
    use stark::stark_e_opt as Stark;
    use rescue::p128_m9_c1_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
    let stat_result_step = hash_result_step;
    let hash_s_result_step = (4 * args.length + Stark::SIZE_OF_T - 1) / 5;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_pixels_trace = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_l = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_h = [BaseElement::ZERO; Hash::RATE];
    let mut hash_med = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_m = [BaseElement::ZERO; Hash::RATE];
    let mut hash_s = [BaseElement::ZERO; 5 * Hash::RATE];
    for i in 0..Hash::RATE {
        hash_pixels_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        hash_omega_l[i] = trace.get(Stark::T_OMEGA_L_HASH.idx + i, hash_result_step);
        hash_omega_h[i] = trace.get(Stark::T_OMEGA_H_HASH.idx + i, hash_result_step);
        hash_med[i] = trace.get(Stark::T_MED_HASH.idx + i, hash_result_step);
        hash_omega_m[i] = trace.get(Stark::T_OMEGA_M_HASH.idx + i, hash_result_step);
        hash_s[Hash::RATE * 0 + i] = trace.get(Stark::T_S_HASH_1.idx + i, hash_s_result_step);
        hash_s[Hash::RATE * 1 + i] = trace.get(Stark::T_S_HASH_2.idx + i, hash_s_result_step);
        hash_s[Hash::RATE * 2 + i] = trace.get(Stark::T_S_HASH_3.idx + i, hash_s_result_step);
        hash_s[Hash::RATE * 3 + i] = trace.get(Stark::T_S_HASH_4.idx + i, hash_s_result_step);
        hash_s[Hash::RATE * 4 + i] = trace.get(Stark::T_S_HASH_5.idx + i, hash_s_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_pixels_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    let manual_stats = utils::get_plain_statistics_u128::<BaseElement>(pixels_u16);
    #[allow(unused_assignments)]
    let mut med_high_trace = BaseElement::ZERO;
    #[allow(unused_assignments)]
    let mut med_low_trace = BaseElement::ZERO;
    med_low_trace = trace.get(Stark::T_MED.begin() + (stat_result_step / 2 - 1) % Stark::CYCLE_LENGTH, stat_result_step / 2);
    med_high_trace = trace.get(Stark::T_MED.begin() + (stat_result_step / 2) % Stark::CYCLE_LENGTH, stat_result_step / 2 + 1);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    let min_trace = trace.get(Stark::T_MIN.begin(), stat_result_step);
    let max_trace = trace.get(Stark::T_MAX.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    assert_eq!(min_trace, manual_stats.min_e);
    assert_eq!(max_trace, manual_stats.max_e);
    assert_eq!(med_low_trace, manual_stats.med_low_e);
    assert_eq!(med_high_trace, manual_stats.med_high_e);
    trace!("{}", utils::get_stats_string_u128(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs {
        hash_pixels: hash,
        hash_omega_l,
        hash_omega_h,
        hash_med,
        hash_omega_m,
        hash_s,
        input_length: BaseElement::new(args.length as u128),
        sum: manual_stats.sum_e,
        avg_rounded: manual_stats.avg_rounded_e,
        variance: manual_stats.var_e,
        min: min_trace,
        max: max_trace,
        med_low: med_low_trace,
        med_high: med_high_trace,
    };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_e_opt_62(args: &Args) {
    let name = "STARK E (opt) 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_e_opt_62 as Stark;
    use rescue::p62_m9_c1_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
    let stat_result_step = hash_result_step;
    let hash_s_result_step = (4 * args.length + Stark::SIZE_OF_T - 1) / 5;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_pixels_trace = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_l = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_h = [BaseElement::ZERO; Hash::RATE];
    let mut hash_med = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_m = [BaseElement::ZERO; Hash::RATE];
    let mut hash_s = [BaseElement::ZERO; 5 * Hash::RATE];
    for i in 0..Hash::RATE {
        hash_pixels_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        hash_omega_l[i] = trace.get(Stark::T_OMEGA_L_HASH.idx + i, hash_result_step);
        hash_omega_h[i] = trace.get(Stark::T_OMEGA_H_HASH.idx + i, hash_result_step);
        hash_med[i] = trace.get(Stark::T_MED_HASH.idx + i, hash_result_step);
        hash_omega_m[i] = trace.get(Stark::T_OMEGA_M_HASH.idx + i, hash_result_step);
        hash_s[Hash::RATE * 0 + i] = trace.get(Stark::T_S_HASH_1.idx + i, hash_s_result_step);
        hash_s[Hash::RATE * 1 + i] = trace.get(Stark::T_S_HASH_2.idx + i, hash_s_result_step);
        hash_s[Hash::RATE * 2 + i] = trace.get(Stark::T_S_HASH_3.idx + i, hash_s_result_step);
        hash_s[Hash::RATE * 3 + i] = trace.get(Stark::T_S_HASH_4.idx + i, hash_s_result_step);
        hash_s[Hash::RATE * 4 + i] = trace.get(Stark::T_S_HASH_5.idx + i, hash_s_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_pixels_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    let manual_stats = utils::get_plain_statistics_u64::<BaseElement>(pixels_u16);
    #[allow(unused_assignments)]
    let mut med_high_trace = BaseElement::ZERO;
    #[allow(unused_assignments)]
    let mut med_low_trace = BaseElement::ZERO;
    med_low_trace = trace.get(Stark::T_MED.begin() + (stat_result_step / 2 - 1) % Stark::CYCLE_LENGTH, stat_result_step / 2);
    med_high_trace = trace.get(Stark::T_MED.begin() + (stat_result_step / 2) % Stark::CYCLE_LENGTH, stat_result_step / 2 + 1);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    let min_trace = trace.get(Stark::T_MIN.begin(), stat_result_step);
    let max_trace = trace.get(Stark::T_MAX.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    assert_eq!(min_trace, manual_stats.min_e);
    assert_eq!(max_trace, manual_stats.max_e);
    assert_eq!(med_low_trace, manual_stats.med_low_e);
    assert_eq!(med_high_trace, manual_stats.med_high_e);
    trace!("{}", utils::get_stats_string_u64(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs {
        hash_pixels: hash,
        hash_omega_l,
        hash_omega_h,
        hash_med,
        hash_omega_m,
        hash_s,
        input_length: BaseElement::new(args.length as u64),
        sum: manual_stats.sum_e,
        avg_rounded: manual_stats.avg_rounded_e,
        variance: manual_stats.var_e,
        min: min_trace,
        max: max_trace,
        med_low: med_low_trace,
        med_high: med_high_trace,
    };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_e_opt_griffin(args: &Args) {
    let name = "STARK E (opt) (Griffin)";
    trace!("BEGIN scenario {}", name);
    use stark::stark_e_opt_griffin as Stark;
    use griffin::p128_t12_c4_s100 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
    let stat_result_step = hash_result_step;
    let hash_s_result_step = (4 * args.length + Stark::SIZE_OF_T - 1) / 5;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_pixels_trace = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_l = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_h = [BaseElement::ZERO; Hash::RATE];
    let mut hash_med = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_m = [BaseElement::ZERO; Hash::RATE];
        let mut hash_s = [BaseElement::ZERO; 5 * Hash::RATE];
    for i in 0..Hash::RATE {
        hash_pixels_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        hash_omega_l[i] = trace.get(Stark::T_OMEGA_L_HASH.idx + i, hash_result_step);
        hash_omega_h[i] = trace.get(Stark::T_OMEGA_H_HASH.idx + i, hash_result_step);
        hash_med[i] = trace.get(Stark::T_MED_HASH.idx + i, hash_result_step);
        hash_omega_m[i] = trace.get(Stark::T_OMEGA_M_HASH.idx + i, hash_result_step);
        hash_s[Hash::RATE * 0 + i] = trace.get(Stark::T_S_HASH_1.idx + i, hash_s_result_step);
        hash_s[Hash::RATE * 1 + i] = trace.get(Stark::T_S_HASH_2.idx + i, hash_s_result_step);
        hash_s[Hash::RATE * 2 + i] = trace.get(Stark::T_S_HASH_3.idx + i, hash_s_result_step);
        hash_s[Hash::RATE * 3 + i] = trace.get(Stark::T_S_HASH_4.idx + i, hash_s_result_step);
        hash_s[Hash::RATE * 4 + i] = trace.get(Stark::T_S_HASH_5.idx + i, hash_s_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_pixels_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    let manual_stats = utils::get_plain_statistics_u128::<BaseElement>(pixels_u16);
    #[allow(unused_assignments)]
    let mut med_high_trace = BaseElement::ZERO;
    #[allow(unused_assignments)]
    let mut med_low_trace = BaseElement::ZERO;
    med_low_trace = trace.get(Stark::T_MED.begin() + (stat_result_step / 2 - 1) % Stark::CYCLE_LENGTH, stat_result_step / 2);
    med_high_trace = trace.get(Stark::T_MED.begin() + (stat_result_step / 2) % Stark::CYCLE_LENGTH, stat_result_step / 2 + 1);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    let min_trace = trace.get(Stark::T_MIN.begin(), stat_result_step);
    let max_trace = trace.get(Stark::T_MAX.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    assert_eq!(min_trace, manual_stats.min_e);
    assert_eq!(max_trace, manual_stats.max_e);
    assert_eq!(med_low_trace, manual_stats.med_low_e);
    assert_eq!(med_high_trace, manual_stats.med_high_e);
    trace!("{}", utils::get_stats_string_u128(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs {
        hash_pixels: hash,
        hash_omega_l,
        hash_omega_h,
        hash_med,
        hash_omega_m,
        hash_s,
        input_length: BaseElement::new(args.length as u128),
        sum: manual_stats.sum_e,
        avg_rounded: manual_stats.avg_rounded_e,
        variance: manual_stats.var_e,
        min: min_trace,
        max: max_trace,
        med_low: med_low_trace,
        med_high: med_high_trace,
    };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_e_opt_griffin_62(args: &Args) {
    let name = "STARK E (opt) (Griffin) 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_e_opt_griffin_62 as Stark;
    use griffin::p62_t12_c4_s100 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
    let stat_result_step = hash_result_step;
    let hash_s_result_step = (4 * args.length + Stark::SIZE_OF_T - 1) / 5;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_pixels_trace = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_l = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_h = [BaseElement::ZERO; Hash::RATE];
    let mut hash_med = [BaseElement::ZERO; Hash::RATE];
    let mut hash_omega_m = [BaseElement::ZERO; Hash::RATE];
        let mut hash_s = [BaseElement::ZERO; 5 * Hash::RATE];
    for i in 0..Hash::RATE {
        hash_pixels_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        hash_omega_l[i] = trace.get(Stark::T_OMEGA_L_HASH.idx + i, hash_result_step);
        hash_omega_h[i] = trace.get(Stark::T_OMEGA_H_HASH.idx + i, hash_result_step);
        hash_med[i] = trace.get(Stark::T_MED_HASH.idx + i, hash_result_step);
        hash_omega_m[i] = trace.get(Stark::T_OMEGA_M_HASH.idx + i, hash_result_step);
        hash_s[Hash::RATE * 0 + i] = trace.get(Stark::T_S_HASH_1.idx + i, hash_s_result_step);
        hash_s[Hash::RATE * 1 + i] = trace.get(Stark::T_S_HASH_2.idx + i, hash_s_result_step);
        hash_s[Hash::RATE * 2 + i] = trace.get(Stark::T_S_HASH_3.idx + i, hash_s_result_step);
        hash_s[Hash::RATE * 3 + i] = trace.get(Stark::T_S_HASH_4.idx + i, hash_s_result_step);
        hash_s[Hash::RATE * 4 + i] = trace.get(Stark::T_S_HASH_5.idx + i, hash_s_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_pixels_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    let manual_stats = utils::get_plain_statistics_u64::<BaseElement>(pixels_u16);
    #[allow(unused_assignments)]
    let mut med_high_trace = BaseElement::ZERO;
    #[allow(unused_assignments)]
    let mut med_low_trace = BaseElement::ZERO;
    med_low_trace = trace.get(Stark::T_MED.begin() + (stat_result_step / 2 - 1) % Stark::CYCLE_LENGTH, stat_result_step / 2);
    med_high_trace = trace.get(Stark::T_MED.begin() + (stat_result_step / 2) % Stark::CYCLE_LENGTH, stat_result_step / 2 + 1);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    let min_trace = trace.get(Stark::T_MIN.begin(), stat_result_step);
    let max_trace = trace.get(Stark::T_MAX.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    assert_eq!(min_trace, manual_stats.min_e);
    assert_eq!(max_trace, manual_stats.max_e);
    assert_eq!(med_low_trace, manual_stats.med_low_e);
    assert_eq!(med_high_trace, manual_stats.med_high_e);
    trace!("{}", utils::get_stats_string_u64(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs {
        hash_pixels: hash,
        hash_omega_l,
        hash_omega_h,
        hash_med,
        hash_omega_m,
        hash_s,
        input_length: BaseElement::new(args.length as u64),
        sum: manual_stats.sum_e,
        avg_rounded: manual_stats.avg_rounded_e,
        variance: manual_stats.var_e,
        min: min_trace,
        max: max_trace,
        med_low: med_low_trace,
        med_high: med_high_trace,
    };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f(args: &Args) {
    let name = "STARK F";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f as Stark;
    use rescue::p128_m9_c1_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u128::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u128(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f_62(args: &Args) {
    let name = "STARK F 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f_62 as Stark;
    use rescue::p62_m9_c1_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u64::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u64(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f_64(args: &Args) {
    let name = "STARK F 64";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f_64 as Stark;
    use rescue::p64_m9_c1_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u64::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u64(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f_griffin(args: &Args) {
    let name = "STARK F (Griffin)";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f_griffin as Stark;
    use griffin::p128_t12_c4_s100 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u128::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u128(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f_griffin_62(args: &Args) {
    let name = "STARK F 62 (Griffin)";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f_griffin_62 as Stark;
    use griffin::p62_t12_c4_s100 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u64::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u64(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f_opt_m2(args: &Args) {
    let name = "STARK F (opt m2)";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f_opt_m2 as Stark;
    use rescue::p128_m17_c1_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u128::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u128(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f_opt_m2_62(args: &Args) {
    let name = "STARK F (opt m2) 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f_opt_m2_62 as Stark;
    use rescue::p62_m17_c1_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u64::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u64(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f_opt_m2_griffin(args: &Args) {
    let name = "STARK F (opt m2) (Griffin)";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f_opt_m2_griffin as Stark;
    use griffin::p128_t20_c4_s100 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u128::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u128(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f_opt_m2_griffin_62(args: &Args) {
    let name = "STARK F (opt m2) (Griffin) 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f_opt_m2_griffin_62 as Stark;
    use griffin::p62_t20_c4_s100 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u64::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u64(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f_opt_m4(args: &Args) {
    let name = "STARK F (opt m4)";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f_opt_m4 as Stark;
    use rescue::p128_m33_c1_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u128::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u128(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f_opt_m4_62(args: &Args) {
    let name = "STARK F (opt m4) 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f_opt_m4_62 as Stark;
    use rescue::p62_m33_c1_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u64::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u64(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f_opt_m4_griffin(args: &Args) {
    let name = "STARK F (opt m4) (Griffin)";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f_opt_m4_griffin as Stark;
    use griffin::p128_t36_c4_s100 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u128::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u128(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f_opt_m4_griffin_62(args: &Args) {
    let name = "STARK F (opt m4) (Griffin) 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f_opt_m4_griffin_62 as Stark;
    use griffin::p62_t36_c4_s100 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u64::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u64(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f_opt_m8(args: &Args) {
    let name = "STARK F (opt m8)";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f_opt_m8 as Stark;
    use rescue::p128_m65_c1_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u128::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u128(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f_opt_m8_62(args: &Args) {
    let name = "STARK F (opt m8) 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f_opt_m8_62 as Stark;
    use rescue::p62_m65_c1_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u64::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u64(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f_opt_m8_griffin(args: &Args) {
    let name = "STARK F (opt m8) (Griffin)";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f_opt_m8_griffin as Stark;
    use griffin::p128_t68_c4_s100 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u128::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u128(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_f_opt_m8_griffin_62(args: &Args) {
    let name = "STARK F (opt m8) (Griffin) 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_f_opt_m8_griffin_62 as Stark;
    use griffin::p62_t68_c4_s100 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = args.length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let manual_stats = utils::get_plain_statistics_u64::<BaseElement>(pixels_u16);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u64(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_g(args: &Args) {
    let name = "STARK G";
    trace!("BEGIN scenario {}", name);
    use stark::stark_g as Stark;
    use rescue::p128_m9_c1_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = Stark::CYCLE_LENGTH_ROI * (args.length / Stark::FRAME_SIZE) * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
    let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let mut roi_pixels = vec![];
    let stat_mask = Stark::get_stat_mask_roi();
    for i in 0..pixels_u16.len() {
        if stat_mask[i % Stark::FRAME_SIZE] == BaseElement::ONE {
            roi_pixels.push(pixels_u16[i]);
        }
    }
    let manual_stats = utils::get_plain_statistics_u128::<BaseElement>(roi_pixels);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u128(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_g_62(args: &Args) {
    let name = "STARK G 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_g_62 as Stark;
    use rescue::p62_m9_c1_s128 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = Stark::CYCLE_LENGTH_ROI * (args.length / Stark::FRAME_SIZE) * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
    let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let mut roi_pixels = vec![];
    let stat_mask = Stark::get_stat_mask_roi();
    for i in 0..pixels_u16.len() {
        if stat_mask[i % Stark::FRAME_SIZE] == BaseElement::ONE {
            roi_pixels.push(pixels_u16[i]);
        }
    }
    let manual_stats = utils::get_plain_statistics_u64::<BaseElement>(roi_pixels);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u64(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_g_griffin(args: &Args) {
    let name = "STARK G (Griffin)";
    trace!("BEGIN scenario {}", name);
    use stark::stark_g_griffin as Stark;
    use griffin::p128_t12_c4_s100 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = Stark::CYCLE_LENGTH_ROI * (args.length / Stark::FRAME_SIZE) * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
    let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let mut roi_pixels = vec![];
    let stat_mask = Stark::get_stat_mask_roi();
    for i in 0..pixels_u16.len() {
        if stat_mask[i % Stark::FRAME_SIZE] == BaseElement::ONE {
            roi_pixels.push(pixels_u16[i]);
        }
    }
    let manual_stats = utils::get_plain_statistics_u128::<BaseElement>(roi_pixels);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u128(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::None));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}

pub fn stark_g_griffin_62(args: &Args) {
    let name = "STARK G (Griffin) 62";
    trace!("BEGIN scenario {}", name);
    use stark::stark_g_griffin_62 as Stark;
    use griffin::p62_t12_c4_s100 as Hash;
    type BaseElement = Hash::Elem;
    let hash_result_step = Stark::CYCLE_LENGTH_ROI * (args.length / Stark::FRAME_SIZE) * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
    let stat_result_step = hash_result_step;

    trace!("Starting to generate random input sequence of {} elements ..", args.length);
    let now = Instant::now();
    let (pixels_u16, pixels_base) = utils::get_rand_values::<BaseElement>(args.begin, args.end, args.length);
    let rand_time = now.elapsed().as_millis();
    trace!("Finished generating random input sequence of {} elements in {} ms!", args.length, rand_time);

    //------------------------------------------------------------------------------------------
    // TRACE CONSTRUCTION
    trace!("Starting to build the trace ..");
    let now = Instant::now();
    let trace = Stark::build_trace(&pixels_u16);
    let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
    for i in 0..Hash::RATE {
        hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
    }
    let build_trace_time = now.elapsed().as_millis();
    trace!("Finished building the trace in {} ms!", build_trace_time);

    //------------------------------------------------------------------------------------------
    // STARK COMPUTATION CHECKS
    // AET computes the same hash value as the native hash function
    trace!("Starting to calculate hash manually ..");
    let now = Instant::now();
    let hash = Hash::hash(&pixels_base);
    let manual_hash_time = now.elapsed().as_millis();
    trace!("Finished calculating hash in {} ms! Comparing Hashes ..", manual_hash_time);
    assert_eq!(hash_trace, hash);
    trace!("AET Hash and manual hash are equal!");

    // comparison of statistics
    let mut roi_pixels = vec![];
    let stat_mask = Stark::get_stat_mask_roi();
    for i in 0..pixels_u16.len() {
        if stat_mask[i % Stark::FRAME_SIZE] == BaseElement::ONE {
            roi_pixels.push(pixels_u16[i]);
        }
    }
    let manual_stats = utils::get_plain_statistics_u64::<BaseElement>(roi_pixels);
    let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
    let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
    assert_eq!(sum_trace, manual_stats.sum_e);
    assert_eq!(var_trace, manual_stats.var_e);
    trace!("{}", utils::get_stats_string_u64(&manual_stats));

    //------------------------------------------------------------------------------------------
    // PROVING and VERIFYING
    trace!("Starting to generate the proof ..");
    let now = Instant::now();
    let prover = Stark::TheProver::new(utils::get_proof_options(8, FieldExtension::Quadratic));
    let proof = prover.prove(trace).unwrap();
    let public_inputs = Stark::PubInputs { hash, input_length: BaseElement::new(args.length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
    let prover_time = now.elapsed().as_millis();
    trace!("Finished generating the proof in {} ms! Starting verification ..", prover_time);
    let now = Instant::now();
    assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    let verifier_time = now.elapsed().as_millis();
    trace!("Proof verified successfully in {} ms!", verifier_time);
    trace!("END scenario {}", name);
    info!("{};{};{};{};{};{};{};{};{};{}", COMPILE_VARIANT, name, args.begin, args.end, args.length, rand_time, build_trace_time, manual_hash_time, prover_time, verifier_time);
}