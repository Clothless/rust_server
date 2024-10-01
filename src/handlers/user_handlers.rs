use actix_web::{web, HttpResponse, Responder};
use diesel::sql_types::Json;
use crate::config::users_table::delete_user_by_id;
use crate::config::users_table::get_user_by_id;
use crate::models::user_model::IncomingUser;
use crate::config::users_table::insert_user;
use crate::config::users_table::get_users;

pub async fn create_user(body: String) -> impl Responder {
    // convert Json to IncomingUser
    let user: IncomingUser = serde_json::from_str(&body).unwrap();
    
    match insert_user(
        user.name,
        user.email,
        user.password,
    ) {
        Ok(_) => HttpResponse::Ok().json("User created successfully"),
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);
            HttpResponse::InternalServerError().json(format!("Couldn't create user: {}", e))
        }
    }
}

// get Users list
pub async fn get_all_users() -> impl Responder {
    match get_users() {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to get users: {}", e)),
    }
}

// get user by id
pub async fn getUserById(id: web::Path<i32>) -> impl Responder {
    match get_user_by_id(id.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to get user: {}", e)),
    }
}

// delete user by id
pub async fn deleteUserById(id: web::Path<i32>) -> impl Responder {
    match delete_user_by_id(id.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("User deleted successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to delete user: {}", e)),
    }
}