use std::io::{BufRead, Stdin};

pub struct Config {
    pub word: String,
    pub len: usize,
}

impl Config {
    pub fn parse(input: Stdin) -> Config {
        let mut lines = input.lock().lines();
        let word: String = lines.next().unwrap().unwrap();
        let len: usize = lines.next().unwrap().unwrap().parse().unwrap();

        Config { word, len }
    }
}

pub fn run(config: Config) -> i64 {
    count_a(config.word.as_str(), config.len)
}

pub fn count_a(s: &str, n: usize) -> i64 {
    if s.len() >= n as usize {
        return s[0..n].matches("a").count() as i64;
    }

    let repeats = (n / s.len()) as i64;
    count_a(s, s.len()) * repeats + count_a(s, n % s.len())
}