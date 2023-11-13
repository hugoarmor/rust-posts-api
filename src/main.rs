#[macro_use]
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;
use models::*;

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

#[get("/posts")]
fn get_posts() -> String {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    serde_json::to_string(&results).unwrap()
}

#[post("/posts", data = "<new_post>")]
pub fn create_post(new_post: rocket::serde::json::Json<NewPost>) -> String {
    use crate::schema::posts;

    let connection = &mut establish_connection();

    let result = diesel::insert_into(posts::table)
        .values(new_post.into_inner())
        .returning(Post::as_returning())
        .get_result(connection)
        .expect("Error saving new post");

    serde_json::to_string(&result).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_posts])
}
