use std::env;

use dotenvy::dotenv;

use crate::services::auth;

pub struct AppConfig {
  pub token: String,
  pub api_url: String,
}

impl AppConfig {
  pub fn setup() -> Self {
    dotenv().ok();

    let token = auth::get_token().expect("You don't have a token configured.");
    let api_url = env::var("RUST_POSTS_API_URL").expect("RUST_POSTS_API_URL not found in environment");

    Self { token, api_url }
  }
}
