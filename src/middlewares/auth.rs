use rocket::{
    async_trait,
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

pub struct AuthMiddleware {
    pub token: String,
}

#[async_trait]
impl<'r> FromRequest<'r> for AuthMiddleware {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            Some(token) => Outcome::Success(AuthMiddleware {
                token: token.to_string(),
            }),
            None => Outcome::Error((Status::Unauthorized, "Unauthorized".to_string())),
        }
    }
}
