use axum::{http::StatusCode, extract::State,
    response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{Email, Transaction, TransactionAPIErrors}};


#[derive(Deserialize, Debug)]
pub struct CreationRequest {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
}

#[derive(Deserialize, Serialize)]
pub struct CreationResponse {
    pub message: String
}


pub async fn create_tx(State(state): State<AppState>, Json(request): Json<CreationRequest>) -> Result<impl IntoResponse, TransactionAPIErrors> {
    let sender_info = Email::parse(request.sender.clone()).map_err(|_| TransactionAPIErrors::InvalidInformation)?;
    let recip_info = Email::parse(request.recipient.clone()).map_err(|_| TransactionAPIErrors::InvalidInformation)?;

    let amount: u64;

    if request.amount == 0 {
        return Err(TransactionAPIErrors::InvalidInformation);
    } else {
        amount = request.amount;
    }

    let new_transaction = Transaction::new(sender_info, recip_info, amount);

    let mut transaction_store = state.transaction_list.write().await;

    if transaction_store.add_transaction(new_transaction).await.is_err() {
        return Err(TransactionAPIErrors::UnexpectedError);
    }

    let response = Json(CreationResponse {
        message: "Transaction created successfully!".to_string()
    });
    
    Ok((StatusCode::CREATED, response))
}

