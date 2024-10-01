use diesel::prelude::*;
use diesel::{pg::PgConnection, sql_query};
use crate::models::user_model::{NewUser, User};

pub fn create_users_table(conn: &mut PgConnection) -> QueryResult<()> {
    sql_query(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL UNIQUE,
            password VARCHAR NOT NULL,
            enabled BOOLEAN NOT NULL DEFAULT TRUE,
            deleted BOOLEAN NOT NULL DEFAULT FALSE,
            created_at VARCHAR NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(conn)?;
    Ok(())
}

// Insert a new user into the users table
pub fn insert_user(
    user_name: String,
    user_email: String,
    user_password: String,
) -> QueryResult<usize> {
    use crate::schemas::users_schema::users::dsl::*;
    let conn = &mut crate::config::db::establish_connection();
    let new_user = NewUser {
        name: user_name,
        email: user_email,
        password: user_password,
        enabled: true,
        deleted: false,
    };
    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
}

// Get all users from the users table
pub fn get_users() -> QueryResult<Vec<User>> {
    use crate::schemas::users_schema::users::dsl::*;
    let conn = &mut crate::config::db::establish_connection();
    users.load::<User>(conn)
}

// get user by id
pub fn get_user_by_id(user_id: i32) -> QueryResult<User> {
    use crate::schemas::users_schema::users::dsl::*;
    let conn = &mut crate::config::db::establish_connection();
    users.find(user_id).first(conn)
}

// delete user by id
pub fn delete_user_by_id(user_id: i32) -> QueryResult<usize> {
    use crate::schemas::users_schema::users::dsl::*;
    let conn = &mut crate::config::db::establish_connection();
    diesel::delete(users.find(user_id)).execute(conn)
}