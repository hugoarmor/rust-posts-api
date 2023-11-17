use rocket::{http::Status, response::Responder};

#[derive(Debug)]
pub struct ApiError(pub Status, pub String);

impl ApiError {
    pub fn from_with_status<E: std::error::Error>(status: Status) -> impl FnOnce(E) -> Self {
        move |err: E| Self(status, err.to_string())
    }
}

impl<E: std::error::Error> From<E> for ApiError {
    fn from(err: E) -> Self {
        Self(Status::UnprocessableEntity, err.to_string())
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, _: &rocket::Request) -> rocket::response::Result<'static> {
        let ApiError(status, message) = self;

        rocket::Response::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .streamed_body(std::io::Cursor::new(message))
            .ok()
    }
}
