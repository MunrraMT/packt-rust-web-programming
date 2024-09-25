mod create;

pub fn to_do_views_factory(app: &mut actix_web::web::ServiceConfig) {
    app.service(
        actix_web::web::scope("v1/item")
            .route("create/{title}", actix_web::web::post().to(create::create)),
    );
}
