// AoC 2023 - Day 02

use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};


fn parse_engine_file(input_path: &Path) -> Vec<Vec<char>> {
    let file = File::open(input_path).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("failed to read a line");
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }
    grid
}


fn part1(grid: Vec<Vec<char>>) -> i32 {
    for (i, row) in grid.iter().enumerate() { 
        for (j, col) in row.iter().enumerate() {
            // do stuff
        }
    }


    let mut total: i32 = 0;
    total
}


fn main() {
    let part1_path = Path::new("inputs/ex1.txt");
    let grid = parse_engine_file(part1_path);
    let ans1: i32 = part1(grid);
    println!("Part 1 answer: {}", ans1);

    /*let part2_path = Path::new("inputs/input.txt");
    let ans2: i32 = part2(&part2_path);
    println!("Part 2 answer: {}", ans2);*/
}
