use regex::Regex;
use std::fs;

pub fn run() {
    let corrupt = parse_input("src/day03.txt");

    let operation_result = solution_one(&corrupt);
    println!("Solution 1 operations result: {}", operation_result);
}

fn parse_input(filename: &str) -> String {
    let input = fs::read_to_string(filename).expect("Failed to read input file");
    return input.lines().collect::<String>();
}

fn solution_one(corrupt: &String) -> i32 {
    let re = Regex::new(r"mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)").unwrap();
    return re
        .captures_iter(&corrupt)
        .map(|caps| {
            let left = caps.name("left").unwrap().as_str().parse::<i32>().unwrap();
            let right = caps.name("right").unwrap().as_str().parse::<i32>().unwrap();
            return left * right;
        })
        .sum();
}
