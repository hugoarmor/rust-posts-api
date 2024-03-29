use diesel::prelude::*;
use rocket::serde::json::Json;

use crate::context::AppState;
use crate::error::ApiError;
use crate::middlewares::auth::AuthMiddleware;
use crate::models::author::Author;
use crate::models::post::*;
use crate::models::schema;

#[get("/")]
pub fn get_posts(app: &AppState) -> Result<Json<Vec<Post>>, ApiError> {
    use self::schema::posts::dsl::*;

    let cached_value = app.redis.get("posts");

    if let Ok(value) = cached_value {
        let result: Vec<Post> = serde_json::from_str(&value).map_err(ApiError::from)?;

        return Ok(Json::from(result));
    }

    let results = app.db.with_connection(|connection| {
        posts
            .limit(5)
            .select(Post::as_select())
            .load(connection)
            .map_err(ApiError::from)
    })?;

    let serialized = serde_json::to_string(&results).map_err(ApiError::from)?;
    app.redis
        .set("posts", &serialized)
        .map_err(ApiError::from)?;

    Ok(Json::from(results))
}

#[get("/<post_id>")]
pub fn get_post(app: &AppState, post_id: i32) -> Result<Json<Post>, ApiError> {
    use self::schema::posts::dsl::*;

    let result = app.db.with_connection(|connection| {
        posts
            .filter(id.eq(post_id))
            .select(Post::as_select())
            .first::<Post>(connection)
            .map_err(ApiError::from)
    })?;

    Ok(Json::from(result))
}

#[post("/", data = "<new_post>")]
pub fn create_post(app: &AppState, new_post: Json<NewPost>, _auth: AuthMiddleware,) -> Result<Json<Post>, ApiError> {
    use schema::posts;

    let result = app.db.with_connection(|connection| {
        diesel::insert_into(posts::table)
            .values(new_post.into_inner())
            .returning(Post::as_returning())
            .get_result(connection)
            .map_err(ApiError::from)
    })?;

    app.redis.delete("posts").map_err(ApiError::from)?;

    Ok(Json::from(result))
}

#[put("/<post_id>", data = "<updated_post>")]
pub fn update_post(
    app: &AppState,
    post_id: i32,
    updated_post: Json<NewPost>,
) -> Result<Json<Post>, ApiError> {
    use schema::posts::dsl::*;

    let result = app.db.with_connection(|connection| {
        posts
            .filter(id.eq(post_id))
            .select(Post::as_select())
            .first::<Post>(connection)
            .map_err(ApiError::from)?;

        diesel::update(posts.filter(id.eq(post_id)))
            .set(updated_post.into_inner())
            .returning(Post::as_returning())
            .get_result(connection)
            .map_err(ApiError::from)
    })?;

    app.redis.delete("posts").map_err(ApiError::from)?;

    Ok(Json::from(result))
}

#[delete("/<post_id>")]
pub fn delete_post(app: &AppState, post_id: i32) -> Result<Json<Post>, ApiError> {
    use schema::posts::dsl::*;

    let result = app.db.with_connection(|connection| {
        posts
            .filter(id.eq(post_id))
            .select(Post::as_select())
            .first::<Post>(connection)
            .map_err(ApiError::from)?;

        diesel::delete(posts.filter(id.eq(post_id)))
            .returning(Post::as_returning())
            .get_result(connection)
            .map_err(ApiError::from)
    })?;

    app.redis.delete("posts").map_err(ApiError::from)?;

    Ok(Json::from(result))
}

#[post("/<post_id>/publish")]
pub fn publish_post(
    app: &AppState,
    auth: AuthMiddleware,
    post_id: i32,
) -> Result<Json<Post>, ApiError> {
    use schema::authors::dsl::{id as authors_id, *};
    use schema::posts::dsl::{id as posts_id, *};

    let result = app.db.with_connection(|connection| {
        let selected_author = authors
            .filter(authors_id.eq(auth.author.id))
            .select(Author::as_select())
            .first::<Author>(connection)
            .map_err(ApiError::from)?;

        diesel::update(posts.filter(posts_id.eq(post_id)))
            .set((
                published_at.eq(diesel::dsl::now),
                author_id.eq(selected_author.id),
            ))
            .returning(Post::as_returning())
            .get_result(connection)
            .map_err(ApiError::from)
    })?;

    app.redis.delete("posts").map_err(ApiError::from)?;

    Ok(Json::from(result))
}
