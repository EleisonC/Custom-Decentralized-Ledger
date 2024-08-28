use axum::{http::StatusCode, extract::State,
    response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;


#[derive(Deserialize)]
pub struct CreationRequest {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
}

#[derive(Deserialize, Serialize)]
pub struct CreationResponse {
    pub message: String
}


pub async fn create_tx(State(state): State<AppState>, Json(request): Json<CreationRequest>) -> impl IntoResponse {
    // let sender_info = 
    
}

