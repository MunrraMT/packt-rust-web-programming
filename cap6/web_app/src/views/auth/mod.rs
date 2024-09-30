mod login;
mod logout;

pub fn auth_views(app: &mut actix_web::web::ServiceConfig) {
    app.service(
        actix_web::web::scope("v1/auth")
            .route("login", actix_web::web::get().to(login::login))
            .route("logout", actix_web::web::get().to(logout::logout)),
    );
}
