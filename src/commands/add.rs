use crate::package;
use std::{
    env,
    fs::{self, File},
    io::Write,
};
use whoami;

pub(crate) fn run(name: String) {
    if !fs::metadata("./node_modules").is_ok() {
        fs::create_dir("./node_modules").unwrap();
    }

    if !fs::metadata("./node_modules/.temp").is_ok() {
        fs::create_dir("./node_modules/.temp").unwrap();
    }

    if !fs::metadata("./package.json").is_ok() {
        let current_dir = env::current_dir().unwrap();
        let dir_name = current_dir.file_name().unwrap().to_str().unwrap();
        let json_str = format!("{0}\n  \"name\": \"{2}\",\n  \"version\": \"{3}\",\n  \"main\": \"{4}\",\n  \"author\": \"{5}\",\n  \"license\": \"{6}\"\n{1}", "{", "}", dir_name, "1.0.0", "index.js", whoami::username(), "MIT");

        let mut package = File::create("./package.json").unwrap();
        package.write(json_str.as_bytes()).unwrap();
    }

    if !fs::metadata("./yedin.lock").is_ok() {
        File::create("./yedin.lock").unwrap();
    }

    package::install::install(&name, "latest");
}
