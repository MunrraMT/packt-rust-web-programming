use crate::to_do;

#[derive(serde::Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<to_do::structs::base::Base>,
    pub done_items: Vec<to_do::structs::base::Base>,
    pub pending_item_count: u8,
    pub done_item_count: u8,
}
