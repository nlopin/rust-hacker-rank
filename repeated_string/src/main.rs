use std::{io, process};
use repeated_string;
use repeated_string::Config;

fn main() {
    let stdin = io::stdin();
    let parse_result = Config::parse(stdin);

    match parse_result {
        Ok(config) => println!("{}", repeated_string::run(config)),
        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1);
        }
    }
}