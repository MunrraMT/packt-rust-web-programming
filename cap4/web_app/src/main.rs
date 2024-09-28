use std;

use actix_service::Service;
use actix_web;

mod json_serialization;
mod process;
mod state;
mod to_do;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    return actix_web::HttpServer::new(|| {
        let app = actix_web::App::new()
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
