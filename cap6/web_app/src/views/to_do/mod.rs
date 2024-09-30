mod create;
mod delete;
mod edit;
mod get;

pub fn to_do_views_factory(app: &mut actix_web::web::ServiceConfig) {
    app.service(
        actix_web::web::scope("v1/item")
            .route("create/{title}", actix_web::web::post().to(create::create))
            .route("get", actix_web::web::get().to(get::get))
            .route("edit", actix_web::web::post().to(edit::edit))
            .route("delete", actix_web::web::post().to(delete::delete)),
    );
}
