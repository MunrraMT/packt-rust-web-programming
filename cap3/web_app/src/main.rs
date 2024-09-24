use std::io;

use actix_web;

async fn greet(req: actix_web::HttpRequest) -> impl actix_web::Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {name}")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/", actix_web::web::get().to(greet))
            .route("/{name}", actix_web::web::get().to(greet))
            .route(
                "/say/hello",
                actix_web::web::get().to(|| async { "Hello Again!" }),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
