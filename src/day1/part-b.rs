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

            // One
            // Two
            // Three
            // Four
            // Five
            // Six
            // Seven
            // Eight
            // Nine

            // O, T {W, H} , F {O, I}, S {I, E}, E, N

            for c in val.chars().enumerate() {
                match c.1 {
                    'o' => {
                        let one_substr: String = val.chars().skip(c.0).take(3).collect();
                        if one_substr.contains("one") {
                            line_val.push('1');
                        }
                    },
                    't' => {
                        match val.chars().nth(c.0 + 1).unwrap_or('\0') {
                            'w' => {
                                let two_substr: String = val.chars().skip(c.0).take(3).collect();
                                if two_substr.contains("two") {
                                    line_val.push('2');
                                }
                            },
                            'h' => {
                                let three_substr: String = val.chars().skip(c.0).take(5).collect();
                                if three_substr.contains("three") {
                                    line_val.push('3');
                                }
                            },
                            _ => ()
                        }
                    },
                    'f' => {
                        match val.chars().nth(c.0 + 1).unwrap_or('\0') {
                            'o' => {
                                let four_substr: String = val.chars().skip(c.0).take(4).collect();
                                if four_substr.contains("four") {
                                    line_val.push('4');
                                }
                            },
                            'i' => {
                                let five_substr: String = val.chars().skip(c.0).take(4).collect();
                                if five_substr.contains("five") {
                                    line_val.push('5');
                                }
                            },
                            _ => ()
                        }
                    },
                    's' => {
                        match val.chars().nth(c.0 + 1).unwrap_or('\0') {
                            'i' => {
                                let six_substr: String = val.chars().skip(c.0).take(3).collect();
                                if six_substr.contains("six") {
                                    line_val.push('6');
                                }
                            },
                            'e' => {
                                let seven_substr: String = val.chars().skip(c.0).take(5).collect();
                                if seven_substr.contains("seven") {
                                    line_val.push('7');
                                }
                            },
                            _ => ()
                        }
                    },
                    'e' => {
                        let eight_substr: String = val.chars().skip(c.0).take(5).collect();
                        if eight_substr.contains("eight") {
                            line_val.push('8');
                        }
                    },
                    'n' => {
                        let nine_substr: String = val.chars().skip(c.0).take(4).collect();
                        if nine_substr.contains("nine") {
                            line_val.push('9');
                        }
                    },
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        line_val.push(c.1);
                    },
                    _ => continue
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
