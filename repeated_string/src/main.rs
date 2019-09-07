use std::io;
use repeated_string;
use repeated_string::Config;

fn main() {
    let stdin = io::stdin();
    let config = Config::parse(stdin);

    println!("{}", repeated_string::run(config));
}