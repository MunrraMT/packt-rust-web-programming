use crate::{json_serialization, jwt, process, state, to_do};

pub async fn edit(
    to_do_item: actix_web::web::Json<json_serialization::to_do_item::ToDoItem>,
    token: jwt::JwtToken,
) -> actix_web::HttpResponse {
    dbg!(&token);
    println!("here is the message in the token: {}", &token.message);

    let current_state = state::read_file("./state.json");

    let current_item_status = match current_state.get(&to_do_item.title) {
        Some(item_found) => item_found,
        None => {
            return actix_web::HttpResponse::NotFound()
                .json(format!("{} not in state", &to_do_item.title));
        }
    };

    let current_item_status_valid = match to_do::enums::TaskStatus::from_string(
        current_item_status.as_str().unwrap().to_string(),
    ) {
        Ok(status_valid) => status_valid,
        Err(_message_error) => {
            return actix_web::HttpResponse::InternalServerError()
                .json(format!("Something went wrong, try again later"));
        }
    };

    let existing_item = to_do::to_do_factory(&to_do_item.title, current_item_status_valid.clone());

    let new_item_status =
        match to_do::enums::TaskStatus::from_string(to_do_item.status.as_str().to_string()) {
            Ok(status_valid) => status_valid,
            Err(_message_error) => {
                return actix_web::HttpResponse::BadRequest()
                    .json(format!("status: {} not supported", &to_do_item.status));
            }
        };

    if current_item_status_valid.stringify() == new_item_status.stringify() {
        return actix_web::HttpResponse::Ok()
            .json(json_serialization::to_do_items::ToDoItems::get_state());
    }

    process::process_input(existing_item, "edit".to_owned(), &current_state);

    return actix_web::HttpResponse::Ok()
        .json(json_serialization::to_do_items::ToDoItems::get_state());
}
