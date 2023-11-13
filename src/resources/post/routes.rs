use diesel::prelude::*;

use crate::schema;
use crate::models::*;
use crate::establish_connection;

#[get("/posts")]
pub fn get_posts() -> String {
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
pub fn get_post(post_id: i32) -> String {
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
