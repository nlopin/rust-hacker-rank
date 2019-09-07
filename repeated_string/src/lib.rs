use std::io::{BufRead, Stdin};
use std::error::Error;

pub struct Config {
    pub word: String,
    pub len: usize,
}

impl Config {
    pub fn parse(input: Stdin) -> Result<Config, Box<dyn Error>> {
        let mut lines = input.lock().lines();
        let word: String = lines.next().unwrap().unwrap();
        let len: usize = lines.next().unwrap().unwrap().parse().unwrap();

        Ok(Config { word, len })
    }
}

pub fn run(config: Config) -> i64 {
    count_a(config.word.as_str(), config.len)
}

pub fn count_a(s: &str, n: usize) -> i64 {
    if s.len() == 0 {
        return 0;
    }

    if s.len() >= n as usize {
        return s[0..n].matches("a").count() as i64;
    }

    let repeats = (n / s.len()) as i64;
    count_a(s, s.len()) * repeats + count_a(s, n % s.len())
}

#[cfg(test)]
mod count_a {
    use super::count_a;

    #[test]
    fn len_is_less_than_word() {
        assert_eq!(
            count_a("mama", 3),
            1
        );
    }

    #[test]
    fn len_is_equal_to_word() {
        assert_eq!(
            count_a("mama", 4),
            2
        );
    }

    #[test]
    fn len_is_zero() {
        assert_eq!(
            count_a("mama", 0),
            0
        );
    }

    #[test]
    fn len_is_bigger_than_word() {
        assert_eq!(
            count_a("mama", 8),
            4
        );
        assert_eq!(
            count_a("mama", 10),
            5
        )
    }

    #[test]
    fn word_is_empty() {
        assert_eq!(
            count_a("", 5),
            0
        );
    }

    #[test]
    fn word_has_only_a() {
        assert_eq!(
            count_a("aaa", 10),
            10
        );
    }

    #[test]
    fn word_has_no_a() {
        assert_eq!(
            count_a("hello", 10),
            0
        );
    }
}