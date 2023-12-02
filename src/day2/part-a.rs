use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;

struct BagConfig {
    red: usize,
    green: usize,
    blue: usize,
}


fn main() {
    const BAG: BagConfig = BagConfig {
        red: 12,
        green: 13,
        blue: 14
    };

    let input_file = match File::open("./resources/day2/input.txt") {
        Ok(val) => val,
        Err(_) => exit(1)
    };

    let mut sum: usize = 0;

    let reader = BufReader::new(input_file);
    for line_res in reader.lines() {
        if let Ok(line) = line_res {
            let split: Vec<&str> = line.split(':').collect();
            let game_id: usize = match split[0].split(' ').last().unwrap_or("0").parse() {
                Ok(val) => val,
                Err(_) => 0
            };
            let mut game_validity = true;

            for attempt in split[1].split(';') {
                for cubes_untrimmed in attempt.split(',') {
                    let cubes = cubes_untrimmed.trim();

                    let cube_split: Vec<&str> = cubes.split(' ').collect();
                    match cube_split[1].chars().next().unwrap_or_default() {
                        'r' => game_validity = game_validity && cube_split[0].parse::<usize>().unwrap_or_default() <= BAG.red,
                        'g' => game_validity = game_validity && cube_split[0].parse::<usize>().unwrap_or_default() <= BAG.green,
                        'b' => game_validity = game_validity && cube_split[0].parse::<usize>().unwrap_or_default() <= BAG.blue,
                        _ => (),
                    }

                }
            }

            if game_validity {
                sum += game_id;
            }
        }
    }

    println!("Sum: {}", sum);
}