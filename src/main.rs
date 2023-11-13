#[macro_use]
extern crate rocket;
use context::AppContext;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

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

use resources::post::routes::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().manage(AppContext::new()).mount(
        "/",
        routes![
            index,
            get_posts,
            create_post,
            delete_post,
            update_post,
            get_post
        ],
    )
}
