use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use serde::Serialize;

use crate::{
    state::read_file,
    to_do::{enums::TaskStatus, structs::base::Base, to_do_factory, ItemTypes},
};

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_items_count: u8,
    pub done_items_count: u8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> Self {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_array_buffer.push(packed.super_struct),

                ItemTypes::Done(packed) => done_array_buffer.push(packed.super_struct),
            }
        }

        let done_count = done_array_buffer.len() as u8;
        let pending_count = pending_array_buffer.len() as u8;

        Self {
            pending_items: pending_array_buffer,
            done_items: done_array_buffer,
            pending_items_count: pending_count,
            done_items_count: done_count,
        }
    }

    pub fn get_state() -> Self {
        let state = read_file("./state.json");
        let mut array_buffer = Vec::new();

        for (key, value) in state {
            let status = TaskStatus::from_string(&value.as_str().unwrap().to_string());
            let item = to_do_factory(&key, status);

            array_buffer.push(item);
        }

        Self::new(array_buffer)
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
