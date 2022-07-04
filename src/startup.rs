use super::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    // HttpServer takes a closure that RETURNS an App
    // each actix-web worker runs its own copy of App
    // PgConnection needs therefore to be clonable, but it's not...
    // SOLUTION ==> wrap PgConnection on Arc<T> (web::Data<T>)
    //
    // web::Data is an extractor -> extracts a PgConnection from the type map
    // that actix-web uses to represent its app data
    // sort of DEPENDENCY INJECTION

    // capture connection with 'move' from the surrounding environment
    let connection = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // get a pointer copy of the connection and attach it to the app state
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
