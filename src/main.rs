use std::net::TcpListener;
use zero_to_prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = format!("127.0.0.1:8080");

    run(TcpListener::bind(address).unwrap())?.await
}
