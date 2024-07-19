use actix_web::{http::header::ContentType, HttpResponse};

pub async fn items() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("<h1>Items</h1>")
}
