#[macro_use]
extern crate rocket;

extern crate openssl;

use context::AppContext;

pub mod context;
pub mod db {
    pub mod schema;
    pub mod models {
        pub mod post;
        pub mod author;
    }
}
pub mod error;
pub mod resources;
pub mod services {
    pub mod postgres;
    pub mod redis;
    pub mod crypto;
}

#[launch]
fn rocket() -> _ {
    let app = rocket::build()
        .manage(AppContext::setup().expect("Could not setup the application context"));
    let app = resources::setup(app);
    app
}
