use std::sync::Mutex;

use redis::Connection;

pub fn get_redis_conenction() -> Result<Connection, redis::RedisError> {
    let client = redis::Client::open("redis://127.0.0.1/")?;

    let connection = client.get_connection()?;

    Ok(connection)
}

pub struct RedisService {
    connection: Mutex<Connection>,
}

impl RedisService {
    pub fn connect() -> Result<Self, redis::RedisError> {
        let connection = Mutex::new(get_redis_conenction()?);

        Ok(Self { connection })
    }

    fn with_connection<T, Callback: FnOnce(&mut redis::Connection) -> T>(
        &self,
        callback: Callback,
    ) -> T {
        let conenction = &mut *match self.connection.lock() {
            Ok(client) => client,
            Err(poisoned) => poisoned.into_inner(),
        };

        callback(conenction)
    }

    pub fn get(&self, key: &str) -> Result<String, redis::RedisError> {
        let result = self.with_connection(|conn| redis::cmd("GET").arg(key).query(conn))?;

        Ok(result)
    }

    pub fn set(&self, key: &str, value: &str) -> Result<(), redis::RedisError> {
        self.with_connection(|conn| redis::cmd("SET").arg(key).arg(value).query(conn))?;

        Ok(())
    }

    pub fn delete(&self, key: &str) -> Result<(), redis::RedisError> {
        self.with_connection(|conn| redis::cmd("DEL").arg(key).query(conn))?;

        Ok(())
    }
}
