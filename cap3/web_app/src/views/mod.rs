use actix_web::web::ServiceConfig;
use auth::auth_view_factory;

mod auth;

pub fn views_factory(app: &mut ServiceConfig) {
    auth_view_factory(app);
}
