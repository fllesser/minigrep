#![allow(dead_code)]

use std::env;
use dotenv::dotenv;

pub struct Config {
    pub query: String,
    pub filename: String,
    // true 为不区分大小写 
    pub case_sensitive: bool,
}


impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!("Usage: {} <query> <filename>", args[0]));
        }
        dotenv().ok();
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env
            ::var("CASE_INSENSITIVE")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .map_err(|_| "Invalid value for CASE_INSENSITIVE".to_string())?;
        Ok(Config { query, filename, case_sensitive })
    }
}
