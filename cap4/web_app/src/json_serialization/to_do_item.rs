#[derive(serde::Deserialize)]
pub struct ToDoItem {
    pub title: String,
    pub status: String,
}