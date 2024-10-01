// config for database connection (Postgres) using r2d2 and diesel
extern crate diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
// use dotenv::dotenv;

pub fn establish_connection() -> PgConnection  {
    // Load the .env file
    dotenv::dotenv().ok();
    // Get the DATABASE_URL from the .env file
    let database_url = std::env::var("DB_URL").expect("DATABASE_URL must be set");
    println!("Connecting to {}", database_url);
    PgConnection::establish(&database_url)
        .unwrap()
}