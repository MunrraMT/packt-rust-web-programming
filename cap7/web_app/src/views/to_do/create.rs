use crate::{diesel, models};
use diesel::prelude::*;

use crate::{database, json_serialization, jwt, schema};

pub async fn create(req: actix_web::HttpRequest, token: jwt::JwtToken) -> actix_web::HttpResponse {
    dbg!(&token);
    println!("here is the message in the token: {}", &token.message);

    let title = match req.match_info().get("title") {
        Some(title_valid) => title_valid.to_string(),
        None => {
            return actix_web::HttpResponse::BadRequest().json("title not found".to_string());
        }
    };
    let connection = &mut database::establish_connection();
    let items = schema::to_do::table
        .filter(schema::to_do::columns::title.eq(&title.as_str()))
        .order(schema::to_do::columns::id.asc())
        .load::<models::item::item::Item>(connection)
        .unwrap();

    if items.len() == 0 {
        let new_post = models::item::new_item::NewItem::new(title);
        let _ = diesel::insert_into(schema::to_do::table)
            .values(&new_post)
            .execute(connection);
    }

    return actix_web::HttpResponse::Ok()
        .json(json_serialization::to_do_items::ToDoItems::get_state());
}
