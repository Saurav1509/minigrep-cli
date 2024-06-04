use std::{env, process};

use minigrep::Config;

fn main() {
    // using env to get input from user. collect() puts it in vector collection.
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for \'{}\' ", config.query);
    println!("In the file {}", config.file_path);

    // here we handle Result error case like this because the run fn does
    // not return any instance like config. Hence we use if let to detect
    // only if the error is being returned back.
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
