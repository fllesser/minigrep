//! minigrep by flless
//!
//! minigrep is a simple command line tool that searches for a query string in a given file and returns the lines that contain the query string.
//!
//! # Usage
//! minigrep query filename
//!

pub mod config;
pub mod search;
pub use search::search_comprehensive;

use config::Config;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let mut flag: bool = false;
    let results = search_comprehensive(&config.query, &contents, config.case_sensitive);
    if results.len() != 0 {
        flag = true;
        for line in results {
            println!("{}", line);
        }
    }
    if !flag {
        //没有在文件中找到匹配的内容
        println!(
            "does not search '{}' in file '{}'",
            config.query, config.filename
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::search::{search_for, search_iter};

    use super::*;
    use search::{search, search_case_insensitive, search_mark};

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = concat!(
            "Rust:\n",
            "safe, fast, productive.\n",
            "Pick three.\n",
            "xixixixixixiixixixiixiixixixixixixixixixixixixixi"
        );

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_search_mark() {
        let query = "duct";
        let contents = concat!(
            "Rust:\n",
            "safe, fast, productive.\n",
            "Pick three.\n",
            "xixixixixixiixixixiixiixixixixixixixixixixixixixi"
        );
        assert_eq!(
            vec!["safe, fast, pro<<<duct>>>ive."],
            search_mark(query, contents)
        );
    }

    #[test]
    // 测试不区分大小写的搜索功能
    fn test_case_insensitive() {
        let query = "rUsT"; // 测试词，包含大小写混合
        let contents = concat!(
            "Rust:\n",
            "safe, fast, productive.\n",
            "Pick three.\n",
            "Trust me."
        );
        // 期望能找到两行包含 "rust" 的内容（不区分大小写）：
        // 1. "Rust:"
        // 2. "Trust me."
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn bench_for_and_iter() {
        let query = "the";
        let contents = fs::read_to_string("poem.txt").unwrap();
        // Performance test
        let start = std::time::Instant::now();
        let result_for = search_for(&query, &contents);
        let duration_for = start.elapsed();
        println!(
            "search by for took: {:?}, result len: {}",
            duration_for,
            result_for.len()
        );

        let contents = fs::read_to_string("poem.txt").unwrap();
        let start = std::time::Instant::now();
        let result_iter = search_iter(&query, &contents);
        let duration_iter = start.elapsed();
        println!(
            "search by iter took: {:?}, result len: {}",
            duration_iter,
            result_iter.len()
        );

        assert_eq!(result_for, result_iter);
    }
}
