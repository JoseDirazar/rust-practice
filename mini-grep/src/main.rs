use mini_grep::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argunments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in file {}", config.query, config.filename);

    if let Err(e) = mini_grep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
