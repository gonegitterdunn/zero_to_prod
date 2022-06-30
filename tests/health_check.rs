use std::net::TcpListener;

#[tokio::test]
async fn test_health_check() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    // entirely decoupled from implementation details
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero_to_prod::run(listener).expect("Failed to bind address");
    // Launch the server as a background test_health_check
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
