use actix_web::{web, HttpResponse};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};

use crate::{
    database::DB,
    json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems},
    jwt::JwToken,
    schema::to_do,
    to_do::enums::TaskStatus,
};

pub async fn edit(to_do_item: web::Json<ToDoItem>, token: JwToken, db: DB) -> HttpResponse {
    println!("here is the message in the token: {}", token.message);

    let results = to_do::table.filter(to_do::columns::title.eq(&to_do_item.title));

    let _ = diesel::update(results)
        .set(to_do::columns::status.eq(TaskStatus::DONE.stringify()))
        .execute(&db.connection);

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
