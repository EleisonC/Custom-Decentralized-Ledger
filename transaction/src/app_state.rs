use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::TransactionStore;

pub type TransactionStoreType = Arc<RwLock<dyn TransactionStore + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub transaction_list: TransactionStoreType
}

impl AppState {
    pub fn new(transaction_store: TransactionStoreType) -> Self {
        Self { transaction_list: transaction_store }
    }
}
