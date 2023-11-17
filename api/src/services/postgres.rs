use anyhow::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use std::env;
pub struct PostgresService {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresService {
    pub fn connect() -> Result<Self> {
        let database_url = env::var("DATABASE_URL")?;
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().test_on_check_out(true).build(manager)?;

        Ok(Self { pool })
    }

    pub fn with_connection<
        T,
        Callback: FnOnce(&mut PooledConnection<ConnectionManager<PgConnection>>) -> T,
    >(
        &self,
        callback: Callback,
    ) -> T {
        callback(&mut self.pool.get().unwrap())
    }
}
