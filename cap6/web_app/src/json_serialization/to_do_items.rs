use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{database, models, schema, to_do};

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
        let mut array_buffer = Vec::new();
        let connection = &mut database::establish_connection();
        let items = schema::to_do::table
            .order(schema::to_do::columns::id.asc())
            .load::<models::item::item::Item>(connection)
            .unwrap();

        for item in items {
            let status = to_do::enums::TaskStatus::from_string(item.status.as_str().to_string());
            let item = to_do::to_do_factory(&item.title, status.unwrap());

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
