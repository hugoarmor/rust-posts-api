use anyhow::anyhow;
use directories::ProjectDirs;
use std::{
    fs::{DirBuilder, File},
    io::{Read, Write},
    path::PathBuf,
};

fn get_token_txt_path() -> Option<PathBuf> {
    let project_dir = ProjectDirs::from("com", "rust-posts", "cli")?;
    Some(project_dir.config_dir().join("token.txt"))
}

pub fn get_token() -> Result<Option<String>, anyhow::Error> {
    let token_txt_path = get_token_txt_path();
    if token_txt_path.is_none() {
        return Ok(None);
    }

    let file = File::open(token_txt_path.unwrap());
    if let Err(err) = file {
        if err.kind() == std::io::ErrorKind::NotFound {
            return Ok(None);
        }
        return Err(anyhow!("Could not open token file"));
    }

    let mut token = String::new();
    file.unwrap().read_to_string(&mut token)?;

    Ok(Some(token))
}

pub fn store_token(token: String) -> Result<(), anyhow::Error> {
    let token_txt_path = get_token_txt_path();
    if token_txt_path.is_none() {
        return Err(anyhow!(
            "Could not retrieve the location of the app configuration files"
        ));
    }

    DirBuilder::new()
        .recursive(true)
        .create(token_txt_path.unwrap().parent().unwrap())?;
    let mut file = File::create(get_token_txt_path().ok_or(anyhow!(
        "Could not retrieve the location of the app configuration files"
    ))?)?;
    file.write_all(token.as_bytes())?;

    Ok(())
}
