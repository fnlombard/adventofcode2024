use regex::Regex;
use std::fs;

pub fn run() {
    let corrupt = parse_input("src/day03.txt");

    let operation_result = solution_one(&corrupt);
    println!("Solution 1 operations result: {}", operation_result);

    let conditional_operation_result = solution_two(&corrupt);
    println!(
        "Solution 2 conditional operations result: {}",
        conditional_operation_result
    );
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

fn solution_two(corrupt: &String) -> i32 {
    let filter: Vec<&str> = corrupt.split("don't()").collect();
    let mut result: Vec<&str> = Vec::new();

    result.push(filter[0]);

    for idx in 1..filter.len() {
        let inner_parts: Vec<&str> = filter[idx].split("do()").collect();
        for idy in 1..inner_parts.len() {
            result.push(inner_parts[idy]);
        }
    }

    let filtered_corrupt = result.join("");

    let re = Regex::new(r"mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)").unwrap();
    return re
        .captures_iter(&filtered_corrupt)
        .map(|caps| {
            let left = caps.name("left").unwrap().as_str().parse::<i32>().unwrap();
            let right = caps.name("right").unwrap().as_str().parse::<i32>().unwrap();
            return left * right;
        })
        .sum();
}
