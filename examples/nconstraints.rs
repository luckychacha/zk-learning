use ark_bn254::Bn254;
use ark_circom::CircomBuilder;
use ark_circom::CircomConfig;
use ark_groth16::Groth16;
use ark_snark::SNARK;
use ark_std::rand;

fn main() {
    // Load the WASM and R1CS for witness and proof generation
    let cfg = CircomConfig::<Bn254>::new(
        "test_circuits/n_test/nconstraints_js/nconstraints.wasm",
        "test_circuits/n_test/nconstraints.r1cs",
    )
    .unwrap();

    // Insert our secret inputs as key value pairs. We insert a single input, namely the input to the hash function.
    let mut builder = CircomBuilder::new(cfg);
    let secret_input = 200000;
    builder.push_input("in", secret_input);
    println!("Secret inputs: {:?}", secret_input);

    // Create an empty instance for setting it up
    let circom = builder.setup();

    // WARNING: The code below is just for debugging, and should instead use a verification key generated from a trusted setup.
    // See for example https://docs.circom.io/getting-started/proving-circuits/#powers-of-tau.
    let mut rng = rand::thread_rng();
    let params =
        Groth16::<Bn254>::generate_random_parameters_with_reduction(circom, &mut rng).unwrap();

    let circom = builder.build().unwrap();

    // There's only one public input, namely the hash digest.
    let inputs: Vec<ark_ff::Fp<ark_ff::MontBackend<ark_bn254::FrConfig, 4>, 4>> =
        circom.get_public_inputs().unwrap();

    // Generate the proof
    let proof = Groth16::<Bn254>::prove(&params, circom, &mut rng).unwrap();
    println!("Proof: {:?}", proof);

    // Check that the proof is valid
    let pvk: ark_groth16::PreparedVerifyingKey<ark_ec::bn::Bn<ark_bn254::Config>> =
        Groth16::<Bn254>::process_vk(&params.vk).unwrap();
    // println!("Verification key: {:?}", pvk.vk);
    println!("alpha_g1: {:?}", pvk.vk.alpha_g1);
    println!("beta_g2: {:?}", pvk.vk.beta_g2);
    println!("gamma_g2: {:?}", pvk.vk.gamma_g2);
    println!("delta_g2: {:?}", pvk.vk.delta_g2);
    println!("gamma_abc_g1: {:?}", pvk.vk.gamma_abc_g1);
    println!("Public inputs: {:?}", inputs);

    // println!("Verification key: {:?}", pvk.alpha_g1_beta_g2);
    // println!("Verification key: {:?}", pvk.delta_g2_neg_pc);
    // println!("Verification key: {:?}", pvk.gamma_g2_neg_pc);
    let verified = Groth16::<Bn254>::verify_with_processed_vk(&pvk, &inputs, &proof).unwrap();
    println!("Verified: {:?}", verified);
    assert!(verified);
}
