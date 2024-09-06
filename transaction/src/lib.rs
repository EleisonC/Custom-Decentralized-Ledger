use std::error::Error;

use axum::{
    http::StatusCode, response::{IntoResponse, Response}, routing::{get,post}, serve::Serve, Json, Router
};

use app_state::AppState;
use domain::TransactionAPIErrors;
use serde::{Deserialize, Serialize};

pub mod domain;
pub mod routes;
pub mod services;
pub mod app_state;
pub mod utils;


#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for TransactionAPIErrors {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            TransactionAPIErrors::InvalidInformation => (StatusCode::CONFLICT, "Invalid information"),
            TransactionAPIErrors::TransactionNotFound => (StatusCode::NOT_FOUND, "Record Not Found"),
            TransactionAPIErrors::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "Uexpected error"),
            TransactionAPIErrors::InvalidIndex => (StatusCode::CONFLICT, "Invalid transaction index"),
            TransactionAPIErrors::FailedToSignTransaction => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to sign the transaction")
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
    
        (status, body).into_response()
    }
}


pub struct Application {
    server: Serve<Router, Router>,
    pub address: String
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let app = Router::new()
            .route("/", get(health_check))
            .route("/create-tx", post(routes::create_tx))
            .route("/get-all-transactions", get(routes::get_all_tx))
            .route("/get-transaction-by-index/:tx_index", get(routes::get_transaction_by_index))
            .with_state(app_state.clone());

        let router = app;

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        let app_inst = Application {
            server,
            address
        };

        Ok(app_inst)
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await  
    }
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
