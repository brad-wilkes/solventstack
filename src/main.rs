#[macro_use]
extern crate diesel;
extern crate dotenv;

use rocket_dyn_templates::Template;
use dotenv::dotenv;
use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::{launch, routes};
use tera::Tera;

mod routes;
mod models;
mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let _connection = establish_connection();
    println!("Starting Rocket server...");

    let tera = match Tera::new("templates/*.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Error parsing templates: {}", e);
            std::process::exit(1);
        }
    };

    rocket::build()
        .manage(tera)
        .attach(Template::fairing())
        .mount("/", routes::get_routes())
        .mount("/api", routes![routes::user::create_user])
}