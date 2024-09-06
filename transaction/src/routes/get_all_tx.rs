use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{Transaction, TransactionAPIErrors}};


#[derive(Deserialize, Debug, Serialize)]
pub struct AllTransactionResponse {
    pub transactions: Vec<Transaction>
}

pub  async fn get_all_tx(State(state): State<AppState>) -> Result<impl IntoResponse, TransactionAPIErrors> {
    let transaction_store = state.transaction_list.read().await;

    let transactions = transaction_store.get_transactions().await;
    let mut all_transactions: Vec<Transaction> = Vec::new();

    if let Ok(transactions) = transactions {
        all_transactions = transactions
    } 

    let response = Json(AllTransactionResponse{
        transactions: all_transactions
    });

    Ok((StatusCode::OK, response))
}
