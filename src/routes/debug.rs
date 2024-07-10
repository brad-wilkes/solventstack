use rocket::get;
use rocket::response::content::RawHtml;

#[get("/debug")]
pub fn debug_route() -> RawHtml<&'static str> {
    RawHtml(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Debug</title>
    </head>
    <body>
        <h1>This is a debug route</h1>
    </body>
    </html>
    "#)
}
