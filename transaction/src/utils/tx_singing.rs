use crate::domain::{Transaction, TransactionAPIErrors};
use sha2::{Sha256, Digest};
use ring::{rand, signature};
use ring::signature::{EcdsaKeyPair, KeyPair, Signature, ECDSA_P256_SHA256_FIXED_SIGNING};
use ring::pkcs8::Document;

pub fn sign_my_tx(transaction: &mut Transaction, private_key: &[u8]) -> Result<(), TransactionAPIErrors> {
    let tx_vec = serialize_tx(transaction).map_err(|err_msg| {
        println!("This is the err {}", err_msg);
        TransactionAPIErrors::FailedToSignTransaction
    })?;
    
    let hashed_tx = hash_tx(&tx_vec).map_err(|_| TransactionAPIErrors::FailedToSignTransaction)?;
    let signature = sign_transaction(private_key, &hashed_tx).map_err(|_| TransactionAPIErrors::FailedToSignTransaction)?;
    attach_signature(transaction, signature);
    Ok(())
}

fn serialize_tx(transaction: &Transaction) -> Result<Vec<u8>, String> {
    if let Ok(result) = serde_json::to_vec(transaction) {
        Ok(result)
    } else {
        Err("Failed to serialize transaction".to_string())
    }
}

// we hash the transaction the was serialized
fn hash_tx(tx_vec: &[u8]) -> Result<Vec<u8>, TransactionAPIErrors> {
    let mut hasher = Sha256::new();
    hasher.update(tx_vec);
    Ok(hasher.finalize().to_vec())
}

fn sign_transaction(private_key: &[u8], hashed_tx: &[u8]) ->  Result<Signature, TransactionAPIErrors> {
    let rng = rand::SystemRandom::new();

    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, private_key, &rng).map_err(|err_msg| {
        println!("This could be the error message 1: {}", err_msg);
        TransactionAPIErrors::FailedToSignTransaction})?;

    key_pair.sign(&rng, hashed_tx).map_err(|err_msg| {
        println!("This could be the error message 2: {}", err_msg);
        TransactionAPIErrors::FailedToSignTransaction
    })
}

fn attach_signature(transaction: &mut Transaction, signature: Signature) {
    transaction.signature = Some(hex::encode(signature.as_ref()));
    transaction.tx_status = "transaction signed".to_string();
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::{Email, Transaction};
    use hex;

    #[tokio::test]
    async fn it_signs_and_attaches_signature() {
        let private_key = "308187020100301306072a8648ce3d020106082a8648ce3d030107046d306b02010104203c1636a384b67f7099a5923d22af174427cd910e600cb3245edb90d8f95bcb66a14403420004e461420ee838d41b5a03d91b5dc70f79e84e85ad56e3214290bcd260e9a9cfa8ec593d1d9a478c0d72e861ac284022ab403cb131b2b09d6750f72bbda6f845e1";
        let send_mail = Email::parse("send@mail.com".to_owned()).unwrap();
        let receive_mail = Email::parse("receive@mail.com".to_owned()).unwrap();
        let mut tx = Transaction::new(send_mail, receive_mail, 20);
        let decoded_hex_pk = hex::decode(private_key).expect("Failed to decode hex string");

        if sign_my_tx(&mut tx, &decoded_hex_pk).is_err() {
            panic!("Failed to sign transaction");
        }
        
        assert!(tx.signature.is_some(), "Expected the signature to be present in the response");
        assert_eq!(tx.tx_status, "transaction signed", "Expected the tx_status to be 'transaction signed'")
    }
}


