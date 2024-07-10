use rocket::get;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
pub fn index_route() -> Template {
    let mut context = HashMap::new();
    context.insert("greeting", "Hello, Rocket!");
    Template::render("index", &context)
}
