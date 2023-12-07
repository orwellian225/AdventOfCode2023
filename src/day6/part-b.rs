use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;

use std::time::Instant;

fn main() {

    let start = Instant::now();

    let input_file = match File::open("./resources/day6/input.txt") {
        Ok(val) => val,
        Err(_) => exit(1)
    };

    let mut info: Vec<i64> = vec![0, 0];

    let reader = BufReader::new(input_file);
    for (line_i, line_res) in reader.lines().enumerate() {
        if let Ok(line) = line_res {

            let split: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
            let mut num_str = String::new();
            for s in split[1].split(' ') {
                if s.chars().nth(0).unwrap_or_default().is_digit(10) {
                    num_str.push_str(s);
                }
            }

            info[line_i] = num_str.parse().unwrap_or_default();
        }
    }

    let tm = info[0] as f64;
    let dm = info[1] as f64;

    let t1 = (( -tm + ( tm.powf(2.) - 4. * dm ).sqrt() ) / ( -2. )).floor() + 1.;
    let t2 = (( -tm - ( tm.powf(2.) - 4. * dm ).sqrt() ) / ( -2. )).ceil() - 1.;
    let result: f64 = t2 - t1 + 1.;
    dbg!(result);

    let duration = start.elapsed();
    dbg!(duration);
}