use redis::Connection;

pub fn get_redis_conenction() -> Result<Connection, redis::RedisError> {
  let client = redis::Client::open("redis://127.0.0.1/")?;

  let connection = client.get_connection()?;

  Ok(connection)
}
