use actix_web::web::{post, scope, ServiceConfig};

mod create;

pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(scope("v1/item").route("create/{title}", post().to(create::create)));
}
