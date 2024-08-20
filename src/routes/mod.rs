pub mod index;
pub mod user;
pub mod debug;

pub use index::index_route;
pub use user::user_route;
pub use debug::debug_route;

use rocket::{Route, routes};

pub fn get_routes() -> Vec<Route> {
    routes![index_route, user_route, user::create_user, debug_route]
}
