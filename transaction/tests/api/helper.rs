use std::sync::Arc;

use tokio::sync::RwLock;
use transaction::{app_state::AppState, services::VecTransactionsStore, Application};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let transaction_store = Arc::new(RwLock::new(VecTransactionsStore::default()));
        let test_app_state = AppState::new(transaction_store);

        let app = Application::build(test_app_state,"127.0.0.1:0")
            .await
            .expect("Failed to build test app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a seprate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new();
        let test_app = TestApp {
            address,
            http_client
        };

        test_app
    }

    pub async fn health_check(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", self.address))
            .send()
            .await
            .expect("Failed to send health check request")
    }

    pub async fn create_tx<Body>(&self, body: &Body) -> reqwest::Response
    where
    Body: serde::Serialize
     {
        self.http_client
            .post(&format!("{}/create-tx", self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to send health check request")
    }

    pub async fn get_all_transactions(&self) -> reqwest::Response { 
        self.http_client
            .get(&format!("{}/get-all-transactions", self.address))
            .send()
            .await
            .expect("Failed to send health check request")
    }

    pub async fn get_one_transaction_by_index(&self, index: u32) -> reqwest::Response { 
        self.http_client
            .get(&format!("{}/get-transaction-by-index/{index}", self.address))
            .send()
            .await
            .expect("Failed to send health check request")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
