use super::items;

pub fn app_views_factory(app: &mut actix_web::web::ServiceConfig) {
    app.route("/", actix_web::web::get().to(items::items));
}
