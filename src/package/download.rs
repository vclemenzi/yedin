use std::fs::File;
use std::io::prelude::*;

pub fn from_url(name: &str, url: &str) -> Result<(), reqwest::Error> {
    let bytes = reqwest::blocking::get(url)?.bytes()?;

    match File::create(format!("./node_modules/.temp/{}.tar.gz", name)) {
        Ok(mut file) => {
            file.write_all(&bytes).unwrap();
        }
        Err(_e) => (),
    }

    Ok(())
}