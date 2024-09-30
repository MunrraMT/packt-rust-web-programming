use super::content_loader;

pub async fn items() -> actix_web::HttpResponse {
    let html_data = content_loader::read_file("src/templates/main.html");
    return actix_web::HttpResponse::Ok()
        .content_type(actix_web::http::header::ContentType::html())
        .body(html_data);
}
