use regex::Regex;
use std::fs;

pub fn run() {
    let word_search = parse_input("src/day04.txt");

    let operation_result = solution_one(&word_search);
    println!("Solution 1 operations result: {}", operation_result);
}

fn parse_input(filename: &str) -> String {
    let input = fs::read_to_string(filename).expect("Failed to read input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
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

    sequences.join(" ")
}

fn solution_one(word_search: &String) -> usize {
    let re = Regex::new(r"XMAS").unwrap();
    let normal_count = re.find_iter(word_search).count();
    let reverse_count = re
        .find_iter(&word_search.chars().rev().collect::<String>())
        .count();
    return normal_count + reverse_count;
}
