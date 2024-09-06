use crate::domain::{Transaction, TransactionAPIErrors};
use sha2::{Sha256, Digest};
use ring::{rand, signature};
use ring::signature::{EcdsaKeyPair, KeyPair, Signature, ECDSA_P256_SHA256_FIXED_SIGNING};

pub fn sign_my_tx(mut transaction: &mut Transaction, private_key: &[u8]) -> Result<(), TransactionAPIErrors> {
    let tx_vec = serialize_tx(transaction)?;
    let hashed_tx = hash_tx(&tx_vec).map_err(|_| TransactionAPIErrors::FailedToSignTransaction)?;
    let signature = sign_transaction(private_key, &hashed_tx).map_err(|_| TransactionAPIErrors::FailedToSignTransaction)?;
    attach_signature(transaction, signature);
    Ok(())
}


fn serialize_tx(transaction: &Transaction) -> Result<Vec<u8>, TransactionAPIErrors> {
    serde_json::to_vec(transaction).map_err(|_| TransactionAPIErrors::FailedToSignTransaction)
}


// we hash the transaction the was serialized
fn hash_tx(tx_vec: &[u8]) -> Result<Vec<u8>, TransactionAPIErrors> {
    let mut hasher = Sha256::new();
    hasher.update(tx_vec);
    Ok(hasher.finalize().to_vec())
}

fn sign_transaction(private_key: &[u8], hashed_tx: &[u8]) ->  Result<Signature, TransactionAPIErrors> {
    let rng = rand::SystemRandom::new();

    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, private_key, &rng).map_err(|_| TransactionAPIErrors::FailedToSignTransaction)?;

    key_pair.sign(&rng, hashed_tx).map_err(|_| TransactionAPIErrors::FailedToSignTransaction)
}

fn attach_signature(transaction: &mut Transaction, signature: Signature) {
    transaction.signature = Some(hex::encode(signature.as_ref()));
}


