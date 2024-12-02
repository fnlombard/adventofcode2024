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
        if is_safe(report, is_ascending) {
            safe_reports += 1;
        }
    }

    return safe_reports;
}

fn is_safe(report: &Vec<i32>, is_ascending: bool) -> bool {
    return report.windows(2).all(|level_pair| {
        let diff = level_pair[1] - level_pair[0];
        return (1..=3).contains(&diff.abs()) && (diff > 0) == is_ascending;
    });
}

fn is_safe_tolerance(report: &Vec<i32>, tolerance: usize) -> bool {
    let mut indexed_arr: Vec<(usize, i32)> =
        report.iter().enumerate().map(|(i, &v)| (i, v)).collect();

    let is_ascending: bool = report.windows(2).filter(|w| w[1] - w[0] > 0).count() >= tolerance;
    if is_ascending {
        indexed_arr.sort_by_key(|&(_, value)| value);
    } else {
        indexed_arr.sort_by_key(|&(_, value)| -value);
    }

    let mut changed_indices: Vec<usize> = indexed_arr
        .iter()
        .enumerate()
        .filter_map(|(new_index, &(original_index, _))| {
            if new_index != original_index {
                Some(original_index)
            } else {
                None
            }
        })
        .collect();
    changed_indices.sort_by(|a, b| b.cmp(a));

    let mut report_clone = report.clone();
    for idx in changed_indices {
        report_clone.remove(idx);
    }

    return is_safe(report, is_ascending);
}

fn solution_two(reports: &[Vec<i32>]) -> i32 {
    let mut safe_reports = 0;
    for report in reports {
        if is_safe_tolerance(report, 1) {
            safe_reports += 1;
        }
    }

    return safe_reports;
}
