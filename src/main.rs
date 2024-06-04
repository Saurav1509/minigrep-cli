use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let (query, file_path) = parse_config(&args);

    println!("Searching for \'{}\' ", query);
    println!("In the file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");

    println!("With Text: \n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    return (query, file_path);
}