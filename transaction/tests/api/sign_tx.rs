use transaction::routes::GetTransactionResponse;

use crate::helper::{self, TestApp};


#[tokio::test]
async fn should_return_200_tx_signing_success() {
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
        assert_eq!(response.status().as_u16(), 201, "Failed to create transactions")
    }

    let response_1 = test_app.get_one_transaction_by_index(1).await;

    assert_eq!(response_1.status().as_u16(), 200, "Failed to get transaction");

    let transaction_response_1 = response_1
        .json::<GetTransactionResponse>()
        .await
        .expect("Failed to deserialize the response");

    assert!(transaction_response_1.transaction.amount == 10,
        "Expected the transaction to match the created transaction"
    );
    let tx_request = serde_json::json!({
            "signature": "308187020100301306072a8648ce3d020106082a8648ce3d030107046d306b02010104203c1636a384b67f7099a5923d22af174427cd910e600cb3245edb90d8f95bcb66a14403420004e461420ee838d41b5a03d91b5dc70f79e84e85ad56e3214290bcd260e9a9cfa8ec593d1d9a478c0d72e861ac284022ab403cb131b2b09d6750f72bbda6f845e1",
        });

    let response_2 = test_app.sign_transaction(1, &tx_request).await;

    if response_2.status().as_u16() != 200 {
        let error_body = response_2.text().await.expect("Failed to read response body");
        
        eprintln!("Failed to sign transaction. Error: {}", error_body);
        
        // Fail the test with a meaningful message
        panic!("Failed to sign");
    }
    

    assert_eq!(response_2.status().as_u16(), 200, "Failed to sign transaction");
    let tx_signed_response = response_2
        .json::<GetTransactionResponse>()
        .await
        .expect("Failed to deserialize the response");

    assert!(transaction_response_1.transaction.amount == tx_signed_response.transaction.amount,
        "Expected the signed transaction to match the original transaction");
    
    assert!(tx_signed_response.transaction.tx_status == "transaction signed",
        "Expected the tx_status to be 'transaction signed'");

    assert!(tx_signed_response.transaction.signature.is_some(),
        "Expected the signature to be present in the response")

}

#[tokio::test]
async fn should_return_404_when_try_to_sign_non_existing_transaction() {
    let test_app = TestApp::new().await;

    let tx_request = serde_json::json!({
            "signature": "308187020100301306072a8648ce3d020106082a8648ce3d030107046d306b02010104203c1636a384b67f7099a5923d22af174427cd910e600cb3245edb90d8f95bcb66a14403420004e461420ee838d41b5a03d91b5dc70f79e84e85ad56e3214290bcd260e9a9cfa8ec593d1d9a478c0d72e861ac284022ab403cb131b2b09d6750f72bbda6f845e1"
    });

    let response = test_app.sign_transaction(1000, &tx_request).await;

    assert_eq!(response.status().as_u16(), 404, "Expected 404 for non-existing transaction");
}


#[tokio::test]
async fn should_return_400_when_signature_is_invalid() {
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
        assert_eq!(response.status().as_u16(), 201, "Failed to create transactions")
    }

    let response_1 = test_app.get_one_transaction_by_index(1).await;

    assert_eq!(response_1.status().as_u16(), 200, "Failed to get transaction");

    let tx_request = serde_json::json!({
        "signature": "6082a8648ce3d030107046d306b02010104203c1636a384b67f7099a5923d22af174427cd910e600cb3245edb90d8f95bcb66a14403420004e461420ee838d41b5a03d91b5dc70f79e84e85ad56e3214290bcd260e9a9cfa8ec593d1d9a478c0d72e861ac284022ab403cb131b2b09d6750f72bbda6f845e1",
    });

    let response_2 = test_app.sign_transaction(1, &tx_request).await;

    assert_eq!(response_2.status().as_u16(), 400, "Invalid signature");
}
