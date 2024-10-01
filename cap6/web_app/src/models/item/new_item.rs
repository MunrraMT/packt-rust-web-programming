use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::to_do)]
pub struct NewItem {
    pub title: String,
    pub status: String,
    pub date: chrono::NaiveDateTime,
}

impl NewItem {
    pub fn new(title: String) -> Self {
        return Self {
            title,
            status: "PENDING".to_string(),
            date: chrono::Utc::now().naive_local(),
        };
    }
}
