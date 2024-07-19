use actix_web::web::ServiceConfig;

mod app;
mod auth;
mod to_do;

pub fn views_factory(app: &mut ServiceConfig) {
    auth::auth_view_factory(app);
    to_do::to_do_views_factory(app);
}
