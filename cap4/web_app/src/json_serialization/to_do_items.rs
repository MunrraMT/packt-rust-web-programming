use crate::to_do;

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
}
