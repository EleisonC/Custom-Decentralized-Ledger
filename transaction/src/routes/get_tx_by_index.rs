
use axum::{extract::{State, Path}, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{Transaction, TransactionAPIErrors}};


#[derive(Serialize, Deserialize, Debug)]
pub struct GetTransactionResponse {
    pub transaction: Transaction,
}

pub async fn get_transaction_by_index(Path(tx_index): Path<u32>, State(state): State<AppState>) -> Result<impl IntoResponse, TransactionAPIErrors> {
    let transaction_store = state.transaction_list.read().await;

    let transaction = transaction_store.get_transaction_by_index(tx_index).await;

    match transaction {
        Ok(transaction) => Ok((StatusCode::OK, Json(GetTransactionResponse{transaction}))),
        Err(_) => Err(TransactionAPIErrors::TransactionNotFound),
    }
}