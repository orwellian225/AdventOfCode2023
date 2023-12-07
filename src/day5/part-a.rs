
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;

use std::time::Instant;

#[derive(Debug, Default, PartialEq, PartialOrd, Eq)]
struct MapSegment {
    offset: usize,
    len: usize,
    dest: usize,
}

impl Ord for MapSegment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.offset.cmp(&other.offset)
    }
}

fn eval_map(value: usize, map: &Vec<MapSegment>) -> usize {
    for map_segment in map {
        if value < map_segment.offset {
            return value;
        }

        if value >= map_segment.offset + map_segment.len {
            continue;
        }

        return map_segment.dest + (value - map_segment.offset);
    }

    return value;
}

fn main() {

    let start = Instant::now();

    let input_file = match File::open("./resources/day5/input.txt") {
        Ok(val) => val,
        Err(_) => exit(1)
    };

    let reader = BufReader::new(input_file);
    let mut all_lines = reader.lines();

    // Seeds parsing
    let first_line = match all_lines.next() {
        Some(val) => match val {
            Ok(val) => val,
            Err(_) => exit(1)
        },
        None => exit(1)
    };

    let mut values: Vec<(usize, usize)> = Vec::new(); // start off as a seed
    let seed_split: Vec<&str> = first_line.split(':').collect();
    let seed_strs: Vec<&str> = seed_split[1].trim().split(' ').collect();
    for i in (0..seed_strs.len()).step_by(2) {
        let start: usize = match seed_strs[i].parse() {
            Ok(val) => val,
            Err(_) => 0
        };

        let len: usize = match seed_strs[i + 1].parse() {
            Ok(val) => val,
            Err(_) => 0
        };

        values.push((start, len));
    }

    let mut all_maps: [Vec<MapSegment>; 7] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new()
    ];
    
    for map_idx in 0..7 {
        let map_iter = all_lines.by_ref().skip_while(|line_res| {
            match line_res {
                Ok(val) => !val.chars().nth(0).unwrap_or_default().is_digit(10),
                Err(_) => false
            }
        }).take_while(|line_res| {
            match line_res {
                Ok(val) => val.chars().nth(0).unwrap_or_default().is_digit(10),
                Err(_) => false
            }
        });
        for map_line_res in map_iter {
            if let Ok(map_line) = map_line_res {
                let numbers: Vec<usize> = map_line.split(' ').map(|str| str.parse::<usize>().unwrap_or_default()).collect();
                all_maps[map_idx].push(MapSegment { offset: numbers[1], dest: numbers[0], len: numbers[2] });
            }
        }
        all_maps[map_idx].sort();
    }

    let mut min = usize::MAX;
    for value in values {
        for x in value.0..value.0 + value.1 {
            let mut y = x;
            for map in all_maps.iter() {
                y = eval_map(y, &map)
            }

            if min > y {
                min = y;
            }
        }
    }

    dbg!(min);
    dbg!(start.elapsed());
}