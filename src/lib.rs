use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let result = if config.case_sensitive_search {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in result {
        println!("Found, {}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive_search: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let case_sensitive_search = env::var("CASE_SENSITIVE_SEARCH").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive_search,
        })
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = vec![];

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
To all my friends.
Too Many boys :)";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(query, contents)
        );
    }
}
