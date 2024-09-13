use ring::rand::SystemRandom;
use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};
use chrono::Utc;
use std::fs::File;
use std::io::Write;



pub fn generate_private_key() -> Result<(), String> {
    let rng = SystemRandom::new();

    let private_key_pkcs8 = match EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng) {
        Ok(private_key) => private_key,
        Err(err) => return Err(format!("Failed to generate private key: {}", err))
    };

    let private_key_bytes = private_key_pkcs8.as_ref().to_vec();

    let current_time = Utc::now();
    let formatted_time = current_time.format("%Y-%m-%d-%H-%M-%S");

    let filename = format!("eleisonC-chain-pk-{}.txt", formatted_time);

    let file = match File::create(filename.clone()) {
        Ok(mut file) => file.write_all(&private_key_bytes),
        Err(err) => return Err(format!("Failed to create file: {}", err))
    };

    if let Err(e) = file {
        return Err(format!("Failed to write to file: {}", e));
    }

    println!("Private key saved to: {}", filename);

    Ok(())
}

