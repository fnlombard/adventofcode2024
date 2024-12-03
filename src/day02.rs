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

        if is_safe(report) {
            safe_reports += 1;
        }
    }
    return safe_reports;
}

fn is_safe(report: &Vec<i32>) -> bool {
    let is_ascending: bool = report.windows(2).all(|w| w[1] - w[0] > 0);
    let is_descending: bool = report.windows(2).all(|w| w[1] - w[0] < 0);

    if !is_ascending && !is_descending {
        return false;
    }

    return report.windows(2).all(|w| {
        let diff = w[1] - w[0];
        return (1..=3).contains(&diff.abs());
    });
}

fn is_safe_tolerance(report: &Vec<i32>) -> bool {

    for idx in 0..report.len() {
        let mut report_copy = report.clone();
        report_copy.remove(idx);
        if is_safe(&report_copy) {
            return true
        }
    }
    return false;
}

fn solution_two(reports: &[Vec<i32>]) -> i32 {
    let mut safe_reports = 0;
    for report in reports {
        if is_safe_tolerance(report) {
            safe_reports += 1;
        }
    }
    return safe_reports;
}
