use crate::helpers::spawn_app;

#[tokio::test]
async fn test_health_check() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let request = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(request.status().is_success());
    assert_eq!(Some(0), request.content_length());
}
