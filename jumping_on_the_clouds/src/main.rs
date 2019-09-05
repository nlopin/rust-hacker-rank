use std::io;
use std::io::BufRead;

enum Cloud {
    Cumulus,
    Thunderheads,
}

fn main() {
    for line in io::stdin().lock().lines().skip(1).take(1) {
        if let Ok(line) = line {
            let clouds: Vec<Cloud> = line
                .split_whitespace()
                .map(|x| if x == "0" { Cloud::Cumulus } else { Cloud::Thunderheads })
                .collect();

            let mut position: usize = 0;
            let mut hops = 0;
            while position != (clouds.len() - 1) {
                let best_position = position + 2;
                position = match clouds.get(best_position) {
                    Some(c) => if let Cloud::Cumulus = c { best_position } else { position + 1 },
                    None => position + 1,
                };
                hops += 1;
            }
            println!("{}", hops);
        }
    }
}
