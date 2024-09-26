use crate::{json_serialization, state, to_do};

pub async fn get() -> impl actix_web::Responder {
    let current_state = state::read_file("./state.json");
    let mut array_buffer = Vec::new();

    for (key, value) in &current_state {
        let current_status =
            to_do::enums::TaskStatus::from_string(value.as_str().unwrap().to_string());
        let item = to_do::to_do_factory(&key, current_status);
        array_buffer.push(item);
    }

    let return_package = json_serialization::to_do_items::ToDoItems::new(array_buffer);
    return actix_web::web::Json(return_package);
}
