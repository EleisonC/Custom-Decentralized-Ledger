use crate::helper::{self, TestApp};
use transaction::routes::AllTransactionResponse;

#[tokio::test]
async fn should_return_200_for_all_records() {
    let test_app = TestApp::new().await;

    let random_sender = helper::get_random_email();
    let random_reciv = helper::get_random_email();

    let new_txs = [
        serde_json::json!({
        "sender": random_sender,
        "recipient": random_reciv,
        "amount": 15,
        }),
        serde_json::json!({
            "sender": random_sender,
            "recipient": random_reciv,
            "amount": 10,
        }),
        serde_json::json!({
            "sender": random_sender,
            "recipient": random_reciv,
            "amount": 1,
        })
    ];

    for tx in new_txs.iter() {
        let response = test_app.create_tx(&tx).await;
        assert_eq!(response.status().as_u16(), 201, "Failed to create transaction");
    }

    let response = test_app.get_all_transactions().await;

    assert_eq!(response.status().as_u16(), 200, "Failed to get all transactions");
    
    let transactions_response = response
        .json::<AllTransactionResponse>()
        .await
        .expect("Could not deserialize response body to AllTransactionResponse");

    // Assert that you received a non-empty list of transactions
    assert!(!transactions_response.transactions.is_empty(), "Expected a list of transactions but got an empty list.");
}
