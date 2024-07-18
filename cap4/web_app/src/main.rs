use std::io;

use actix_service::Service;
use actix_web::{App, HttpServer};

mod json_serialization;
mod process;
mod state;
mod to_do;
mod views;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
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
