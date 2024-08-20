use rocket::get;
use rocket::post;
use diesel::prelude::*;
use crate::models::user::{NewUser, User};
use crate::establish_connection;


#[get("/user")]
pub fn user_route() -> &'static str {
    "Hello, User!"
}
