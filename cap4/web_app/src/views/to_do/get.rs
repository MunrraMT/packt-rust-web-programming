use crate::{json_serialization, jwt};

pub async fn get(token: jwt::JwtToken) -> impl actix_web::Responder {
    dbg!(&token);
    println!("here is the message in the token: {}", &token.message);

    return json_serialization::to_do_items::ToDoItems::get_state();
}
