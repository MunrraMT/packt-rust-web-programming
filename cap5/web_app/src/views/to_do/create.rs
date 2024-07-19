use actix_web::{HttpRequest, HttpResponse};

use crate::{
    json_serialization::to_do_items::ToDoItems,
    process::process_input,
    state::read_file,
    to_do::{enums::TaskStatus, to_do_factory},
};

pub async fn create(req: HttpRequest) -> HttpResponse {
    let state = read_file("./state.json");
    let title = req.match_info().get("title").unwrap().to_string();
    let item = to_do_factory(&title.as_str(), TaskStatus::PENDING);

    process_input(item, "create".to_string(), &state);

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
