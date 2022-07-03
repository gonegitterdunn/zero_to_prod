use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero_to_prod::{configuration::get_configuration, startup::run};

#[tokio::test]
async fn test_health_check() {
    let url = spawn_app();

    let client = reqwest::Client::new();

    let request = client
        .get(format!("{}/health_check", url))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(request.status().is_success());
    assert_eq!(Some(0), request.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let address = spawn_app();

    // Load configuration
    let configuration = get_configuration().expect("Failed to load configuration.yml");
    let connection_string = configuration.database.get_connection_string();

    // Connect to db
    // Must import sqlx::{PgConnection, Connection}
    // Connection is required for PgConnection and not an inherent method of the struct
    let mut connection = PgConnection::connect(connection_string.as_str())
        .await
        .expect("Failed to connect to Postgres");

    // Create new http client
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=`ursula_le_guin%40gmail.com";

    // Act
    let request = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, request.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le_guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let request = client
            .post(format!("{}/subscriptions", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            request.status().as_u16(),
            // additional, custom error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

// need to start the application in the background on a random port
// need the url to run the test request against
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
