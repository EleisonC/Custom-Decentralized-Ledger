use ring::rand::SystemRandom;
use ring::signature::{EcdsaKeyPair, KeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};
use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use chrono::Utc;

use crate::utils::save_to_file;



pub fn generate_private_key() -> Result<(), String> {
    let rng = SystemRandom::new();

    let private_key_pkcs8 = match EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng) {
        Ok(private_key) => private_key,
        Err(err) => return Err(format!("Failed to generate private key: {}", err))
    };

    let private_key_bytes = private_key_pkcs8.as_ref();

    let key_pair = match EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, private_key_bytes) {
        Ok(key_pair) => key_pair,
        Err(err) => return Err(format!("Failed to parse private key: {}", err))
    };

    let pub_key_bytes = key_pair.public_key().as_ref().to_vec();
    let pub_address = generate_public_address(&pub_key_bytes);

    let current_time = Utc::now();
    let formatted_time = current_time.format("%Y-%m-%d-%H-%M-%S");

    let pk_filename = format!("onePiece-chain-pk-{}.txt", formatted_time);
    let pub_key_filename = format!("onePiece-chain-pub-key-{}.txt", formatted_time);
    let pub_address_filename = format!("onePiece-chain-pub-address-{}.txt", formatted_time);
    

    save_to_file(&pk_filename, &private_key_bytes)?;

    save_to_file(&pub_key_filename, &pub_key_bytes)?;

    save_to_file(&pub_address_filename, &pub_address.as_bytes())?;

    println!("Private key saved to: {}", pk_filename);
    println!("Public key saved to: {}", pub_key_filename);
    println!("Public address: {}", pub_address);

    Ok(())
}

fn generate_public_address(public_key_bytes: &[u8]) -> String {
    let sha256 = Sha256::digest(public_key_bytes);

    let ripemd160 = Ripemd160::digest(&sha256);

    hex::encode(ripemd160)
}

