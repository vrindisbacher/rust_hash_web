use self::encrypt::{sha_256, HashResult};
use self::models::NewAuth;
use actix_web::{error, web, App, HttpResponse, HttpServer, Result};

use diesel::prelude::*;
use diesel::sqlite::{SqliteConnection};
use dotenvy::dotenv;
use serde::Deserialize;
use std::env;

mod encrypt;
mod models;
mod schema;

#[derive(Deserialize)]
struct Encrypt {
    value: String,
    id: String,
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn create_new_auth(conn: &mut SqliteConnection, new_auth: NewAuth) -> QueryResult<usize> {
    diesel::insert_into(schema::auth::table)
        .values(&new_auth)
        .execute(conn)
}

async fn encrypt(encrypt_data: web::Json<Encrypt>) -> Result<String> {
    let value_to_encrypt = &encrypt_data.value;
    let HashResult {
        salt: salt_value,
        hash: hash_value,
    } = sha_256(value_to_encrypt);

    // insert to db now
    let connection = &mut establish_connection();

    let new_auth = NewAuth {
        id: &encrypt_data.id,
        salt: &salt_value,
        hashed_value: &hash_value,
        encryption_algorithm: "sha256",
    };

    let res = create_new_auth(connection, new_auth);
    match res {
        Ok(_) => Ok(format!("Success!")),
        Err(_) => Ok(format!("whoops"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new().service(
            web::resource("/encrypt")
                // change json extractor configuration
                .app_data(json_config)
                .route(web::post().to(encrypt)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
