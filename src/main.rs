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

#[get("/posts/<post_id>")]
fn get_post(post_id: i32) -> String {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let result = posts
        .filter(id.eq(post_id))
        .select(Post::as_select())
        .first::<Post>(connection)
        .expect("Error loading post");

    serde_json::to_string(&result).unwrap()
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

#[put("/posts/<post_id>", data = "<updated_post>")]
pub fn update_post(post_id: i32, updated_post: rocket::serde::json::Json<NewPost>) -> String {
    use crate::schema::posts::dsl::*;

    let existing = posts
        .filter(id.eq(post_id))
        .select(Post::as_select())
        .first::<Post>(&mut establish_connection());

    if existing.is_err() {
        return format!("Post with id {} not found", post_id);
    }

    let connection = &mut establish_connection();

    let result = diesel::update(posts.filter(id.eq(post_id)))
        .set(updated_post.into_inner())
        .returning(Post::as_returning())
        .get_result(connection)
        .expect("Error updating post");

    serde_json::to_string(&result).unwrap()
}

#[delete("/posts/<post_id>")]
pub fn delete_post(post_id: i32) -> String {
    use crate::schema::posts::dsl::*;

    let existing = posts
        .filter(id.eq(post_id))
        .select(Post::as_select())
        .first::<Post>(&mut establish_connection());

    if existing.is_err() {
        return format!("Post with id {} not found", post_id);
    }

    let connection = &mut establish_connection();

    let result = diesel::delete(posts.filter(id.eq(post_id)))
        .returning(Post::as_returning())
        .get_result(connection)
        .expect("Error deleting post");

    serde_json::to_string(&result).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_posts, create_post, delete_post, update_post, get_post])
}
