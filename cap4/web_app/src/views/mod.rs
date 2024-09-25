mod auth;
mod to_do;

pub fn views_factory(app: &mut actix_web::web::ServiceConfig) {
    auth::auth_views(app);
    to_do::to_do_views_factory(app);
}
