use axum::{http::StatusCode, extract::State,
    response::IntoResponse, Json};
use serde::{Deserialize, Serialize};


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


pub async fn create_tx(Json(request): Json<CreationRequest>) -> impl IntoResponse {
    
}

