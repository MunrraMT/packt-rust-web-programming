use crate::{json_serialization, jwt, process, state, to_do};

pub async fn delete(req: actix_web::HttpRequest, token: jwt::JwtToken) -> actix_web::HttpResponse {
    dbg!(token);

    let current_state = state::read_file("./state.json");

    let title = match req.match_info().get("title") {
        Some(title_valid) => title_valid,
        None => {
            return actix_web::HttpResponse::BadRequest().json("title not found".to_string());
        }
    };

    let item_status = match &current_state.get(title) {
        Some(status_valid) => status_valid.as_str().unwrap_or("").to_string(),
        None => {
            return actix_web::HttpResponse::InternalServerError()
                .json(format!("Something went wrong, try again later"));
        }
    };

    let item_status_converted = match to_do::enums::TaskStatus::from_string(item_status) {
        Ok(status_converted) => status_converted,
        Err(_message_error) => {
            return actix_web::HttpResponse::InternalServerError()
                .json(format!("Something went wrong, try again later"));
        }
    };

    let item = to_do::to_do_factory(title, item_status_converted);

    process::process_input(item, "delete".to_owned(), &current_state);

    return actix_web::HttpResponse::Ok()
        .json(json_serialization::to_do_items::ToDoItems::get_state());
}
