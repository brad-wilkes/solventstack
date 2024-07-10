extern crate dotenv;

use rocket_dyn_templates::Template;
use dotenv::dotenv;
use std::env;
use diesel::Connection;
use rocket::launch;

mod routes;

pub fn establish_connection() -> diesel::PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    diesel::PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let _connection = establish_connection();
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes::get_routes())
}
