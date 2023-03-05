use std::env;
mod commands;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "init" => commands::init::run(),
        _ => {
            println!("Usage: {} <a> <b>", args[0]);
        }
    }
}
