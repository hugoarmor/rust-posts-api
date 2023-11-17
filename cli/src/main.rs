extern crate clap;

use std::{
    fs::{DirBuilder, File},
    io::{Read, Write},
};

use clap::Parser;
use models::post::NewPost;

pub mod commands;

use commands::{Command, Cli, PostCommand};

async fn handle_post_command(command: &PostCommand) -> Result<(), anyhow::Error> {
    match command {
        PostCommand::GetCommand { id } => {
            println!("Get post with id {}", id);
            let response: models::post::Post = serde_json::from_str(
                &reqwest::get(&format!("http://localhost:8000/posts/{}", id))
                    .await?
                    .text()
                    .await?,
            )?;
            println!("Response: {:#?}", response);
        }
        PostCommand::GetAllCommand => {
            println!("Get all posts");
            let response: Vec<models::post::Post> = serde_json::from_str(
                &reqwest::get("http://localhost:8000/posts")
                    .await?
                    .text()
                    .await?,
            )?;
            println!("Response: {:#?}", response);
        }
        PostCommand::CreateCommand(args) => {
            println!("Get all posts");
            let client = reqwest::Client::new();

            let auth_token = get_token()?;

            let formatted_body = serde_json::to_string(&NewPost {
                title: args.title.to_string(),
                body: args.body.to_string(),
            })?;

            let response = client
                .post("http://localhost:8000/posts")
                .header("Authorization", auth_token)
                .body(formatted_body)
                .send()
                .await?;

            if !response.status().is_success() {
                println!("Error: {:#?}", response.text().await?);
                return Ok(());
            }

            let response: models::post::Post = serde_json::from_str(
                &response.text().await?,
            )?;

            println!("Post created with id: {:#?}", response.id);
        }
    }

    Ok(())
}

fn handle_config_command(token: String) -> Result<(), anyhow::Error> {
    DirBuilder::new().recursive(true).create("config")?;
    let mut file = File::create("config/token.txt")?;
    file.write_all(token.as_bytes())?;

    Ok(())
}

fn get_token() -> Result<String, anyhow::Error> {
    let mut file = File::open("config/token.txt").expect("You don't have a token configured.");
    let mut token = String::new();
    file.read_to_string(&mut token)?;

    Ok(token)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match cli.command {
        Command::PostCommand(command) => handle_post_command(&command).await?,
        Command::ConfigCommand { token } => handle_config_command(token)?,
    }

    Ok(())
}
