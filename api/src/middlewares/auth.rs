use diesel::prelude::*;
use rocket::{
    async_trait,
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::{context::AppContext, services::crypto::IntoEncrypted};

use crate::models::author::Author;

pub struct AuthMiddleware {
    pub author: Author,
}

#[async_trait]
impl<'r> FromRequest<'r> for AuthMiddleware {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        use crate::models::schema::authors::dsl::*;

        let ctx = request.rocket().state::<AppContext>().unwrap();

        let req_token = match request.headers().get_one("Authorization") {
            Some(req_token) => Some(req_token.to_string()),
            None => None,
        };

        if let None = req_token {
            return Outcome::Error((Status::Unauthorized, "Unauthorized".to_string()));
        }

        let req_token = req_token.unwrap().into_encrypted(true);
        let req_token_serialized = serde_json::to_string(&req_token).unwrap();

        let author: Option<Author> = ctx.db.with_connection(|connection| {
            authors
                .filter(token.eq(req_token_serialized))
                .select(Author::as_select())
                .first::<Author>(connection)
                .ok()
        });

        match author {
            Some(author) => Outcome::Success(AuthMiddleware { author }),
            None => Outcome::Error((Status::Unauthorized, "Unauthorized".to_string())),
        }
    }
}
