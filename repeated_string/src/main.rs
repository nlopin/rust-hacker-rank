use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let string: String = lines.next().unwrap().unwrap();
    let len: usize = lines.next().unwrap().unwrap().parse().unwrap();

    println!("{}", calc_repeated_string(&string[..], len));
}

fn calc_repeated_string(s: &str, n: usize) -> i64 {
    let bytes = s.as_bytes();
    if bytes.len() >= n as usize {
        let mut count = 0;
        for i in 0..n {
            if bytes[i] == b'a' {
                count += 1;
            }
        }
        return count;
    }

    calc_repeated_string(s, s.len()) * (n / s.len()) as i64 + calc_repeated_string(s, n % s.len())
}
