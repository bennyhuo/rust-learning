use std::error::Error;
use std::{env, fs};

pub struct Config<'a> {
    pub query: &'a String,
    pub file_path: &'a String,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            Err("not enough arguments")
        } else {
            let query = &args[1];
            let file_path = &args[2];

            let ignore_case = env::var("IGNORE_CASE").map_or(false, |value| match value.as_str() {
                "true" => true,
                "false" => false,
                "1" => true,
                _ => false,
            });

            Ok(Config {
                query,
                file_path,
                ignore_case,
            })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
        search_case_insensitive(config.query, &contents)
    } else {
        search(config.query, &contents)
    }
    .iter()
    .for_each(|line| println!("{line}"));
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().fold(vec![], |mut acc, line| {
        if line.contains(query) {
            acc.push(line)
        }
        acc
    })
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();
    contents.lines().fold(vec![], |mut acc, line| {
        if line.to_lowercase().contains(query) {
            acc.push(line)
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = indoc! {
            "
            Rust:
            safe, fast, productive.
            Pick three."
        };

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = indoc! {
            "
            Rust:
            safe, fast, productive.
            Pick three.
            Trust me.
            "
        };

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
