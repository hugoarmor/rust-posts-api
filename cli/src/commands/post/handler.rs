use models::post::NewPost;

use crate::{commands::PostCommand, config::AppConfig};

pub async fn handle_post_command(_app_config: AppConfig, command: &PostCommand) -> Result<(), anyhow::Error> {
    match command {
        PostCommand::Get { id } => {
            let request_url = &format!("{}/posts/{}", _app_config.api_url, id);
            let response: models::post::Post = serde_json::from_str(
                &reqwest::get(request_url)
                    .await?
                    .text()
                    .await?,
            )?;
            println!("Response: {:#?}", response);
        }
        PostCommand::GetAll => {
            let request_url = format!("{}/posts", _app_config.api_url);
            let response: Vec<models::post::Post> = serde_json::from_str(
                &reqwest::get(request_url)
                    .await?
                    .text()
                    .await?,
            )?;
            println!("Response: {:#?}", response);
        }
        PostCommand::Create(args) => {
            let client = reqwest::Client::new();

            let formatted_body = serde_json::to_string(&NewPost {
                title: args.title.to_string(),
                body: args.body.to_string(),
            })?;

            let request_url = format!("{}/posts", _app_config.api_url);
            let response = client
                .post(request_url)
                .header("Authorization", _app_config.token)
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
