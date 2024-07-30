use actix_web::{HttpRequest, HttpResponse};
use diesel::{
    query_dsl::methods::{FilterDsl, OrderDsl},
    ExpressionMethods, RunQueryDsl,
};

use crate::{
    database::establish_connection,
    json_serialization::to_do_items::ToDoItems,
    jwt::JwToken,
    models::item::{item::Item, new_item::NewItem},
    schema::to_do,
};

pub async fn create(req: HttpRequest, token: JwToken) -> HttpResponse {
    println!("here is the message in the token: {}", token.message);

    let title = req.match_info().get("title").unwrap().to_string();
    let connection = &mut establish_connection();
    let items: Vec<Item> = to_do::table
        .filter(to_do::columns::title.eq(&title.as_str()))
        .order(to_do::columns::id.asc())
        .load(connection)
        .unwrap();

    if items.len() == 0 {
        let new_item = NewItem::new(title);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_item)
            .execute(connection);
    }

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
