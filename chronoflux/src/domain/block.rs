use chrono::{DateTime, Utc};
use bincode::{serialize, deserialize};
use serde::{Deserialize, Serialize};
use sled::IVec;

use crate::utils::sha256_digest;

use super::{transactions, Transaction};

#[derive(Deserialize, Serialize)]
pub struct Block {
    timestamp: DateTime<Utc>,
    pre_block_hash: String,
    hash: Option<String>,
    transactions: Vec<Transaction>,
    nonce: Option<u64>,
    height: Option<usize>
}

impl Block {
    pub fn new(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Self {
        Block {
            timestamp: Utc::now(),
            pre_block_hash,
            hash: None,
            transactions: transactions.to_vec(),
            nonce: None,
            height: None
        }
    }

    pub fn deserialize(bytes: &[u8]) -> Block {
        bincode::deserialize(bytes).unwrap()
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap().to_vec()
    }

    pub fn get_transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    pub fn get_pre_block_hash(&self) -> String {
        self.pre_block_hash.clone()
    }

    pub fn get_hash(&self) -> Option<&str> {
        self.hash.as_ref().map(|s| s.as_str())
    }

    pub fn get_hash_bytes(&self) -> Option<Vec<u8>> {
        self.hash.as_ref().map(|s| s.as_bytes().to_vec())
    }

    pub fn get_timestamp(&self) -> DateTime<Utc> {
        self.timestamp.clone()
    }

    pub fn get_height(&self) -> usize {
        self.height.unwrap()
    }

    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut txhashs = vec![];
        for transaction in &self.transactions {
            txhashs.extend(transaction.get_id())
        }

        sha256_digest(&txhashs)
    }

    pub fn generate_genesis_block(transaction: &Transaction) -> Block {
        let transactions = vec![transaction.clone()];
        return Block::new(String::from("None"), &transactions, 0)
    }

}

impl From<Block> for IVec {
    fn from(b: Block) -> Self {
        let bytes = bincode::serialize(&b).unwrap();
        Self::from(bytes)
    }
} 
