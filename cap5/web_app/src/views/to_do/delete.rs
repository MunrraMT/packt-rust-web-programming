use crate::{json_serialization, jwt, process, state, to_do};

pub async fn delete(
    to_do_item: actix_web::web::Json<json_serialization::to_do_item::ToDoItem>,
    token: jwt::JwtToken,
) -> actix_web::HttpResponse {
    dbg!(token);

    let current_state = state::read_file("./state.json");

    let item_status_converted =
        match to_do::enums::TaskStatus::from_string(to_do_item.status.as_str().to_string()) {
            Ok(status_converted) => status_converted,
            Err(_message_error) => {
                return actix_web::HttpResponse::InternalServerError()
                    .json(format!("Something went wrong, try again later"));
            }
        };

    let item = to_do::to_do_factory(&to_do_item.title, item_status_converted);

    process::process_input(item, "delete".to_owned(), &current_state);

    return actix_web::HttpResponse::Ok()
        .json(json_serialization::to_do_items::ToDoItems::get_state());
}
