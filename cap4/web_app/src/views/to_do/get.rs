use crate::state;

pub async fn get() -> impl actix_web::Responder {
    let current_state = state::read_file("./state.json");
    return actix_web::web::Json(current_state);
}
