use super::Transaction;


#[derive(Debug, PartialEq)]
pub enum TransactionStoreError {
    TransactionNotFound,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait TransactionStore {
    async fn add_transaction(&mut self, transaction: Transaction) -> Result<(), TransactionStoreError>;
    async fn get_transactions(&self) -> Result<Vec<Transaction>, TransactionStoreError>;
    async fn get_transaction_by_index(&self, index_num: u32) -> Result<Transaction, TransactionStoreError>;
}