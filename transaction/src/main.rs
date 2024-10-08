use std::sync::Arc;

use tokio::sync::RwLock;
use transaction::{app_state::AppState, services::VecTransactionsStore, Application};

#[tokio::main]
async fn main() {
    let transaction_store = Arc::new(RwLock::new(VecTransactionsStore::default()));
    let app_state = AppState::new(transaction_store);

    
    let app = Application::build(app_state,"0.0.0.0:2000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}