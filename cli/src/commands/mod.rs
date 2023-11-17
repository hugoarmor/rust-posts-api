use clap::{Parser, Subcommand};
use models::post::NewPost;

pub mod post {
    pub mod handler;
}

pub mod config {
    pub mod handler;
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum PostCommand {
    Get { id: u32 },
    GetAll,
    Create(NewPost),
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(subcommand)]
    Post(PostCommand),
    #[command()]
    Config { token: String },
}
