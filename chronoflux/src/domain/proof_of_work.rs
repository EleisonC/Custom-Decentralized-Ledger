use num_bigint::{BigInt, Sign};
use super::Block;
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