use chrono::{NaiveDateTime, Utc};
use diesel::prelude::Insertable;

use crate::schema::to_do;
use crate::to_do::enums::TaskStatus;

#[derive(Insertable)]
#[diesel(table_name = to_do)]
pub struct NewItem {
    pub title: String,
    pub status: String,
    pub date: NaiveDateTime,
}

impl NewItem {
    pub fn new(title: String) -> Self {
        let now = Utc::now().naive_local();

        return Self {
            title,
            date: now,
            status: TaskStatus::PENDING.stringify(),
        };
    }
}
