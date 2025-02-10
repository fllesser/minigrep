pub mod config;

use std::fs;
use config::Config;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    println!("Searching for {}", config.query);
    let mut flag: bool = false;
    let results = search_mark(&config.query, &contents);
    if results.len() != 0 {
        flag = true;
        for line in results {
            println!("{}", line);
        }
    }
    if !flag {
        //没有在文件中找到匹配的内容
        println!("does not search '{}' in file '{}'", config.query, config.filename);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_mark(query: &str, contents: &str) -> Vec<String> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .map(|line| { line.replace(query, &format!("<<<{}>>>", query)) })
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        // Update path to be relative to project root
        let args = vec!["minigrep".to_string(), "pub".to_string(), "poem.txt".to_string()];
        let config = Config::new(&args).unwrap_or_else(|e| {
            panic!("Problem parsing arguments, {}", e);
        });
        if let Err(e) = run(config) {
            panic!("Application error: {}", e);
        }
    }

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
        assert_eq!(vec!["safe, fast, pro<<<duct>>>ive."], search_mark(query, contents));
    }

    #[test]
    fn test_search_noline() {
        let args = vec!["minigrep".to_string(), "test".to_string(), "noline.txt".to_string()];
        let config = Config::new(&args).unwrap_or_else(|e| {
            panic!("Problem parsing arguments, {}", e);
        });
        if let Err(e) = run(config) {
            panic!("Application error: {}", e);
        }
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
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
