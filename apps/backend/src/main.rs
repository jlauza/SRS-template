use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use dotenvy::dotenv;
use std::env;

#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Backend is alive!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or("8080".into());

    println!("🚀 Running backend on http://localhost:{}", port);

    HttpServer::new(|| App::new().service(health))
        .bind(("0.0.0.0", port.parse().unwrap()))?
        .run()
        .await
}
