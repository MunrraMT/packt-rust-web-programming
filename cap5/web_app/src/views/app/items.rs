use actix_web::{http::header::ContentType, HttpResponse};

use super::content_loader::{add_component, read_file};

pub async fn items() -> HttpResponse {
    let js_data = read_file("./js/main.js");
    let css_data = read_file("./css/main.css");
    let css_base_data = read_file("./css/base.css");

    let mut html_data = read_file("./templates/main.html")
        .replace("// {{JAVASCRIPT}}", &js_data)
        .replace("/* {{BASE_CSS}} */", &css_data)
        .replace("/* {{CSS}} */", &css_base_data);

    html_data = add_component(String::from("header"), html_data);

    return HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html_data);
}
