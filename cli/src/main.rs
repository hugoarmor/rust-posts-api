extern crate clap;

use clap::Parser;

pub mod commands;
pub mod services {
    pub mod auth;
}

pub mod config;

use commands::{
    config::handler::handle_config_command, Cli, Command, post::handler::handle_post_command,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let _app_config = config::AppConfig::setup();

    let cli = Cli::parse();

    match cli.command {
        Command::Post(command) => handle_post_command(_app_config, &command).await?,
        Command::Config { token } => handle_config_command(_app_config, token)?,
    }

    Ok(())
}
