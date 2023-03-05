use dialoguer::{theme::ColorfulTheme, Input};
use whoami;
use std::env;
use std::{fs::File, io::Write};

pub(crate) fn run() {
    let current_dir = env::current_dir().unwrap();
    let dir_name = current_dir.file_name().unwrap().to_str().unwrap();

    // make some question
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Package name")
        .default(dir_name.to_string())
        .interact_text()
        .unwrap();
    let version: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Package version")
        .default("1.0.0".to_string())
        .interact_text()
        .unwrap();
    let main: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Package entry point")
        .default("index.js".to_string())
        .interact_text()
        .unwrap();
    let author: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Package author")
        .default(whoami::username())
        .interact_text()
        .unwrap();
    let license: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Package license")
        .default("MIT".to_string())
        .interact_text()
        .unwrap();

    // ...
    let json_str = format!("{0}\n  \"name\": \"{2}\",\n  \"version\": \"{3}\",\n  \"main\": \"{4}\",\n  \"author\": \"{5}\",\n  \"license\": \"{6}\"\n{1}", "{", "}", name, version, main, author, license);

    let mut package = File::create("./package.json").unwrap();
    package.write_all(json_str.as_bytes()).unwrap();
}