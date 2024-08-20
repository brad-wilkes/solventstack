use rocket::{get, State};
use rocket::response::content::RawHtml;
use tera::{Context, Tera};

#[get("/")]
pub fn index_route(tera: &State<Tera>) -> RawHtml<String> {
    let mut context = Context::new();
    context.insert("greeting", "Hello, Juppo!");
    context.insert("message", "beep boop beep!");
    // Render template
    let rendered = match tera.render("index.tera", &context) {
        Ok(r) => r,
        Err(e) => {
            println!("Error rendering template: {}", e);
            return RawHtml("Internal Server Error".to_string());
        }
    };

    println!("Rendered template: {}", rendered);
    RawHtml(rendered)
}
