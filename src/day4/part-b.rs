use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;

fn main() {
    let input_file = match File::open("./resources/day4/input.txt") {
        Ok(val) => val,
        Err(_) => exit(1)
    };

    let mut sum: u32 = 0;
    let mut copies: Vec<u32> = Vec::from([0]);
    let reader = BufReader::new(input_file);
    for (line_i, line_res) in reader.lines().enumerate() {
        if let Ok(line) = line_res {
            if copies.len() == line_i {
                copies.push(0);
            }
            let current_copies = copies[line_i] + 1;

            let split: Vec<&str> = line.split(':').collect();
            let numbers: Vec<&str> = split[1].split('|').collect();
            let winning_nums_str = numbers[0].trim();
            let my_nums_str = numbers[1].trim();

            let mut winning_nums: Vec<u32> = Vec::new();
            for num_str in winning_nums_str.split(' ') {
                match num_str.parse::<u32>() {
                    Ok(val) => winning_nums.push(val),
                    Err(_) => ()
                }
            }

            let mut my_nums: Vec<u32> = Vec::new();
            for num_str in my_nums_str.split(' ') {
                match num_str.parse::<u32>() {
                    Ok(val) => my_nums.push(val),
                    Err(_) => ()
                }
            }

            winning_nums.sort();
            my_nums.sort();

            let mut local_sum: u32 = 0;
            let mut idx: usize = 0;
            for num in my_nums.iter() {
                while winning_nums[idx] < *num && idx < winning_nums.len() - 1 {
                    idx += 1;
                }

                if winning_nums[idx] == *num {
                    // if local_sum == 0 {
                    //     local_sum = 1;
                    // } else {
                    //     local_sum *= 2;
                    // }
                    local_sum += 1;
                }
            }

            // dbg!(line_i, local_sum, &copies);
            for _ in 0..current_copies {
                for i in 0..local_sum {
                    let idx = i as usize + line_i + 1;
                    match copies.get_mut(idx) {
                        Some(val) => *val += 1,
                        None => copies.push(1)
                    }
                }
            }

            sum += current_copies;
        }

    }
    println!("Sum of scratch cards {}", sum);

}
