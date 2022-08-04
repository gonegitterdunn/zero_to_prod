use crate::{
    configuration::{DatabaseSettings, Settings},
    email_client::EmailClient,
};

use super::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, web::Data, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address");

        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
        );

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool, email_client)?;

        // we 'save' the bound port in one of Application's fields
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub fn run(
    listener: TcpListener,
    pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    // actix-web runtime spins up a new worker process for each available core.
    // each worker runs its own copy of the app built by HttpServer.
    // -> db connection must therefore be cloneable (to be useable by each instance)
    // -> web::Data<T> wraps <T> in an ARC (allowing cloneability)
    //      the wrapped object is accessed through a cloned instance being passed to the .app_data() method on an App instance.
    //
    // web::Data is an extractor -> extracts a PgConnection from the type map that actix-web uses to represent its app data
    // -> sort of DEPENDENCY INJECTION

    // capture connection with 'move' from the surrounding environment to pass to app_data
    let connection = Data::new(pool);
    let email_client = Data::new(email_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // get a pointer copy of the connection and attach it to the app state
            .app_data(connection.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
