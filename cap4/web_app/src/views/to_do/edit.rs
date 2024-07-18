use actix_web::{web, HttpResponse};

use crate::{
    json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems},
    process::process_input,
    state::read_file,
    to_do::{enums::TaskStatus, to_do_factory},
};

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state = read_file("./state.json");
    let status: TaskStatus;

    match &state.get(&to_do_item.title) {
        Some(result) => {
            status = TaskStatus::from_string(&result.as_str().unwrap().to_string());
        }

        None => {
            return HttpResponse::NotFound().json(format!("{} not in state", &to_do_item.title));
        }
    }

    let existing_item = to_do_factory(to_do_item.title.as_str(), status.clone());

    if &status.stringify()
        == &TaskStatus::from_string(&to_do_item.status.as_str().to_string()).stringify()
    {
        return HttpResponse::Ok().json(ToDoItems::get_state());
    }

    process_input(existing_item, "edit".to_owned(), &state);

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
