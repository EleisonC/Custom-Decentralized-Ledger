use crate::domain::{Transaction, TransactionAPIErrors};
use sha2::{Sha256, Digest};

fn serialize_tx(transaction: &Transaction) -> Result<Vec<u8>, TransactionAPIErrors> {
    serde_json::to_vec(transaction).map_err(|_| TransactionAPIErrors::FailedToSignTransaction)
}


// we hash the transaction the was serialized
fn hash_tx(tx_vec: &[u8]) -> Result<Vec<u8>, TransactionAPIErrors> {
    let mut hasher = Sha256::new();
    hasher.update(tx_vec);
    Ok(hasher.finalize().to_vec())
}


