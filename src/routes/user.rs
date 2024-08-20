use rocket::form::Form;
use rocket::response::Redirect;
use diesel::prelude::*;
use rocket::{get, post, FromForm};
use crate::models::user::{users, NewUser, User};
use crate::establish_connection;

#[get("/user")]
pub fn user_route() -> &'static str {
    "Hello, User!"
}

#[derive(FromForm)]
pub struct NewUserData {
    username: String,
    email: String,
    password_hash: String,
}

#[post("/user", data = "<new_user>")]
pub fn create_user(new_user: Form<NewUserData>) -> Redirect {
    let connection = &mut establish_connection();
    let new_user = NewUser {
        username: &new_user.username,
        email: &new_user.email,
        password_hash: &new_user.password_hash,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new user");

    Redirect::to("/user")
}