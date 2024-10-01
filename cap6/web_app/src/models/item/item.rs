use crate::schema;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = schema::to_do)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub date: chrono::NaiveDateTime,
}
