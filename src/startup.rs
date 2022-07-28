use super::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    // actix-web runtime spins up a new worker process for each available core.
    // each worker runs its own copy of the app built by HttpServer.
    // -> db connection must therefore be cloneable (to be useable by each instance)
    // -> web::Data<T> wraps <T> in an ARC (allowing cloneability)
    //      the wrapped object is accessed through a cloned instance being passed to the .app_data() method on an App instance.
    //
    // web::Data is an extractor -> extracts a PgConnection from the type map that actix-web uses to represent its app data
    // -> sort of DEPENDENCY INJECTION

    // capture connection with 'move' from the surrounding environment to pass to app_data
    let connection = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // get a pointer copy of the connection and attach it to the app state
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
