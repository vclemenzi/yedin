use std::fs::File;
use std::io::prelude::*;

pub fn from_url(name: &str, url: &str) -> Result<(), reqwest::Error> {
    let bytes = reqwest::blocking::get(url)?.bytes()?;

    let mut file = File::create(format!("./node_modules/.temp/{}.tar.gz", name)).unwrap();
    file.write_all(&bytes).unwrap();

    Ok(())
}