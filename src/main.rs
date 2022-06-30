use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .route("/hello/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
