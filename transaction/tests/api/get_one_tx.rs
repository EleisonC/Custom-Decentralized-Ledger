use crate::helper::{self, TestApp};
use transaction::routes::GetTransactionResponse;


#[tokio::test]
async fn should_return_200_if_tx_exists() {
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

    let response = test_app.get_one_transaction_by_index(1).await;

    assert_eq!(response.status().as_u16(), 200, "Failed to get transaction");
    
    let transaction_response = response
        .json::<GetTransactionResponse>()
        .await
        .expect("Could not deserialize the response body");
    assert!(transaction_response.transaction.amount == 10,
        "Expected the transaction to match the created transaction"
    )
}

#[tokio::test]
async fn should_return_404_if_tx_does_not_exist() {
    let test_app = TestApp::new().await;

    let response = test_app.get_one_transaction_by_index(100).await;

    assert_eq!(response.status().as_u16(), 404, "Expected 404 for non-existent transaction");
}
