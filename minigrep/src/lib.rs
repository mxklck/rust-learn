use std::env;
use std::error::Error;
use std::fs;

// everything is public here
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // new is expected to never fail, so we rename to build
    // 'static is a lifetime specifier,
    // which means that the reference will live for the entire duration of the program
    // mut args: impl Iterator<Item = String> is a trait bound
    // it means that args has to be an iterator that yields strings
    // take ownership (no reference), mutate it and consume it.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next(); // moves the iterator to second element (first contains the program name)

        // next returns an Option, so we can match on it
        let query = match args.next() {
            Some(args) => args,
            None => return Err("No query string provided"), // return early with an Err variant of Result
        };

        let file_path = match args.next() {
            Some(args) => args,
            None => return Err("No file path provided"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    // I hate this syntax :(
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

// lifetime specifier 'a denotes that the returned vector string slices will
// live at least as long as the contents string
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // filter is an iterator adaptor that takes a closure and returns a new iterator
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// not borrowed mutably because we don't want to modify the query
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // iterators are a zero-cost abstraction
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*; // this imports all the public functions from the parent module

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
