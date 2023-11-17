use std::{fs::{File, DirBuilder}, io::{Write, Read}};

pub fn get_token() -> Result<String, anyhow::Error> {
  let mut file = File::open("config/token.txt").expect("You don't have a token configured.");
  let mut token = String::new();
  file.read_to_string(&mut token)?;

  Ok(token)
}

pub fn store_token(token: String) -> Result<(), anyhow::Error> {
  DirBuilder::new().recursive(true).create("config")?;
  let mut file = File::create("config/token.txt")?;
  file.write_all(token.as_bytes())?;

  Ok(())
}
