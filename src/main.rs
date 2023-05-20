use self::db::{create_new_auth, get_hashed_value_by_id};
use self::encrypt::{sha_256, HashResult};
use actix_web::{error, web, App, HttpResponse, HttpServer};

use encrypt::check_sha_256;
use serde::Deserialize;

mod db;
mod encrypt;

#[derive(Deserialize)]
struct ValueIdPair {
    value: String,
    id: String,
}

async fn encrypt(encrypt_data: web::Json<ValueIdPair>) -> HttpResponse {
    let value_to_encrypt = &encrypt_data.value;
    let HashResult {
        salt,
        hash,
        encryption_algorithm,
    } = sha_256(value_to_encrypt);

    let res = create_new_auth(&encrypt_data.id, &salt, &hash, &encryption_algorithm);
    match res {
        Ok(_) => HttpResponse::Accepted().body("Success! Value has been stored"),
        Err(_) => HttpResponse::InternalServerError().body("The provided ID already exists"),
    }
}

async fn check(data_to_check: web::Json<ValueIdPair>) -> HttpResponse {
    let passed_value = &data_to_check.value;
    let passed_id = &data_to_check.id;

    let Ok(row) = get_hashed_value_by_id(passed_id) else { return HttpResponse::InternalServerError().body("Failed to find record with provided id") };

    let mut salt = row.salt;
    let Ok(_) = check_sha_256(&mut salt, passed_value, &row.hashed_value) else { return HttpResponse::Unauthorized().body("Value does not match") };

    HttpResponse::Accepted().body("Success!")
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

        App::new()
            .app_data(json_config)
            .route("/encrypt", web::post().to(encrypt))
            .route("/check", web::post().to(check))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
