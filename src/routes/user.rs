use rocket::get;

#[get("/user")]
pub fn user_route() -> &'static str {
    "Hello, User!"
}
