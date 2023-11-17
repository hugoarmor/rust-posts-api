use anyhow::Result;
use dotenvy::dotenv;
use rocket::State;

use crate::services::{crypto::CryptoService, postgres::PostgresService, redis::RedisService};

pub struct AppContext {
    pub db: PostgresService,
    pub redis: RedisService,
    pub crypto: CryptoService,
}
pub type AppState = State<AppContext>;

impl AppContext {
    pub fn setup() -> Result<Self> {
        dotenv().ok();

        Ok(Self {
            db: PostgresService::connect()?,
            redis: RedisService::connect()?,
            crypto: CryptoService::new(),
        })
    }
}
