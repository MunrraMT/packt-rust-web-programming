use crate::json_serialization;

pub async fn get() -> impl actix_web::Responder {
    return json_serialization::to_do_items::ToDoItems::get_state();
}
