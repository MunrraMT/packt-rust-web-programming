mod auth;

pub fn views_factory(app: &mut actix_web::web::ServiceConfig) {
    auth::auth_views(app);
}
