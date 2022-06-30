#[tokio::test]
async fn test_health_check() {
    spawn_app();

    let client = reqwest::Client::new();

    // entirely decoupled from implementation details
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero_to_prod::run().expect("Failed to bind address");
    // Launch the server as a background test_health_check
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}
