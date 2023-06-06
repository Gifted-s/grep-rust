use std::fs;
use std::error::Error;
use std::env;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)?;// automatically return error
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("Found {}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Imcomplete Arguments");
        }

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str)-> Vec<&'a str>{
 let mut result = Vec::new();
  for line in contents.lines() {
    if line.contains(query){
      result.push(line);
    }
  }
  return result;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str)-> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines(){
       if line.to_lowercase().contains(&query){
        result.push(line);
       }
    }
    return result;
   }
   

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.
Trust me.
Duct tape.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}

