use crate::helper::{self, TestApp};
// use transaction::routes::CreationRequest;

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let test_app = TestApp::new().await;

    let random_reciv = helper::get_random_email();
    let random_sender = helper::get_random_email();

    let creation_request = serde_json::json!({
        "sender": random_sender,
        "recipient": random_reciv,
        "amount": 10.0,
    });

    let response = test_app.create_tx(&creation_request).await;

    assert_eq!(response.status().as_u16(),
    201,
    "Transaction created successfully!"
    );
}