# Master’s Thesis: "Fitting Huge Datasets into Zero-Knowledge Proof Systems"

2021/2022

**Study programme**: Computer Science (Graz University of Technology)

**DISCLAIMER**: The variant of <a href="https://eprint.iacr.org/2022/403">Griffin</a> implemented in this project is a preliminary version. <a href="https://eprint.iacr.org/2022/403">Griffin</a> was designed in parallel to writing this thesis. The variant of <a href="https://eprint.iacr.org/2022/403">Griffin</a> in this project is INSECURE. Furthermore, we use a capacity of 1 for <a href="https://eprint.iacr.org/2020/1143">Rescue-Prime</a> which is also considered insecure.

**Furthermore repeating the WARNING from <a href="https://github.com/facebook/winterfell">winterfell</a>**: This is a research project. It has not been audited and may contain bugs and security flaws. This implementation is NOT ready for production use.

We utilized the library <a href="https://github.com/facebook/winterfell">winterfell</a> (version 0.3.0) to implement a STARK that uses a private video as input. The public outputs are a hash of the video and statistics about the video's content. The private input (video) is a list of frames (simplified: a list of pixels). A pixel is a 16-bit unsigned integer representing temperature (infrared camera).

The statement of our STARK is

"I know a video that hashes to ABC and its statistics are XYZ"

with the goal of keeping the video private, while revealing certain statements about the video (mean/average/min/max temperature within a given region of influence). Industry interest: Is it feasible to use large inputs (e.g. a 1 GB video)?

The motivation for the above statement is elaborated on further in Chapter 3 (Problem Definition and Context) of the thesis.

The full thesis (thesis/thesis.pdf) explains the code. This was a pen-and-paper project with first designing the STARKs on paper and then "writing them down" using the winterfell library. The code itself is poorly documented, as the full thesis provides a full explanation of the AET, transition/boundary constrains, periodic columns, variable names (and their indices in the tables (AET, periodic column table)).

We used lookup tables for designing our STARKs. Winterfell 0.3.0 does not natively support Randomized AIRs with Pre-Processing (RAPs) (being a requirement for lookup tables), therefore we used the Fiat-Shamir heuristic to create pseudo randomness by computing a hash inside the AET, which is expensive. More recent versions of winterfell support RAPs natively. The existing code could be extended to use these native RAPs which should result in increased performance.

Details on the performance and results are to be found in the full thesis (thesis/thesis.pdf).

## Code Compilation/Execution

### Running the Benchmarker

To build the benchmarker binary: Inside `/code/rust` run

    cargo build --release --features concurrent --features master_thesis_quarter

After compilation the binary is ready for testing:

    code/rust/target/release/master_thesis

Run the binary with the flag `-h` for help. `--features concurrent` enables multithreaded proof generation for winterfell. `--features master_thesis_quarter` defines sizes of `t` for plookup and the size of one frame (video resolution). The feature `master_thesis_quarter` allows to run all STARKs with less than 11GB RAM, where the script `eval_local_quarter_time.sh` performs tests on all STARKs. The available features are

    master_thesis_full
    master_thesis_half
    master_thesis_quarter
    master_thesis_test

where `master_thesis_full` uses the configuration for inputs that almost use the full 16-bit range and the video resolution provided by the industry partner of 382x288 pixels.

Select a STARK with the `-s` flag, e.g. for running STARK F:

    code/rust/target/release/master_thesis -s stark_f

The following STARKs are available for execution:

    stark_a
    stark_a_62
    stark_a_griffin
    stark_a_griffin_62
    stark_b
    stark_b_62
    stark_b_griffin
    stark_b_griffin_62
    stark_c
    stark_c_griffin
    stark_d
    stark_d_griffin
    stark_e
    stark_e_62
    stark_e_griffin
    stark_e_griffin_62
    stark_e_opt
    stark_e_opt_62
    stark_e_opt_griffin
    stark_e_opt_griffin_62
    stark_f
    stark_f_62
    stark_f_64
    stark_f_griffin
    stark_f_griffin_62
    stark_f_opt_m2
    stark_f_opt_m2_62
    stark_f_opt_m2_griffin
    stark_f_opt_m2_griffin_62
    stark_f_opt_m4
    stark_f_opt_m4_62
    stark_f_opt_m4_griffin
    stark_f_opt_m4_griffin_62
    stark_f_opt_m8
    stark_f_opt_m8_62
    stark_f_opt_m8_griffin
    stark_f_opt_m8_griffin_62
    stark_g
    stark_g_62
    stark_g_griffin
    stark_g_griffin_62

More options are available for defining the length and domain of the input sequence. The binary returns with exit code `0` when all steps (building the trace, proving, and verification) complete successfully. Assertions get triggered as soon as any step fails with a non-zero exit code.

To receive some output, define the variable `RUST_LOG` before running the binary. Performance metrics get logged as `info`, more detailed information is available in the `trace` log. See https://docs.rs/env_logger/latest/env_logger/

### Running tests

Inside `/code/rust` run

    cargo test <test> --features master_thesis_test
    
