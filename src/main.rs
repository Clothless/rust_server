use actix_web::{
    get, middleware::Logger, App, HttpResponse, HttpServer, Responder
};
use config::users_table::create_users_table;
mod config;
mod handlers;
mod models;
mod routes;
mod schemas;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // call the establish connection function from db.rs
    // call the create_users_table function from users_table.rs
    let _ = create_users_table(&mut config::db::establish_connection());
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .configure(routes::users::user_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
