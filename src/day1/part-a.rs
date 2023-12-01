use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;

fn main() {
    let input_file = match File::open("./resources/day1/input.txt") {
        Ok(val) => val,
        Err(_) => exit(1)
    };

    let mut net_val = 0;
    let reader = BufReader::new(input_file);
    for line in reader.lines() {
        if let Ok(val) = line {
            let mut line_val = String::new();
            for char in val.chars() {
                match char {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        line_val.push(char);
                    },
                    _ => ()
                }
            }

            let size = line_val.len();
            let mut found = line_val.chars().enumerate().filter(|a| a.0 == 0 || a.0 == size - 1).map(|a| a.1).collect::<String>();
            if found.len() == 1 {
                found = format!("{found}{found}");
            }
            let val: u32 = match found.parse() {
                Ok(val) => val,
                Err(_) => 0,
            };
            net_val += val;
        }
    }

    println!("Resultant value: {net_val}");

}
