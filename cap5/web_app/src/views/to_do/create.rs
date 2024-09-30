use crate::{json_serialization, jwt, process, state, to_do};

pub async fn create(req: actix_web::HttpRequest, token: jwt::JwtToken) -> actix_web::HttpResponse {
    dbg!(&token);
    println!("here is the message in the token: {}", &token.message);

    let current_state = state::read_file("./state.json");

    let title = match req.match_info().get("title") {
        Some(title_valid) => title_valid,
        None => {
            return actix_web::HttpResponse::BadRequest().json("title not found".to_string());
        }
    };

    let item = to_do::to_do_factory(&title, to_do::enums::TaskStatus::PENDING);

    process::process_input(item, "create".to_string(), &current_state);

    return actix_web::HttpResponse::Ok()
        .json(json_serialization::to_do_items::ToDoItems::get_state());
}
