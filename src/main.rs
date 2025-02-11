use minigrep::config::Config;
use std::env;

fn main() {
    //let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments, {}", e);
        std::process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
