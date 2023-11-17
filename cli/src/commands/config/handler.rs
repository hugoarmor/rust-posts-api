use crate::{services::auth, config::AppConfig};

pub fn handle_config_command(_app_config: AppConfig, token: String) -> Result<(), anyhow::Error> {
  auth::store_token(token)?;

  Ok(())
}
