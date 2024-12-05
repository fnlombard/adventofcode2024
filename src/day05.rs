use std::fs;
use std::collections::HashMap;

pub fn run() {
    let (rule_map, updates) = parse_input("src/day05.txt");
    println!("{:?}", rule_map)
}

fn parse_input(filename: &str) -> (HashMap<i32, i32>, Vec<Vec<i32>>) {
    let input = fs::read_to_string(filename).expect("Failed to read input file");
    let input_split: Vec<&str> = input.split("\n\n").collect();

    let mut rule_map = HashMap::new();

    for line in input_split[0].lines() {
        let line_split: Vec<&str> = line.split("|").collect();
        let key = line_split[0].parse::<i32>().unwrap();
        let value = line_split[1].parse::<i32>().unwrap();

        rule_map.insert(key, value );
    }

    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in input_split[1].lines() {
        updates.push(line.split(",").map(|c| c.parse::<i32>().unwrap()).collect());
    }

    return (rule_map, updates);
}

