use crate::{jwt, process, state, to_do};

pub async fn create(req: actix_web::HttpRequest, token: jwt::JwtToken) -> String {
    dbg!(&token);
    println!("here is the message in the token: {}", &token.message);

    let current_state = state::read_file("./state.json");

    match req.match_info().get("title") {
        Some(title_valid) => {
            let item = to_do::to_do_factory(&title_valid, to_do::enums::TaskStatus::PENDING);

            process::process_input(item, "create".to_string(), &current_state);

            return format!("{title_valid} created");
        }
        None => {
            return "title not found".to_string();
        }
    }
}
