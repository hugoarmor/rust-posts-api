extern crate clap;

use clap::Parser;

pub mod commands;
pub mod services {
    pub mod auth;
}

use commands::{
    config::handler::handle_config_command, post::handler::handle_post_command, Cli, Command,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match cli.command {
        Command::PostCommand(command) => handle_post_command(&command).await?,
        Command::ConfigCommand { token } => handle_config_command(token)?,
    }

    Ok(())
}
