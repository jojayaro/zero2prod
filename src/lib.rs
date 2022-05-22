use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result <Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8000))?
    .run();
    Ok(server)
}