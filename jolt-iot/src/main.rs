pub fn main() {
    // Get the prover and verifier functions
    let (prove_is_environment_healthy, verify_is_environment_healthy) = guest::build_is_environment_healthy();

    // Start timing
    let start = std::time::Instant::now();
    
    // Generate proof and get result
    let (is_healthy, proof) = prove_is_environment_healthy(2000, 800, 45);
    
    // Calculate elapsed time
    let duration = start.elapsed();
    let proof_size = std::mem::size_of_val(&proof);
    
    let is_valid = verify_is_environment_healthy(proof);
    println!("Environment healthy: {}", is_healthy);
    println!("Proof valid: {}", is_valid);
    println!("Proof generation time: {:?}", duration);
    println!("Proof size: {} bytes", proof_size);
}
