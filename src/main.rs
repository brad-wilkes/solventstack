mod routes;

#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate dotenv;

use rocket_dyn_templates::Template;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let connection = establish_connection();
    // Your Rocket setup and route mounting here
}
