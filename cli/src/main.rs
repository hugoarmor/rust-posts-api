extern crate clap;

use clap::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum Args {
    #[command()]
    Publish {
        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long, default_value_t = 1)]
        count: u8,
    },
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    println!("{:?}", args);

    let response = reqwest::get("http://localhost:8000/posts")
        .await?
        .text()
        .await?;
    println!("response: {}", response);

    Ok(())
}
