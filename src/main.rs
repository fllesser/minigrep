use std::env;
use minigrep::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments, {}", e);
        std::process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
