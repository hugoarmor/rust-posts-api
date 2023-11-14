use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::context::AppState;
use crate::db::models::post::*;
use crate::db::schema;

#[get("/posts")]
pub fn get_posts(app: &AppState) -> Result<Json<Vec<Post>>, (Status, String)> {
    use self::schema::posts::dsl::*;

    let cached_value = app.redis.get("posts");

    if let Ok(value) = cached_value {
        let result: Vec<Post> = serde_json::from_str(&value).map_err(|err| {
            (
                Status::InternalServerError,
                format!(
                    "Could not serialize cached response. Error: {}",
                    err.to_string()
                ),
            )
        })?;

        return Ok(Json::from(result));
    }

    let results = app.with_db(|connection| {
        posts
            .limit(5)
            .select(Post::as_select())
            .load(connection)
            .map_err(|err| (Status::BadRequest, err.to_string()))
    })?;

    let serialized = serde_json::to_string(&results).map_err(|err| {
        (
            Status::InternalServerError,
            format!(
                "Could not serialize cached response. Error: {}",
                err.to_string()
            ),
        )
    })?;
    app.redis.set("posts", &serialized).map_err(|err| {
        (
            Status::InternalServerError,
            format!("Could not cache response. Error {}", err.to_string()),
        )
    })?;

    Ok(Json::from(results))
}

#[get("/posts/<post_id>")]
pub fn get_post(app: &AppState, post_id: i32) -> Result<Json<Post>, (Status, String)> {
    use self::schema::posts::dsl::*;

    let result = app.with_db(|connection| {
        posts
            .filter(id.eq(post_id))
            .select(Post::as_select())
            .first::<Post>(connection)
            .map_err(|err| (Status::NotFound, err.to_string()))
    })?;

    Ok(Json::from(result))
}

#[post("/posts", data = "<new_post>")]
pub fn create_post(
    app: &AppState,
    new_post: Json<NewPost>,
) -> Result<Json<Post>, (Status, String)> {
    use schema::posts;

    let result = app.with_db(|connection| {
        diesel::insert_into(posts::table)
            .values(new_post.into_inner())
            .returning(Post::as_returning())
            .get_result(connection)
            .map_err(|err| (Status::BadRequest, err.to_string()))
    })?;

    app.redis.delete("posts").map_err(|err| {
        (
            Status::InternalServerError,
            format!("Could not clear cache. Error {}", err.to_string()),
        )
    })?;

    Ok(Json::from(result))
}

#[put("/posts/<post_id>", data = "<updated_post>")]
pub fn update_post(
    app: &AppState,
    post_id: i32,
    updated_post: Json<NewPost>,
) -> Result<Json<Post>, (Status, String)> {
    use schema::posts::dsl::*;

    let result = app.with_db(|connection| {
        posts
            .filter(id.eq(post_id))
            .select(Post::as_select())
            .first::<Post>(connection)
            .map_err(|err| (Status::UnprocessableEntity, err.to_string()))?;

        diesel::update(posts.filter(id.eq(post_id)))
            .set(updated_post.into_inner())
            .returning(Post::as_returning())
            .get_result(connection)
            .map_err(|err| (Status::BadRequest, err.to_string()))
    })?;

    app.redis.delete("posts").map_err(|err| {
        (
            Status::InternalServerError,
            format!("Could not clear cache. Error {}", err.to_string()),
        )
    })?;

    Ok(Json::from(result))
}

#[delete("/posts/<post_id>")]
pub fn delete_post(app: &AppState, post_id: i32) -> Result<Json<Post>, (Status, String)> {
    use schema::posts::dsl::*;

    let result = app.with_db(|connection| {
        posts
            .filter(id.eq(post_id))
            .select(Post::as_select())
            .first::<Post>(connection)
            .map_err(|err| (Status::NotFound, err.to_string()))?;

        diesel::delete(posts.filter(id.eq(post_id)))
            .returning(Post::as_returning())
            .get_result(connection)
            .map_err(|err| (Status::BadRequest, err.to_string()))
    })?;

    app.redis.delete("posts").map_err(|err| {
        (
            Status::InternalServerError,
            format!("Could not clear cache. Error {}", err.to_string()),
        )
    })?;

    Ok(Json::from(result))
}
