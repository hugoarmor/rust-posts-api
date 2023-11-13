use diesel::prelude::*;
use rocket::serde::json::Json;

use crate::context::AppState;
use crate::models::*;
use crate::schema;

#[get("/posts")]
pub fn get_posts(app: &AppState) -> Json<Vec<Post>> {
    use self::schema::posts::dsl::*;

    let results = app.with_db(|connection| {
        posts
            .limit(5)
            .select(Post::as_select())
            .load(connection)
            .expect("Error loading posts")
    });

    Json::from(results)
}

#[get("/posts/<post_id>")]
pub fn get_post(app: &AppState, post_id: i32) -> Json<Post> {
    use self::schema::posts::dsl::*;

    let result = app.with_db(|connection| {
        posts
            .filter(id.eq(post_id))
            .select(Post::as_select())
            .first::<Post>(connection)
            .expect("Error loading post")
    });

    Json::from(result)
}

#[post("/posts", data = "<new_post>")]
pub fn create_post(app: &AppState, new_post: Json<NewPost>) -> Json<Post> {
    use crate::schema::posts;

    let result = app.with_db(|connection| {
        diesel::insert_into(posts::table)
            .values(new_post.into_inner())
            .returning(Post::as_returning())
            .get_result(connection)
            .expect("Error saving new post")
    });

    Json::from(result)
}

#[put("/posts/<post_id>", data = "<updated_post>")]
pub fn update_post(app: &AppState, post_id: i32, updated_post: Json<NewPost>) -> Json<Post> {
    use crate::schema::posts::dsl::*;

    let result = app.with_db(|connection| {
        posts
            .filter(id.eq(post_id))
            .select(Post::as_select())
            .first::<Post>(connection)
            .expect(&format!("Post with id {} not found", post_id));

        diesel::update(posts.filter(id.eq(post_id)))
            .set(updated_post.into_inner())
            .returning(Post::as_returning())
            .get_result(connection)
            .expect("Error updating post")
    });

    Json::from(result)
}

#[delete("/posts/<post_id>")]
pub fn delete_post(app: &AppState, post_id: i32) -> Json<Post> {
    use crate::schema::posts::dsl::*;

    let result = app.with_db(|connection| {
        posts
            .filter(id.eq(post_id))
            .select(Post::as_select())
            .first::<Post>(connection)
            .expect(&format!("Post with id {} not found", post_id));

        diesel::delete(posts.filter(id.eq(post_id)))
            .returning(Post::as_returning())
            .get_result(connection)
            .expect("Error deleting post")
    });

    Json::from(result)
}
