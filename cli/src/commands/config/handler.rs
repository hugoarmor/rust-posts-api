use crate::services::auth;

pub fn handle_config_command(token: String) -> Result<(), anyhow::Error> {
  auth::store_token(token)?;

  Ok(())
}
