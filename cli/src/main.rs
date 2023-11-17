extern crate clap;

use clap::*;
use serde::{Serialize, Deserialize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum PostCommand {
    Get { id: u32 },
    GetAll,
    Create(CreatePostArgs),
}

#[derive(Serialize, Deserialize, Debug, Args)]
struct CreatePostArgs {
    title: String,
    body: String,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(subcommand)]
    Post(PostCommand),
}

async fn handle_post_command(command: &PostCommand) -> Result<(), anyhow::Error> {
    match command {
        PostCommand::Get { id } => {
            println!("Get post with id {}", id);
            let response: models::post::Post = serde_json::from_str(
                &reqwest::get(&format!("http://localhost:8000/posts/{}", id))
                    .await?
                    .text()
                    .await?,
            )?;
            println!("Response: {:#?}", response);
        }
        PostCommand::GetAll => {
            println!("Get all posts");
            let response: Vec<models::post::Post> = serde_json::from_str(
                &reqwest::get("http://localhost:8000/posts")
                    .await?
                    .text()
                    .await?,
            )?;
            println!("Response: {:#?}", response);
        }
        PostCommand::Create(args) => {
            println!("Get all posts");
            let client = reqwest::Client::new();

            let formatted_body = serde_json::to_string(&CreatePostArgs { title: (args.title.to_string()), body: (args.body.to_string()) })?;

            client.post("http://localhost:8000/posts")
                .body(formatted_body)
                .send()
                .await?;

            println!("Post created");
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match cli.command {
        Command::Post(command) => handle_post_command(&command).await?,
    }

    Ok(())
}
