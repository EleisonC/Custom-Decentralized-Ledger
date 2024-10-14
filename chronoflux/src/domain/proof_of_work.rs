use num_bigint::{BigInt, Sign};
use super::{transactions, Block};
use std::{borrow::Borrow, ops::ShlAssign};
use crate::utils::sha256_digest;
use data_encoding::HEXLOWER;




const TARGET_BITS: i32 = 100;

const MAX_NONCE: i64 = i64::MAX;

pub struct ProofOfWork {
    block: Block,
    target: BigInt
}

impl ProofOfWork {
    pub fn new_proof_of_work(block: Block) -> Self {
        let mut target = BigInt::from(1);
        target.shl_assign(256 - TARGET_BITS);
        ProofOfWork {
            block,
            target
        }
    }

    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let pre_block_hash = self.block.get_pre_block_hash();
        let transactions_hash = self.block.hash_transactions();
        let timestamp = self.block.get_timestamp();

        let mut data_bytes = vec![];
        data_bytes.extend(pre_block_hash.as_bytes());
        data_bytes.extend(transactions_hash);
        data_bytes.extend(timestamp.timestamp().to_be_bytes());
        data_bytes.extend(TARGET_BITS.to_be_bytes());
        data_bytes.extend(nonce.to_be_bytes());
        data_bytes
    }

    pub fn run(&self) -> (i64, String) {
        let mut nounce = 0;

        let mut hash = Vec::new();

        print!("Mining the block");

        while nounce < MAX_NONCE {
            let data = self.prepare_data(nounce);

            hash = sha256_digest(data.as_slice());
            let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());

            if hash_int.lt(self.target.borrow()) {
                println!("{}", HEXLOWER.encode(hash.as_slice()));
                break;
            } else {
                nounce += 1
            }
        }
        println!();
        return (nounce, HEXLOWER.encode(hash.as_slice()))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Block, Transaction};

    #[test]
    fn test_new_proof_of_work_func() {
    
    }

}
