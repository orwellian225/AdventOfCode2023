use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;

use std::time::Instant;

fn main() {

    let start = Instant::now();

    let input_file = match File::open("./resources/day1/input.txt") {
        Ok(val) => val,
        Err(_) => exit(1)
    };

    let reader = BufReader::new(input_file);
    for line_res in reader.lines() {
        if let Ok(line) = line_res {
        }
    }

    let duration = start.elapsed();
    dbg!(duration);
}