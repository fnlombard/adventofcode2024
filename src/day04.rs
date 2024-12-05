use regex::Regex;
use std::fs;

pub fn run() {
    let grid = parse_input("src/day04.txt");

    let operation_result = solution_one(&grid);
    println!("Solution 1 operations result: {}", operation_result);

    let operation_result = solution_two(&grid);
    println!("Solution 2 operations result: {}", operation_result);
}

fn parse_input(filename: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(filename).expect("Failed to read input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    return grid;
}

fn solution_one(grid: &Vec<Vec<char>>) -> usize {
    let re = Regex::new(r"XMAS").unwrap();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut sequences = Vec::new();

    // Horizontal
    sequences.extend(grid.iter().map(|row| row.iter().collect::<String>()));

    // Vertical
    for col in 0..cols {
        let sequence: String = (0..rows).map(|row| grid[row][col]).collect();
        sequences.push(sequence);
    }

    // Diagonal sequences (top-left to bottom-right)
    for k in 0..(rows + cols - 1) {
        let mut sequence = String::new();

        let (mut row, mut col) = if k < cols {
            (0, k)
        } else {
            (k - cols + 1, cols - 1)
        };

        while row < rows && col < cols {
            sequence.push(grid[row][col]);
            row += 1;

            if col == 0 {
                break;
            }

            col -= 1;
        }

        sequences.push(sequence);
    }

    // Anti-diagonal sequences (top-right to bottom-left)
    for k in 0..(rows + cols - 1) {
        let mut sequence = String::new();

        let (mut row, mut col) = if k < cols {
            (0, cols - k - 1)
        } else {
            (k - cols + 1, 0)
        };

        while row < rows && col < cols {
            sequence.push(grid[row][col]);
            row += 1;

            if col + 1 >= cols {
                break;
            }

            col += 1;
        }

        sequences.push(sequence);
    }

    let word_search = sequences.join(" ");

    let normal_count = re.find_iter(&word_search).count();
    let reverse_count = re
        .find_iter(&word_search.chars().rev().collect::<String>())
        .count();
    return normal_count + reverse_count;
}

fn solution_two(grid: &Vec<Vec<char>>) -> i32 {
    let rows: usize = grid.len();
    let cols: usize = grid[0].len();

    let mut matches: i32 = 0;

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if grid[row][col] != 'A' {
                continue;
            }

            let d1: (char, char) = (grid[row - 1][col - 1], grid[row + 1][col + 1]);
            let d2: (char, char) = (grid[row - 1][col + 1], grid[row + 1][col - 1]);

            let d1_match: bool = (d1.0 == 'M' && d1.1 == 'S') || (d1.0 == 'S' && d1.1 == 'M');
            let d2_match: bool = (d2.0 == 'M' && d2.1 == 'S') || (d2.0 == 'S' && d2.1 == 'M');

            if d1_match && d2_match {
                matches += 1;
            }
        }
    }

    matches
}
