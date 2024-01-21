#![feature(let_chains)]

use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = args.next();
        let file_path = args.next();

        let ignore_case = env::var("IGNORE_CASE").map_or(false, |value| match value.as_str() {
            "true" => true,
            "false" => false,
            "1" => true,
            _ => false,
        });

        if let Some(query) = query && let Some(file_path) = file_path {
            Ok(Config {
                query,
                file_path,
                ignore_case,
            })
        } else {
            Err("Not ...")
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    }
    .iter()
    .for_each(|line| println!("{line}"));
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
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
