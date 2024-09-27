use crate::{state, to_do};

#[derive(serde::Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<to_do::structs::base::Base>,
    pub done_items: Vec<to_do::structs::base::Base>,
    pub pending_item_count: u8,
    pub done_item_count: u8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<to_do::ItemTypes>) -> ToDoItems {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items {
            match item {
                to_do::ItemTypes::Pending(packed) => pending_array_buffer.push(packed.super_struct),
                to_do::ItemTypes::Done(packed) => done_array_buffer.push(packed.super_struct),
            }
        }

        let done_count = done_array_buffer.len() as u8;
        let pending_count = pending_array_buffer.len() as u8;

        return Self {
            pending_items: pending_array_buffer,
            done_items: done_array_buffer,
            pending_item_count: pending_count,
            done_item_count: done_count,
        };
    }

    pub fn get_state() -> Self {
        let current_state = state::read_file("./state.json");
        let mut array_buffer = Vec::new();

        for (key, value) in current_state {
            let current_status =
                to_do::enums::TaskStatus::from_string(value.as_str().unwrap().to_string());
            let item = to_do::to_do_factory(&key, current_status);

            array_buffer.push(item);
        }

        return Self::new(array_buffer);
    }
}

impl actix_web::Responder for ToDoItems {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match serde_json::to_string(&self) {
            Ok(body) => actix_web::HttpResponse::Ok()
                .content_type(actix_web::http::header::ContentType::json())
                .body(body),
            Err(err) => actix_web::HttpResponse::Ok()
                .content_type(actix_web::http::header::ContentType::json())
                .body(err.to_string()),
        }
    }
}
