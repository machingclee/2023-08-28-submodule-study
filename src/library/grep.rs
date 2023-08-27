use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let search_result = if config.case_insensitive {
        search_insensitive(&config.query, contents.as_str())
    } else {
        search(&config.query, contents.as_str())
    };
    println!("Search Result: {:#?}", search_result);
    Ok(())
}

pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
    pub case_insensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Config, &'static str> {
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err(); // default to false
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];

        Ok(Config {
            query,
            filename,
            case_insensitive,
        })
    }
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut container: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            container.push(line.trim());
        }
    }
    return container;
}

pub fn search_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut container: Vec<&str> = Vec::new();
    let new_query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&new_query) {
            container.push(line.trim());
        }
    }
    return container;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive,
        Pick three.";
        let search_result = search(query, contents);
        assert_eq!(vec!["safe, fast, productive,"], search_result);
    }

    #[test]
    fn case_insensitive() {
        let query = "dUcT";
        let contents = "\
        Rust:
        safe, fasT, producTive,
        Pick three.";
        let search_result = search_insensitive(query, contents);
        assert_eq!(vec!["safe, fasT, producTive,"], search_result);
    }
}
