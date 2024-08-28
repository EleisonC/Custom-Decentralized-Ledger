use crate::domain::{Transaction, TransactionStore, TransactionStoreError};

pub struct VecTransactionsStore {
    transactions_list: Vec<Transaction>
}


#[async_trait::async_trait]
impl TransactionStore for VecTransactionsStore {
    async fn add_transaction(&mut self, transaction: Transaction) -> Result<(), TransactionStoreError> {
        self.transactions_list.push(transaction);
        Ok(())
    }

    async fn get_transactions(&self) -> Result<Vec<Transaction>, TransactionStoreError> {
        Ok(self.transactions_list.clone())
    }

    async fn get_transaction_by_index(&self, index_num: u32) -> Result<Transaction, TransactionStoreError> {
        if index_num < self.transactions_list.len() as u32 {
            Ok(self.transactions_list[index_num as usize].clone())
        } else {
            Err(TransactionStoreError::TransactionNotFound)
        }
    }
}
