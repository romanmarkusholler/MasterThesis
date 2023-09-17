#[cfg(test)]
mod tests {
    use crate::rescue;
    use crate::griffin;
    use winterfell::math::fields::{f128::BaseElement as BE128, f62::BaseElement as BE62, f64::BaseElement as BE64};
    use winterfell::math::{FieldElement};
    use winterfell::{FieldExtension, Prover, Trace};
    use crate::stark;
    use crate::utils::{get_stats_string_u64, get_stats_string_u128, get_rand_values, get_proof_options, get_plain_statistics_u128, get_plain_statistics_u64};

    #[test]
    fn stark_a() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_a as Stark;
        use rescue::p128_m4_c3_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 65535u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128)};
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_a_62() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_a_62 as Stark;
        use rescue::p62_m4_c3_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 65535u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE62::new(input_length as u64)};
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_a_griffin() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_a_griffin as Stark;
        use griffin::p128_t4_c3_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 65535u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128)};
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_a_griffin_62() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_a_griffin_62 as Stark;
        use griffin::p62_t4_c3_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 65535u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE62::new(input_length as u64)};
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_b() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_b as Stark;
        use rescue::p128_m4_c2_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 65535u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128)};
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_b_62() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_b_62 as Stark;
        use rescue::p62_m4_c2_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 65535u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE62::new(input_length as u64)};
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_b_griffin() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_b_griffin as Stark;
        use griffin::p128_t4_c2_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 65535u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128)};
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_b_griffin_62() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_b_griffin_62 as Stark;
        use griffin::p62_t4_c2_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 65535u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE62::new(input_length as u64)};
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_c() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_c as Stark;
        use rescue::p128_m4_c2_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 65535u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, _) = get_rand_values::<BaseElement>(begin, end, input_length);
        let mut pixels_base = vec![];
        for i in 0..(input_length / 8) {
            let mut result: u128 = 0;
            for j in 0..8 {
                result |= (pixels_u16[i*8 + j] as u128) << (16u128 * j as u128);
            }
            pixels_base.push(BaseElement::new(result));
        }

        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128)};
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_c_griffin() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_c_griffin as Stark;
        use griffin::p128_t4_c2_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 65535u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, _) = get_rand_values::<BaseElement>(begin, end, input_length);
        let mut pixels_base = vec![];
        for i in 0..(input_length / 8) {
            let mut result: u128 = 0;
            for j in 0..8 {
                result |= (pixels_u16[i*8 + j] as u128) << (16u128 * j as u128);
            }
            pixels_base.push(BaseElement::new(result));
        }

        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128)};
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_d() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_d as Stark;
        use rescue::p128_m4_c2_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 4095u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, _) = get_rand_values::<BaseElement>(begin, end, input_length);
        let mut pixels_base = vec![];
        for i in 0..(input_length / 8) {
            let mut result: u128 = 0;
            for j in 0..8 {
                result |= (pixels_u16[i*8 + j] as u128) << (16u128 * j as u128);
            }
            pixels_base.push(BaseElement::new(result));
        }

        let hash = Hash::hash(&pixels_base);
        let trace = Stark::build_trace(&pixels_u16, &hash);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        assert_eq!(hash_trace, hash);

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128)};
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_d_griffin() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_d_griffin as Stark;
        use griffin::p128_t4_c2_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 4095u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, _) = get_rand_values::<BaseElement>(begin, end, input_length);
        let mut pixels_base = vec![];
        for i in 0..(input_length / 8) {
            let mut result: u128 = 0;
            for j in 0..8 {
                result |= (pixels_u16[i*8 + j] as u128) << (16u128 * j as u128);
            }
            pixels_base.push(BaseElement::new(result));
        }

        let hash = Hash::hash(&pixels_base);
        let trace = Stark::build_trace(&pixels_u16, &hash);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        assert_eq!(hash_trace, hash);

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128)};
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_e() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_e as Stark;
        use rescue::p128_m9_c1_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 80;
        let (begin, end) = (0u16, 80u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;
        let hash_s_result_step = 4 * input_length + Stark::SIZE_OF_T - 1;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
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

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash_pixels_manual = Hash::hash(&pixels_base);
        assert_eq!(hash_pixels_trace, hash_pixels_manual);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u128::<BaseElement>(pixels_u16);
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
        println!("{}", get_stats_string_u128(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(16, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs {
            hash_pixels: hash_pixels_manual,
            hash_omega_l,
            hash_omega_h,
            hash_med,
            hash_omega_m,
            hash_s,
            input_length: BE128::new(input_length as u128),
            sum: manual_stats.sum_e,
            avg_rounded: manual_stats.avg_rounded_e,
            variance: manual_stats.var_e,
            min: min_trace,
            max: max_trace,
            med_low: med_low_trace,
            med_high: med_high_trace,
        };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
        println!()
    }

    #[test]
    fn stark_e_62() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_e_62 as Stark;
        use rescue::p62_m9_c1_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 80;
        let (begin, end) = (0u16, 80u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;
        let hash_s_result_step = 4 * input_length + Stark::SIZE_OF_T - 1;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
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

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash_pixels_manual = Hash::hash(&pixels_base);
        assert_eq!(hash_pixels_trace, hash_pixels_manual);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u64::<BaseElement>(pixels_u16);
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
        println!("{}", get_stats_string_u64(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(16, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs {
            hash_pixels: hash_pixels_manual,
            hash_omega_l,
            hash_omega_h,
            hash_med,
            hash_omega_m,
            hash_s,
            input_length: BE62::new(input_length as u64),
            sum: manual_stats.sum_e,
            avg_rounded: manual_stats.avg_rounded_e,
            variance: manual_stats.var_e,
            min: min_trace,
            max: max_trace,
            med_low: med_low_trace,
            med_high: med_high_trace,
        };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
        println!()
    }

    #[test]
    fn stark_e_griffin() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_e_griffin as Stark;
        use griffin::p128_t12_c4_s100 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 80;
        let (begin, end) = (0u16, 80u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;
        let hash_s_result_step = 4 * input_length + Stark::SIZE_OF_T - 1;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
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

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash_pixels_manual = Hash::hash(&pixels_base);
        assert_eq!(hash_pixels_trace, hash_pixels_manual);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u128::<BaseElement>(pixels_u16);
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
        println!("{}", get_stats_string_u128(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(16, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs {
            hash_pixels: hash_pixels_manual,
            hash_omega_l,
            hash_omega_h,
            hash_med,
            hash_omega_m,
            hash_s,
            input_length: BE128::new(input_length as u128),
            sum: manual_stats.sum_e,
            avg_rounded: manual_stats.avg_rounded_e,
            variance: manual_stats.var_e,
            min: min_trace,
            max: max_trace,
            med_low: med_low_trace,
            med_high: med_high_trace,
        };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_e_griffin_62() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_e_griffin_62 as Stark;
        use griffin::p62_t12_c4_s100 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 80;
        let (begin, end) = (0u16, 80u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;
        let hash_s_result_step = 4 * input_length + Stark::SIZE_OF_T - 1;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
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

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash_pixels_manual = Hash::hash(&pixels_base);
        assert_eq!(hash_pixels_trace, hash_pixels_manual);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u64::<BaseElement>(pixels_u16);
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
        println!("{}", get_stats_string_u64(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(16, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs {
            hash_pixels: hash_pixels_manual,
            hash_omega_l,
            hash_omega_h,
            hash_med,
            hash_omega_m,
            hash_s,
            input_length: BE62::new(input_length as u64),
            sum: manual_stats.sum_e,
            avg_rounded: manual_stats.avg_rounded_e,
            variance: manual_stats.var_e,
            min: min_trace,
            max: max_trace,
            med_low: med_low_trace,
            med_high: med_high_trace,
        };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_e_opt() {
        // need to change constants in stark_e_opt.rs before running this test (SIZE_OF_T vs input length)
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_e_opt as Stark;
        use rescue::p128_m9_c1_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 1040;
        let (begin, end) = (0u16, 80u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;
        let hash_s_result_step = (4 * input_length + Stark::SIZE_OF_T - 1) / 5;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
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

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash_pixels_manual = Hash::hash(&pixels_base);
        assert_eq!(hash_pixels_trace, hash_pixels_manual);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u128::<BaseElement>(pixels_u16);
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
        println!("{}", get_stats_string_u128(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs {
            hash_pixels: hash_pixels_manual,
            hash_omega_l,
            hash_omega_h,
            hash_med,
            hash_omega_m,
            hash_s,
            input_length: BE128::new(input_length as u128),
            sum: manual_stats.sum_e,
            avg_rounded: manual_stats.avg_rounded_e,
            variance: manual_stats.var_e,
            min: min_trace,
            max: max_trace,
            med_low: med_low_trace,
            med_high: med_high_trace,
        };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_e_opt_62() {
        // need to change constants in stark_e_opt.rs before running this test (SIZE_OF_T vs input length)
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_e_opt_62 as Stark;
        use rescue::p62_m9_c1_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 1040;
        let (begin, end) = (0u16, 80u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;
        let hash_s_result_step = (4 * input_length + Stark::SIZE_OF_T - 1) / 5;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
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

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash_pixels_manual = Hash::hash(&pixels_base);
        assert_eq!(hash_pixels_trace, hash_pixels_manual);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u64::<BaseElement>(pixels_u16);
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
        println!("{}", get_stats_string_u64(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs {
            hash_pixels: hash_pixels_manual,
            hash_omega_l,
            hash_omega_h,
            hash_med,
            hash_omega_m,
            hash_s,
            input_length: BE62::new(input_length as u64),
            sum: manual_stats.sum_e,
            avg_rounded: manual_stats.avg_rounded_e,
            variance: manual_stats.var_e,
            min: min_trace,
            max: max_trace,
            med_low: med_low_trace,
            med_high: med_high_trace,
        };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_e_opt_griffin() {
        // need to change constants in stark_e_opt_griffin.rs before running this test (SIZE_OF_T vs input length)
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_e_opt_griffin as Stark;
        use griffin::p128_t12_c4_s100 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 1040;
        let (begin, end) = (0u16, 80u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;
        let hash_s_result_step = (4 * input_length + Stark::SIZE_OF_T - 1) / 5;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
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

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash_pixels_manual = Hash::hash(&pixels_base);
        assert_eq!(hash_pixels_trace, hash_pixels_manual);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u128::<BaseElement>(pixels_u16);
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
        println!("{}", get_stats_string_u128(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs {
            hash_pixels: hash_pixels_manual,
            hash_omega_l,
            hash_omega_h,
            hash_med,
            hash_omega_m,
            hash_s,
            input_length: BE128::new(input_length as u128),
            sum: manual_stats.sum_e,
            avg_rounded: manual_stats.avg_rounded_e,
            variance: manual_stats.var_e,
            min: min_trace,
            max: max_trace,
            med_low: med_low_trace,
            med_high: med_high_trace,
        };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_e_opt_griffin_62() {
        // need to change constants in stark_e_opt_griffin.rs before running this test (SIZE_OF_T vs input length)
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_e_opt_griffin_62 as Stark;
        use griffin::p62_t12_c4_s100 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 1040;
        let (begin, end) = (0u16, 80u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;
        let hash_s_result_step = (4 * input_length + Stark::SIZE_OF_T - 1) / 5;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
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

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash_pixels_manual = Hash::hash(&pixels_base);
        assert_eq!(hash_pixels_trace, hash_pixels_manual);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u64::<BaseElement>(pixels_u16);
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
        println!("{}", get_stats_string_u64(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs {
            hash_pixels: hash_pixels_manual,
            hash_omega_l,
            hash_omega_h,
            hash_med,
            hash_omega_m,
            hash_s,
            input_length: BE62::new(input_length as u64),
            sum: manual_stats.sum_e,
            avg_rounded: manual_stats.avg_rounded_e,
            variance: manual_stats.var_e,
            min: min_trace,
            max: max_trace,
            med_low: med_low_trace,
            med_high: med_high_trace,
        };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f as Stark;
        use rescue::p128_m9_c1_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 20000u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u128::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u128(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f_62() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f_62 as Stark;
        use rescue::p62_m9_c1_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 20000u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u64::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u64(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE62::new(input_length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f_64() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f_64 as Stark;
        use rescue::p64_m9_c1_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 20000u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u64::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u64(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE64::new(input_length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f_griffin() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f_griffin as Stark;
        use griffin::p128_t12_c4_s100 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 20000u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u128::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u128(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f_griffin_62() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f_griffin_62 as Stark;
        use griffin::p62_t12_c4_s100 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 20000u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u64::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u64(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE62::new(input_length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f_opt_m2() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f_opt_m2 as Stark;
        use rescue::p128_m17_c1_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 100u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u128::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u128(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f_opt_m2_62() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f_opt_m2_62 as Stark;
        use rescue::p62_m17_c1_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 100u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u64::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u64(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE62::new(input_length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f_opt_m2_griffin() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f_opt_m2_griffin as Stark;
        use griffin::p128_t20_c4_s100 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 100u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u128::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u128(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f_opt_m2_griffin_62() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f_opt_m2_griffin_62 as Stark;
        use griffin::p62_t20_c4_s100 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 100u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u64::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u64(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE62::new(input_length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f_opt_m4() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f_opt_m4 as Stark;
        use rescue::p128_m33_c1_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 100u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u128::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u128(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f_opt_m4_62() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f_opt_m4_62 as Stark;
        use rescue::p62_m33_c1_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 100u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u64::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u64(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE62::new(input_length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f_opt_m4_griffin() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f_opt_m4_griffin as Stark;
        use griffin::p128_t36_c4_s100 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 100u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u128::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u128(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f_opt_m4_griffin_62() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f_opt_m4_griffin_62 as Stark;
        use griffin::p62_t36_c4_s100 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 100u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u64::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u64(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE62::new(input_length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f_opt_m8() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f_opt_m8 as Stark;
        use rescue::p128_m65_c1_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 100u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u128::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u128(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f_opt_m8_62() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f_opt_m8_62 as Stark;
        use rescue::p62_m65_c1_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 100u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u64::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u64(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE62::new(input_length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f_opt_m8_griffin() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f_opt_m8_griffin as Stark;
        use griffin::p128_t68_c4_s100 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 100u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u128::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u128(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_f_opt_m8_griffin_62() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_f_opt_m8_griffin_62 as Stark;
        use griffin::p62_t68_c4_s100 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 4096;
        let (begin, end) = (0u16, 100u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = input_length * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let manual_stats = get_plain_statistics_u64::<BaseElement>(pixels_u16);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u64(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE62::new(input_length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_g() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_g as Stark;
        use rescue::p128_m9_c1_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 5 * Stark::FRAME_SIZE;
        let (begin, end) = (0u16, 100u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = Stark::CYCLE_LENGTH_ROI * (input_length / Stark::FRAME_SIZE) * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let mut roi_pixels = vec![];
        let stat_mask = Stark::get_stat_mask_roi();
        for i in 0..pixels_u16.len() {
            if stat_mask[i % Stark::FRAME_SIZE] == BaseElement::ONE {
                roi_pixels.push(pixels_u16[i]);
            }
        }

        let manual_stats = get_plain_statistics_u128::<BaseElement>(roi_pixels);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u128(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_g_62() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_g_62 as Stark;
        use rescue::p62_m9_c1_s128 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 5 * Stark::FRAME_SIZE;
        let (begin, end) = (0u16, 100u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = Stark::CYCLE_LENGTH_ROI * (input_length / Stark::FRAME_SIZE) * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let mut roi_pixels = vec![];
        let stat_mask = Stark::get_stat_mask_roi();
        for i in 0..pixels_u16.len() {
            if stat_mask[i % Stark::FRAME_SIZE] == BaseElement::ONE {
                roi_pixels.push(pixels_u16[i]);
            }
        }

        let manual_stats = get_plain_statistics_u64::<BaseElement>(roi_pixels);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u64(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE62::new(input_length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_g_griffin() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_g_griffin as Stark;
        use griffin::p128_t12_c4_s100 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 5 * Stark::FRAME_SIZE;
        let (begin, end) = (0u16, 100u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = Stark::CYCLE_LENGTH_ROI * (input_length / Stark::FRAME_SIZE) * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let mut roi_pixels = vec![];
        let stat_mask = Stark::get_stat_mask_roi();
        for i in 0..pixels_u16.len() {
            if stat_mask[i % Stark::FRAME_SIZE] == BaseElement::ONE {
                roi_pixels.push(pixels_u16[i]);
            }
        }

        let manual_stats = get_plain_statistics_u128::<BaseElement>(roi_pixels);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u128(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::None));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE128::new(input_length as u128), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }

    #[test]
    fn stark_g_griffin_62() {
        //------------------------------------------------------------------------------------------
        // TEST CONFIGURATION
        use stark::stark_g_griffin_62 as Stark;
        use griffin::p62_t12_c4_s100 as Hash;
        type BaseElement = Hash::Elem;
        let input_length: usize = 5 * Stark::FRAME_SIZE;
        let (begin, end) = (0u16, 100u16);

        //------------------------------------------------------------------------------------------
        // INDICES IN THE AET
        let hash_result_step = Stark::CYCLE_LENGTH_ROI * (input_length / Stark::FRAME_SIZE) * Stark::CYCLE_LENGTH / Stark::NUM_ELEMS_PER_CYCLE;
        let stat_result_step = hash_result_step;

        //------------------------------------------------------------------------------------------
        // TRACE CONSTRUCTION
        let (pixels_u16, pixels_base) = get_rand_values::<BaseElement>(begin, end, input_length);
        let trace = Stark::build_trace(&pixels_u16);
        let mut hash_trace = vec![BaseElement::ZERO; Hash::RATE];
        for i in 0..Hash::RATE {
            hash_trace[i] = trace.get(Stark::T_PIXELS_HASH.idx + i, hash_result_step);
        }

        //------------------------------------------------------------------------------------------
        // STARK COMPUTATION CHECKS
        // AET computes the same hash value as the native hash function
        let hash = Hash::hash(&pixels_base);
        assert_eq!(hash_trace, hash);

        // comparison of statistics
        let mut roi_pixels = vec![];
        let stat_mask = Stark::get_stat_mask_roi();
        for i in 0..pixels_u16.len() {
            if stat_mask[i % Stark::FRAME_SIZE] == BaseElement::ONE {
                roi_pixels.push(pixels_u16[i]);
            }
        }

        let manual_stats = get_plain_statistics_u64::<BaseElement>(roi_pixels);
        let sum_trace = trace.get(Stark::T_SUM.begin(), stat_result_step);
        let var_trace = trace.get(Stark::T_VAR.begin(), stat_result_step);
        assert_eq!(sum_trace, manual_stats.sum_e);
        assert_eq!(var_trace, manual_stats.var_e);
        println!("{}", get_stats_string_u64(&manual_stats));
        //------------------------------------------------------------------------------------------

        let prover = Stark::TheProver::new(get_proof_options(8, FieldExtension::Quadratic));
        let proof = prover.prove(trace).unwrap();
        let public_inputs = Stark::PubInputs { hash, input_length: BE62::new(input_length as u64), sum: manual_stats.sum_e, avg_rounded: manual_stats.avg_rounded_e, variance: manual_stats.var_e };
        assert!(winterfell::verify::<Stark::TheAir>(proof, public_inputs).is_ok());
    }
}

pub mod rescue;
pub mod griffin;
pub mod stark;
pub mod utils;
