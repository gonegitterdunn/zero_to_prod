use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero_to_prod::{
    configuration::get_configuration, startup::run, telemetry::get_subscriber,
    telemetry::init_subscriber, email_client::EmailClient,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero_to_prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.yml");

    let connection = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address");

    let email_client = EmailClient::new(configuration.email_client.base_url, sender_email);

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection, email_client)?.await
}
