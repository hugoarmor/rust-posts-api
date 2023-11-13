use std::{
    env,
    sync::{Arc, Mutex},
};

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use rocket::State;

pub struct AppContext {
    pub db: Arc<Mutex<PgConnection>>,
}
pub type AppState = State<AppContext>;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            db: Arc::new(Mutex::new(establish_connection())),
        }
    }

    pub fn with_db<T, Callback: FnOnce(&mut PgConnection) -> T>(&self, callback: Callback) -> T {
        let connection = &mut *match self.db.lock() {
            Ok(connection) => connection,
            Err(poisoned) => poisoned.into_inner(),
        };
        callback(connection)
    }
}
