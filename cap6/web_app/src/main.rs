#[macro_use]
extern crate diesel;
extern crate dotenv;

use std;

use actix_service::Service;
use actix_web;

mod database;
mod json_serialization;
mod jwt;
mod process;
mod schema;
mod state;
mod to_do;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    return actix_web::HttpServer::new(|| {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        let app = actix_web::App::new()
            .wrap(cors)
            .wrap_fn(|req, srv| {
                println!("{:?}", req);
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    return Ok(result);
                }
            })
            .configure(views::views_factory);

        return app;
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await;
}
