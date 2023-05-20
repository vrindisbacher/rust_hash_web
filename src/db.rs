use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

use self::models::{NewAuth, AuthRow};

mod models;
mod schema;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_new_auth(
    id: &String,
    salt: &String,
    hashed_value: &String,
    encryption_algorithm: &String,
) -> QueryResult<usize> {
    let mut conn = establish_connection();
    let new_auth = NewAuth {
        id,
        salt,
        hashed_value,
        encryption_algorithm,
    };
    diesel::insert_into(schema::auth::table)
        .values(new_auth)
        .execute(&mut conn)
}

pub fn get_hashed_value_by_id(id: &String) -> QueryResult<AuthRow> {
    let mut conn = establish_connection();
    schema::auth::table
        .filter(schema::auth::id.eq(id))
        .first::<AuthRow>(&mut conn)
}
