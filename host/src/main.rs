use methods::{PW_CHECK_ID, PW_CHECK_PATH};
use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkp::core::sha::Digest;

fn main() {
    // Make the prover.
    let method_code = std::fs::read(PW_CHECK_PATH)
        .expect("Method code should be present at the specified path; did you use the correct *_PATH constant?");
    let mut prover = Prover::new(&method_code, PW_CHECK_ID).expect(
        "Prover should be constructed from valid method source code and corresponding method ID",
    );

    let pw = String::from("password!");

    prover.add_input(&to_vec(&pw).unwrap()).unwrap();

    // Run prover & generate receipt
    let receipt = prover.run()
        .expect("Code should be provable unless it 1) had an error or 2) overflowed the cycle limit. See `embed_methods_with_options` for information on adjusting maximum cycle count.");

    // Optional: Verify receipt to confirm that recipients will also be able to verify your receipt
    receipt.verify(PW_CHECK_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );

    let digest: Digest = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();
    println!("We proved this hash {} came from a pw with special characters", digest);
}
