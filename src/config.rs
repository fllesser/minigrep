#![allow(dead_code)]

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            Err(format!("Usage: {} <query> <filename>", args[0]))
        } else {
            Ok(Config {
                query: args[1].clone(),
                filename: args[2].clone(),
            })
        }
    }
}
