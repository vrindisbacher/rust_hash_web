use crate::db::schema::auth;
use diesel::prelude::*;


#[derive(Queryable)]
pub struct AuthRow {
    pub id: String,
    pub salt: String,
    pub hashed_value: String,
    pub encryption_algorithm: String,
}

#[derive(Insertable)]
#[diesel(table_name = auth)]
pub struct NewAuth<'a> {
    pub id: &'a str,
    pub salt: &'a str,
    pub hashed_value: &'a str,
    pub encryption_algorithm: &'a str,
}
