use std::env;
mod commands;
mod package;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "init" => commands::init::run(),
        "add" => commands::add::run(args[2].to_string()),
        _ => {
            println!("Usage: {} <a> <b>", args[0]);
        }
    }
}
