use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zero_to_prod::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 'init' calls 'set_logger', so this is all we need to do
    // We are falling back to printig all logs at info-level or above
    // if the RUST_LOG environment variable has not been set.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read configuration.yml");
    let connection = PgPool::connect(&configuration.database.get_connection_string())
        .await
        .expect("Failed to connect to postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
