#[macro_use]
extern crate rocket;
use context::AppContext;

pub mod context;
pub mod db {
    pub mod schema;
    pub mod models {
        pub mod post;
    }
}
pub mod resources {
    pub mod post {
        pub mod routes;
    }
}
pub mod error;
pub mod services {
    pub mod postgres;
    pub mod redis;
}

use resources::post::routes::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(AppContext::setup().expect("Could not setup the application context"))
        .mount(
            "/",
            routes![get_posts, create_post, delete_post, update_post, get_post],
        )
}
