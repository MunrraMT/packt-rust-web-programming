use std::env;

use crate::config::Config;
use actix_web::{error::ErrorServiceUnavailable, Error, FromRequest, Result};
use diesel::{
    pg,
    r2d2::{ConnectionManager, Pool, PooledConnection},
    Connection, PgConnection,
};
use dotenv::dotenv;
use futures::future::{err, ok, Ready};
use lazy_static::lazy_static;

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection {
    pub db_connection: PgPool,
}

lazy_static! {
    pub static ref DBCONNECTION: DbConnection = {
        let connection_string = Config::new()
            .map
            .get("DB_URL")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        DbConnection {
            db_connection: PgPool::builder()
                .max_size(8)
                .build(ConnectionManager::new(connection_string))
                .expect("failed to create db connection_pool"),
        }
    };
}

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    return DBCONNECTION.db_connection.get().unwrap();
}

pub struct DB {
    pub connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl FromRequest for DB {
    type Error = Error;
    type Future = Ready<Result<DB, Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        match DBCONNECTION.db_connection.get() {
            Ok(connection) => {
                return ok(DB { connection });
            }
            Err(_) => {
                return err(ErrorServiceUnavailable(
                    "could not make connection to database",
                ));
            }
        }
    }
}
