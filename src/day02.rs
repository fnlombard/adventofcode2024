use std::fs;

pub fn run() {
    let reports = parse_input("src/day02.txt");

    let safe_reports = solution_one(&reports);
    println!("Solution 1 safe reports: {}", safe_reports);

    let dampened_safe_reports = solution_two(&reports);
    println!(
        "Solution 2 dampened safe reports: {}",
        dampened_safe_reports
    );
}

fn parse_input(filename: &str) -> Vec<Vec<i32>> {
    let input = fs::read_to_string(filename).expect("Failed to read input file");

    return input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().expect("Failed to parse number"))
                .collect()
        })
        .collect();
}

fn solution_one(reports: &[Vec<i32>]) -> i32 {
    let mut safe_reports = 0;
    for report in reports {
        if report.len() < 2 {
            continue;
        }

        let is_ascending = report[1] > report[0];
        if report.windows(2).all(|level_pair| {
            let diff = level_pair[1] - level_pair[0];
            return (1..=3).contains(&diff.abs()) && (diff > 0) == is_ascending;
        }) {
            safe_reports += 1;
        }
    }

    return safe_reports;
}

fn solution_two(reports: &[Vec<i32>]) -> i32 {
    let mut safe_reports = 0;
    for report in reports {
        if report.len() < 2 {
            continue;
        }

        let is_ascending = report[1] > report[0];

        let mut idx = 0;
        let mut comp_idx = 1;

        while comp_idx < report.len() {
            let diff = report[comp_idx] - report[idx];

            if (1..=3).contains(&diff.abs()) && (diff > 0) == is_ascending {
                idx += 1;
                comp_idx += 1;
                continue;
            } else {
                comp_idx += 1;
            }

            if comp_idx - idx == 3 {
                break;
            }
        }

        if comp_idx - idx < 3 {
            safe_reports += 1;
        }
    }

    return safe_reports;
}
