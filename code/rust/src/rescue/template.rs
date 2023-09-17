
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
