use diesel::prelude::*;
use rocket::serde::json::Json;

use crate::context::AppState;
use crate::error::ApiError;
use crate::models::author::*;
use crate::models::schema;

#[get("/")]
pub fn get_authors(app: &AppState) -> Result<Json<Vec<Author>>, ApiError> {
    use self::schema::authors::dsl::*;

    let cached_value = app.redis.get("authors");

    if let Ok(value) = cached_value {
        let result: Vec<Author> = serde_json::from_str(&value).map_err(ApiError::from)?;

        return Ok(Json::from(result));
    }

    let results = app.db.with_connection(|connection| {
        authors
            .select(Author::as_select())
            .load(connection)
            .map_err(ApiError::from)
    })?;

    let serialized = serde_json::to_string(&results).map_err(ApiError::from)?;
    app.redis
        .set("authors", &serialized)
        .map_err(ApiError::from)?;

    Ok(Json::from(results))
}

#[post("/", data = "<body>")]
pub fn create_author(
    app: &AppState,
    body: Json<NewAuthorRequestBody>,
) -> Result<Json<Author>, ApiError> {
    use schema::authors;

    let new_token = app.crypto.generate_random_token();
    let encrypted_token = app.crypto.encrypt(&new_token.as_bytes(), true);

    let new_author = NewAuthor {
        name: body.name.clone(),
        email: body.email.clone(),
        token: serde_json::to_string(&encrypted_token).map_err(ApiError::from)?,
    };

    let mut result = app.db.with_connection(|connection| {
        diesel::insert_into(authors::table)
            .values(new_author)
            .returning(Author::as_returning())
            .get_result(connection)
            .map_err(ApiError::from)
    })?;

    result.token = new_token;

    app.redis.delete("authors").map_err(ApiError::from)?;

    Ok(Json::from(result))
}
