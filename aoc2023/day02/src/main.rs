// AoC 2023 - Day 02

use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;


fn part1(input_path: &Path) -> i32 {
    let file = File::open(input_path).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut total: i32 = 0;
    for line_result in reader.lines() {
        let line = line_result.expect("Could not split file into lines");
        let parts: Vec<&str> = line.split(':').collect();
        let game_id_str: &str = parts[0].split(" ").collect::<Vec<&str>>()[1];
        let game_id: i32 = game_id_str.parse().expect("could not parse str to i32");

        let mut game_impossible: bool = false;

        let grabs: Vec<&str> = parts[1].split(';').collect();
        for grab in grabs {
            let vals: Vec<&str> = grab.split(" ").collect();
            for idx in (1..vals.len()).step_by(2) {

                let amount: i32 = vals[idx].parse().expect("could not parse");
                let colour: &str = &vals[idx+1].replace(",", "");
                
                let max_amount = match colour {
                    "red" => 12, 
                    "green" => 13,
                    "blue" => 14, 
                    &_ => 99999,
                };
                
                // println!("{}\n{:?}", line, vals);
                // println!("id: {}, amount: {}, colour: {}, max_amount: {}\n", game_id, amount, colour, max_amount);
                if amount > max_amount {
                    game_impossible = true;
                }
            }
        }
        if !game_impossible {
            total += game_id;
        }
    }
    total
}


fn part2(input_path: &Path) -> i32 {
    let file = File::open(input_path).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut total: i32 = 0;
    for line_result in reader.lines() {

        let mut color_counts = HashMap::new();

        color_counts.insert("red", 0);
        color_counts.insert("green", 0);
        color_counts.insert("blue", 0);

        let line = line_result.expect("Could not split file into lines");
        let parts: Vec<&str> = line.split(':').collect();
        
        let grabs: Vec<&str> = parts[1].split(';').collect();
        for grab in grabs {
            let vals: Vec<&str> = grab.split(" ").collect();
            for idx in (1..vals.len()).step_by(2) {

                let amount: i32 = vals[idx].parse().expect("could not parse");
                let colour: &str = &vals[idx+1].replace(",", "");

                if let Some(count) = color_counts.get_mut(colour) {
                    if amount > *count {
                        *count = amount;
                    }
                } else {
                    println!("Could not get colour");
                }
            }
        }
        total += color_counts.values().product::<i32>();
    }
    total
}


fn main() {
    let part1_path = Path::new("inputs/input.txt");
    let ans1: i32 = part1(&part1_path);
    println!("Part 1 answer: {}", ans1);

    let part2_path = Path::new("inputs/input.txt");
    let ans2: i32 = part2(&part2_path);
    println!("Part 2 answer: {}", ans2);
}
