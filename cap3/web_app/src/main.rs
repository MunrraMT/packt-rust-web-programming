use std::io;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

#[tokio::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("0.0.0.0:8080")?
    .bind("0.0.0.0:8081")?
    .workers(3)
    .run()
    .await
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");

    format!("hello {}!", name)
}
