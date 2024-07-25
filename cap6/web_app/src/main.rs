use std::io;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{App, HttpServer};

mod database;
mod json_serialization;
mod jwt;
mod process;
mod schema;
mod state;
mod to_do;
mod views;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        let app = App::new()
            .wrap(cors)
            .wrap_fn(|req, srv| {
                println!("{:?}", req);
                let future = srv.call(req);
                async {
                    let result = future.await?;

                    Ok(result)
                }
            })
            .configure(views::views_factory);
        return app;
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
