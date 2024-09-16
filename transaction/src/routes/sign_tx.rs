use axum::{extract::{State, Path}, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, domain::{Transaction, TransactionAPIErrors}, utils::sign_my_tx};


#[derive(Deserialize, Debug)]
pub struct SignTransactionRequest {
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignTransactionResponse {
    pub transaction: Transaction,
}


pub async fn sign_transaction(
    Path(tx_index): Path<u32>, 
    State(state): State<AppState>,
    Json(request): Json<SignTransactionRequest>
) -> Result<impl IntoResponse, TransactionAPIErrors> {
    let transaction_store = state.transaction_list.read().await;

    let mut transaction = transaction_store.get_transaction_by_index(tx_index).await.map_err(|_| TransactionAPIErrors::TransactionNotFound)?;

    let private_key_bytes = hex::decode(request.signature).expect("Failed to decode hex string");

    sign_my_tx(&mut transaction, &private_key_bytes).map_err(|_|  TransactionAPIErrors::SigningError)?;

    Ok((StatusCode::OK, Json(SignTransactionResponse{transaction})))
}