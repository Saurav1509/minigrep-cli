use std::{error::Error, fs};

// Created Config struct so that we can have better relationship
// between query and file_path as they are used for storing
// fetched arguments
pub struct Config {
    pub query: String,
    pub file_path: String,
}

// implementation of new Config store and return the
// arguments provided by the user
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        //handeling error
        if args.len() < 3 {
            // panic!("Not enough arguments provided"); // the program panics here with a custom message

            return Err("Not enough arguments provided");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// using 'fs' to read from the file we have provided the path as inputs
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

// here in this function we are using something called a Lifetime.
// lifetime parameters specify which argument lifetime is connected to the lifetime of the return value.
// Here we want the lifetime of contents to be same as the return value.
// So there are three places we added 'a notation to connect the lifetime parameter with the return value.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
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
}
