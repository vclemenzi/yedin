use std::fs;

use crate::package;

pub(crate) fn run(name: String) {
    if !fs::metadata("./node_modules").is_ok() {
        fs::create_dir("./node_modules").unwrap();
    }

    if !fs::metadata("./node_modules/.temp").is_ok() {
        fs::create_dir("./node_modules/.temp").unwrap();
    }
    
    package::install::install(&name, "latest");
}