use crate::helper::TestApp;


#[tokio::test]
async fn test_health_check() {
    let test_app = TestApp::new().await;
    let response = test_app.health_check().await;

    assert_eq!(response.status().as_u16(), 200, "Failed Health Check");
}