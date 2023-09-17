
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