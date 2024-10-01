use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schemas::users_schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub enabled: bool,
    pub deleted: bool,
    pub created_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schemas::users_schema::users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub enabled: bool,
    pub deleted: bool,
}

#[derive(Deserialize)]
pub struct IncomingUser {
    pub name: String,
    pub email: String,
    pub password: String,
}