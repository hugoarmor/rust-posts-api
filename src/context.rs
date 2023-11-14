use anyhow::Result;
use dotenvy::dotenv;
use rocket::State;
use std::sync::Arc;

use crate::services::{postgres::PostgresService, redis::RedisService};

pub struct AppContext {
    pub db: PostgresService,
    pub redis: Arc<RedisService>,
}
pub type AppState = State<AppContext>;

impl AppContext {
    pub fn setup() -> Result<Self> {
        dotenv().ok();

        Ok(Self {
            db: PostgresService::connect()?,
            redis: Arc::new(RedisService::connect()?),
        })
    }
}
