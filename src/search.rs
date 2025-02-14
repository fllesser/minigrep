

/// Searches for a query in content with line numbers and highlighting
/// 
/// # Example
/// ```
/// let arg = "hello";
/// let contents = "Hello\nWorld\nRust";
/// let case_insensitive = true;
/// let results = minigrep::search::search_comprehensive(arg, contents, case_insensitive);
/// assert_eq!(vec!["0:<<<Hello>>>"], results);
/// ```
///
/// # Arguments
///
/// * `query` - The string to search for
/// * `contents` - The text content to search within
/// * `case_insensitive` - Boolean flag for case-insensitive search (where applicable)
///
/// # Returns
///
/// Returns a `Vec` containing the matching lines or modified lines depending on the function used.
/// Different functions return either `Vec<&str>` or `Vec<String>` based on whether they modify the output.
///
pub fn search_comprehensive(query: &str, contents: &str, case_insensitive: bool) -> Vec<String> {
    let query = if case_insensitive {
        query.to_lowercase()
    } else {
        query.to_string()
    };

    contents
        .lines()
        .enumerate()
        .map(|(i, line)| format!("{}:{}", i, line))
        .filter_map(|line| {
            let search_line = if case_insensitive {
                line.to_lowercase()
            } else {
                line.clone()
            };

            search_line.find(&query).map(|start| {
                let end = start + query.len();
                let ori_word = &line[start..end];
                line.replace(ori_word, &format!("<<<{}>>>", ori_word))
            })
        })
        .collect()
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
        .map(|line| line.replace(query, &format!("<<<{}>>>", query)))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub fn search_for<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for ele in contents.lines() {
        if ele.contains(query) {
            results.push(ele);
        }
    }
    results
}

pub fn search_iter<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
