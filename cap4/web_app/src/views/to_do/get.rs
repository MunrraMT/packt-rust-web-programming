use actix_web::{web, Responder};

use crate::{
    json_serialization::to_do_items::ToDoItems,
    state::read_file,
    to_do::{enums::TaskStatus, to_do_factory},
};

pub async fn get() -> impl Responder {
    let state = read_file("./state.json");
    let mut array_buffer = Vec::new();

    for (key, value) in state {
        let status = TaskStatus::from_string(&value.as_str().unwrap().to_string());
        let item = to_do_factory(&key, status);

        array_buffer.push(item);
    }

    let return_package = ToDoItems::new(array_buffer);

    web::Json(return_package)
}
