use std::{env::current_dir, sync::{Arc, RwLock}};
use sled::{transaction::TransactionError, Db, Tree};

use super::{block, Block, Transaction};

const TIP_BLOCK_HASH_KEY: &str = "top_of_the_block_hash";
const BLOCKS_TREE: &str = "blocks";

#[derive(Clone)]
pub struct Blockchain {
    tip_hash: Arc<RwLock<String>>,
    db: Db
}


impl Blockchain {

    pub fn create_blockchain(genesis_addr: &str) -> Blockchain {
        let db = sled::open(current_dir().unwrap().join("data")).unwrap();
        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();

        let data = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
        let tip_hash;

        if data.is_none() {
            let chronoflux_tx = Transaction::new_chronoflux_tx(genesis_addr);
            let block = Block::generate_genesis_block(&chronoflux_tx);

            Self::update_blocks_tree(&blocks_tree, &block).unwrap();
            tip_hash = String::from(block.get_hash().unwrap());
        } else {
            tip_hash = String::from_utf8(data.unwrap().to_vec()).unwrap();
        }
        Blockchain {
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db
        }
    }

    fn update_blocks_tree(blocks_tree: &Tree, block: &Block) -> Result<(), String>{
        let block_hash = block.get_hash().ok_or("error retriving block hash")?;
        
        let result: Result<(), TransactionError<String>>  = blocks_tree.transaction(|tx_db| {
            tx_db.insert(block_hash, *block.clone()).map_err(|err| {
                format!("Error updating blocks tree: {}", err)
            });
            tx_db.insert(TIP_BLOCK_HASH_KEY, block_hash)?.ok_or("error retriving block hash");
            Ok(())
        });

        if let Err(err) = result {
            return Err("error updating blocks tree".to_string())
        }
        Ok(())
    }

    pub fn new_blockchain() -> Blockchain {
        let db = sled::open(current_dir().unwrap().join("data")).unwrap();

        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();
        let tip_bytes = blocks_tree
            .get(TIP_BLOCK_HASH_KEY)
            .unwrap()
            .expect("no existing blockchain found. create one frist");

        let tip_hash = String::from_utf8(tip_bytes.to_vec()).unwrap();
        Blockchain {
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db
        }
    }

    pub fn get_db(&self) -> &Db {
        &self.db
    }

    pub fn get_tip_hash(&self) -> String {
        self.tip_hash.read().unwrap().clone()
    }


}

pub struct BlockchainIterator {
    db: Db,
    current_hash: String,
}

impl BlockchainIterator {
    fn new (tip_hash: String, db: Db) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: tip_hash,
            db
        }
    }

    pub fn next(&mut self) -> Option<Block> {
        let block_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        let data = block_tree.get(self.current_hash.clone()).unwrap();

        if data.is_none() {
            return None
        }

        let block = Block::deserialize(data.unwrap().to_vec().as_slice());
        self.current_hash = block.get_pre_block_hash().clone();
        
        Some(block)
    }
}
