use anyhow::Result;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use rocket::State;
use std::{
    env,
    sync::{Arc, Mutex},
};

use crate::services::redis::RedisService;

pub struct AppContext {
    pub db: Arc<Mutex<PgConnection>>,
    pub redis: Arc<RedisService>,
}
pub type AppState = State<AppContext>;

impl AppContext {
    pub fn setup() -> Result<Self> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")?;
        let conn = PgConnection::establish(&database_url).map_err(|err| {
            anyhow::anyhow!(
                "Unable to connect to the database. Error: {}",
                err.to_string()
            )
        })?;

        Ok(Self {
            db: Arc::new(Mutex::new(conn)),
            redis: Arc::new(RedisService::connect()?),
        })
    }

    pub fn with_db<T, Callback: FnOnce(&mut PgConnection) -> T>(&self, callback: Callback) -> T {
        let connection = &mut *match self.db.lock() {
            Ok(connection) => connection,
            Err(poisoned) => poisoned.into_inner(),
        };
        callback(connection)
    }
}
