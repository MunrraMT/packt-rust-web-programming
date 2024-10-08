use std;

use actix_web;

mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    return actix_web::HttpServer::new(|| {
        let app = actix_web::App::new().configure(views::views_factory);
        return app;
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await;
}
