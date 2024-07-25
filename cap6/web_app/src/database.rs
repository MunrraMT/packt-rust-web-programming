use std::env;

use crate::config::Config;
use diesel::{pg, Connection};
use dotenv::dotenv;

pub fn establish_connection() -> pg::PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    return pg::PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
}
