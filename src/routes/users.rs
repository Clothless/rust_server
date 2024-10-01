// Get, Post, Put, Delete users

use actix_web::web;
use crate::handlers::user_handlers;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("add", web::post().to(user_handlers::create_user))
            .route("", web::get().to(user_handlers::get_all_users))
            .route("/{id}", web::get().to(user_handlers::getUserById))
            .route("/delete/{id}", web::delete().to(user_handlers::deleteUserById))
            // Add other user-related routes here
    );
}