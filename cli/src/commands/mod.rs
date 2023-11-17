use clap::{Parser, Subcommand};
use models::post::NewPost;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum PostCommand {
    GetCommand { id: u32 },
    GetAllCommand,
    CreateCommand(NewPost),
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(subcommand)]
    PostCommand(PostCommand),
    #[command()]
    ConfigCommand { token: String },
}
