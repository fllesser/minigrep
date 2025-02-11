#![allow(dead_code)]

use dotenv::dotenv;
use std::env::{self, Args};

pub struct Config {
    pub query: String,
    pub filename: String,
    // true 为不区分大小写
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, String> {
        let cmd = args.next().unwrap();
        if args.len() < 2 {
            return Err(format!("Usage: {cmd} <query> <filename>"));
        }

        let query = args.next().unwrap();
        let filename = args.next().unwrap();

        // 读取 .env
        dotenv().ok();
        let case_sensitive = env::var("CASE_INSENSITIVE")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .map_err(|_| "Invalid value for CASE_INSENSITIVE".to_string())?;
        
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
