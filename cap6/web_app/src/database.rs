use diesel::Connection;

pub fn establish_connection() -> diesel::pg::PgConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    return diesel::pg::PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url));
}
