use std::io;

use actix_web::{App, HttpServer};

mod views;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);
        return app;
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
