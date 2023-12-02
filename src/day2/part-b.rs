use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;

#[derive(Default)]
struct RGB {
    red: usize,
    green: usize,
    blue: usize,
}


fn main() {
    let input_file = match File::open("./resources/day2/input.txt") {
        Ok(val) => val,
        Err(_) => exit(1)
    };

    let mut sum: usize = 0;

    let reader = BufReader::new(input_file);
    for line_res in reader.lines() {
        if let Ok(line) = line_res {
            let split: Vec<&str> = line.split(':').collect();
            let mut mins_needed = RGB::default();

            for attempt in split[1].split(';') {
                for cubes_untrimmed in attempt.split(',') {
                    let cubes = cubes_untrimmed.trim();

                    let cube_split: Vec<&str> = cubes.split(' ').collect();
                    match cube_split[1].chars().next().unwrap_or_default() {
                        'r' => {
                            let red = cube_split[0].parse::<usize>().unwrap_or_default();
                            if red > mins_needed.red {
                                mins_needed.red = red;
                            }
                        },
                        'g' => {
                            let green = cube_split[0].parse::<usize>().unwrap_or_default();
                            if green > mins_needed.green {
                                mins_needed.green = green;
                            }
                        },
                        'b' => {
                            let blue = cube_split[0].parse::<usize>().unwrap_or_default();
                            if blue > mins_needed.blue {
                                mins_needed.blue = blue;
                            }
                        },
                        _ => (),
                    }
                }
            }

            sum += mins_needed.red * mins_needed.green * mins_needed.blue;
        }
    }

    println!("Sum: {}", sum);
}