use actix_web::HttpRequest;

use crate::{
    process::process_input,
    state::read_file,
    to_do::{enums::TaskStatus, to_do_factory},
};

pub async fn create(req: HttpRequest) -> String {
    let state = read_file("./state.json");
    let title = req.match_info().get("title").unwrap().to_string();
    let item = to_do_factory(&title.as_str(), TaskStatus::PENDING);

    process_input(item, "create".to_string(), &state);

    format!("{} created", title)
}
