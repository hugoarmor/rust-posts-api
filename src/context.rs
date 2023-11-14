use anyhow::Result;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use redis::Connection as RedisConnection;
use rocket::State;
use std::{
    env,
    sync::{Arc, Mutex},
};

use crate::services::redis::get_redis_conenction;

pub struct AppContext {
    pub db: Arc<Mutex<PgConnection>>,
    pub redis: Arc<Mutex<RedisConnection>>,
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

        let redis_conn = get_redis_conenction()?;

        Ok(Self {
            db: Arc::new(Mutex::new(conn)),
            redis: Arc::new(Mutex::new(redis_conn)),
        })
    }

    pub fn with_db<T, Callback: FnOnce(&mut PgConnection) -> T>(&self, callback: Callback) -> T {
        let connection = &mut *match self.db.lock() {
            Ok(connection) => connection,
            Err(poisoned) => poisoned.into_inner(),
        };
        callback(connection)
    }

    pub fn with_cache<T, Callback: FnOnce(&mut redis::Connection) -> T>(&self, callback: Callback) -> T {
        let conenction = &mut *match self.redis.lock() {
            Ok(client) => client,
            Err(poisoned) => poisoned.into_inner(),
        };

        callback(conenction)
    }
}
