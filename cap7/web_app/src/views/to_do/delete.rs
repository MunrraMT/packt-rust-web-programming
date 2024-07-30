use actix_web::{web, HttpResponse};
use diesel::{
    query_dsl::methods::{FilterDsl, OrderDsl},
    ExpressionMethods, RunQueryDsl,
};

use crate::{
    database::establish_connection,
    json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems},
    jwt::JwToken,
    models::item::item::Item,
    schema::to_do,
};

pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
    println!("here is the message in the token: {}", token.message);
    let connection = &mut establish_connection();
    let results: Vec<Item> = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title))
        .order(to_do::columns::id.asc())
        .load(connection)
        .unwrap();

    let _ = diesel::delete(&results[0]).execute(connection);

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
