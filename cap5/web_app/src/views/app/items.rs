use super::content_loader;

pub async fn items() -> actix_web::HttpResponse {
    let javascript_data = content_loader::read_file("src/javascript/main.js");
    let base_css_data = content_loader::read_file("src/css/base.css");
    let main_css_data = content_loader::read_file("src/css/main.css");
    let html_data = content_loader::read_file("src/templates/main.html")
        .replace("//{{JAVASCRIPT}}", &javascript_data)
        .replace("/*{{BASE_CSS}}*/", &base_css_data)
        .replace("/*{{CSS}}*/", &main_css_data);

    return actix_web::HttpResponse::Ok()
        .content_type(actix_web::http::header::ContentType::html())
        .body(html_data);
}
