use std::collections::HashMap;
use std::fs;

pub fn run() {
    let (mut left_column, mut right_column) = parse_input("src/day01.txt");

    left_column.sort();
    right_column.sort();

    let solution_one_score = solution_one(&left_column, &right_column);
    println!("Solution 1 Score: {}", solution_one_score);

    let solution_two_score = solution_two(&left_column, &right_column);
    println!("Solution 2 Score: {}", solution_two_score);
}

fn parse_input(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let input = fs::read_to_string(filename).expect("Failed to read input file");

    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();

            match (parts.next(), parts.next()) {
                (Some(l), Some(r)) => match (l.parse::<i32>(), r.parse::<i32>()) {
                    (Ok(l_val), Ok(r_val)) => Some((l_val, r_val)),
                    _ => {
                        eprintln!("Invalid integers in line: {}", line);
                        None
                    }
                },
                _ => {
                    eprintln!("Invalid format in line: {}", line);
                    None
                }
            }
        })
        .unzip()
}

fn solution_one(left_column: &[i32], right_column: &[i32]) -> i32 {
    left_column
        .iter()
        .zip(right_column)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn solution_two(left_column: &[i32], right_column: &[i32]) -> i32 {
    let mut right_counts: HashMap<i32, i32> = HashMap::new();

    for &value in right_column {
        *right_counts.entry(value).or_insert(0) += 1;
    }

    left_column
        .iter()
        .map(|&value| value * (*right_counts.get(&value).unwrap_or(&0) as i32))
        .sum()
}
