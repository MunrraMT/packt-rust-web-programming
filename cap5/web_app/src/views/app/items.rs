use actix_web::{http::header::ContentType, HttpResponse};

use super::content_loader::read_file;

pub async fn items() -> HttpResponse {
    let html_data = read_file("./templates/main.html");

    return HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html_data);
}
