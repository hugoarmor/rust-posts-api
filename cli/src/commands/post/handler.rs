use models::post::NewPost;

use crate::{commands::PostCommand, services::auth};

pub async fn handle_post_command(command: &PostCommand) -> Result<(), anyhow::Error> {
    match command {
        PostCommand::GetCommand { id } => {
            let response: models::post::Post = serde_json::from_str(
                &reqwest::get(&format!("http://localhost:8000/posts/{}", id))
                    .await?
                    .text()
                    .await?,
            )?;
            println!("Response: {:#?}", response);
        }
        PostCommand::GetAllCommand => {
            let response: Vec<models::post::Post> = serde_json::from_str(
                &reqwest::get("http://localhost:8000/posts")
                    .await?
                    .text()
                    .await?,
            )?;
            println!("Response: {:#?}", response);
        }
        PostCommand::CreateCommand(args) => {
            let client = reqwest::Client::new();

            let auth_token = auth::get_token()?;

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

            let response: models::post::Post = serde_json::from_str(&response.text().await?)?;

            println!("Post created with id: {:#?}", response.id);
        }
    }

    Ok(())
}
