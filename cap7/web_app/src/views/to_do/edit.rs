use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};

use crate::{database, json_serialization, jwt, schema};

pub async fn edit(
    to_do_item: actix_web::web::Json<json_serialization::to_do_item::ToDoItem>,
    token: jwt::JwtToken,
) -> actix_web::HttpResponse {
    dbg!(&token);
    println!("here is the message in the token: {}", &token.message);

    let connection = &mut database::establish_connection();
    let results = schema::to_do::table.filter(schema::to_do::columns::title.eq(&to_do_item.title));

    let _ = diesel::update(results)
        .set(schema::to_do::columns::status.eq("DONE"))
        .execute(connection);

    return actix_web::HttpResponse::Ok()
        .json(json_serialization::to_do_items::ToDoItems::get_state());
}
