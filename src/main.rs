use std::net::TcpListener;

use zero_to_prod::run;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let address = format!("127.0.0.1:8080");
    run(TcpListener::bind(address).unwrap())?.await
}
