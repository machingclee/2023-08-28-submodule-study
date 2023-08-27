use std::env;
use std::process;

mod library;
use library::grep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = grep::Config::new(&args).unwrap_or_else(|err| {
        println!("[Problem parsing arguments] {}", err);
        process::exit(1);
    });

    if let Err(err) = grep::run(config) {
        println!("Application Error: {}", err);
        process::exit(1);
    }
}
