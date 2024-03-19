// AoC 2023 - Day 01

use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};


fn part1(input_path: &Path) -> i32 {
    // read the file line by line
    let file = File::open(input_path).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut total: i32 = 0;
    for line_result in reader.lines() {
        let line = line_result.expect("Could not split file into lines");

        let mut first_digit: i32 = -1;
        let mut last_digit: i32 = -1;

        for c in line.chars() {
            if c.is_digit(10) {
                if first_digit == -1 {
                    first_digit = c.to_digit(10).expect("Could not convert to int") as i32;
                }
                else {
                    last_digit = c.to_digit(10).expect("Could not convert to int") as i32;
                }
            }
        }
        if last_digit == -1 {
            last_digit = first_digit;
        }

        let new_num: i32 = format!("{}{}", first_digit.to_string(), last_digit.to_string()).parse().expect("Could not convert String->i32");
        total += new_num;
    }
    total
}


fn part2(input_path: &Path) -> i32 {
    // read the file line by line
    let file = File::open(input_path).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut total: i32 = 0;
    for line_result in reader.lines() {
        let line = line_result.expect("Could not split file into lines");

        // type usize to match idx from enumeration + position from find
        let mut earliest_position: usize = line.len() + 1;
        let mut latest_position: usize = 0;

        // I first init'd these as empty, but the chance they might be empty at runtime prevents \
        // the compiler from compiling, so am using Option instead
        let mut first_digit: Option<usize> = None;
        let mut last_digit: Option<usize> = None;

        let text_digits: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let char_digits: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

        // enumerate over text digits and look for them in line
        for (idx, digit) in text_digits.iter().enumerate() {
            // if line.find is Some, then inner scope gets the result of find as position
            // otherwise it continues
            
            let position: Vec<usize> = line.match_indices(digit).map(|(i, _)|i).collect();
            if position.len() > 0 {
                let min_pos = position.iter().min().expect("No min");
                let max_pos = position.iter().max().expect("No max");

                // if this is the earliest digit we've seen, update:
                if min_pos < &earliest_position {
                    earliest_position = *min_pos;
                    first_digit = Some(idx+1);
                }
                // same if this is the latest digit we've seen:
                if max_pos >= &latest_position {
                    latest_position = *max_pos;
                    last_digit = Some(idx+1);
                }
            }
        }
        // same for char digits
        for (idx, digit) in char_digits.iter().enumerate() {
            let position: Vec<usize> = line.match_indices(digit).map(|(i, _)|i).collect();
            if position.len() > 0 {
                let min_pos = position.iter().min().expect("No min");
                let max_pos = position.iter().max().expect("No max");

                if min_pos < &earliest_position {
                    earliest_position = *min_pos;
                    first_digit = Some(idx);
                }
                if max_pos >= &latest_position {
                    latest_position = *max_pos;
                    last_digit = Some(idx);
                }
            }
        }

        // append the first two digits
        if let(Some(first), Some(last)) = (first_digit, last_digit) {
            let new_num: i32 = format!("{}{}", first.to_string(), last.to_string()).parse().expect("Could not convert String->i32");
            total += new_num;
        } else {
            // we shouldn't reach here
            println!("ruh roh: {}", line);
        }
    }
    total
}


fn main() {
    let part1_path = Path::new("inputs/input1.txt");
    let ans1: i32 = part1(&part1_path);
    println!("Part 1 answer: {}", ans1);

    let part2_path = Path::new("inputs/input1.txt");
    let ans2: i32 = part2(&part2_path);
    println!("Part 2 answer: {}", ans2);
}
