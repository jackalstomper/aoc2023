use std::fs::File;
use std::io::{self, BufRead};

fn main() {}

fn read_number(col_idx: usize, line: &str) -> (i32, usize) {
    let mut start = 0;
    for i in (0..col_idx).rev() {
        if !line.chars().nth(i).unwrap().is_numeric() {
            start = i + 1;
            break;
        }
    }
    let end = match line[col_idx..line.len()].find(|c: char| !c.is_numeric()) {
        Some(i) => col_idx + i,
        None => line.len(),
    };
    (line[start..end].parse().unwrap(), end)
}

fn get_numbers(col_idx: usize, row_idx: usize, lines: &Vec<String>) -> Vec<i32> {
    let mut nums = Vec::new();
    let top = if row_idx == 0 { 0 } else { row_idx - 1 };
    let bottom = lines.len().min(row_idx + 2);
    let left = if col_idx == 0 { 0 } else { col_idx - 1 };
    let right = col_idx + 2;
    for row in top..bottom {
        let line = &lines[row];
        let mut col = left;
        while col < right {
            if line.chars().nth(col).unwrap().is_numeric() {
                let num_info = read_number(col, line);
                nums.push(num_info.0);
                col = num_info.1;
                continue;
            }
            col += 1;
        }
    }
    nums
}

fn check_line(line_idx: usize, lines: &Vec<String>, part2: bool) -> i32 {
    let mut char_idx = 0;
    let line = &lines[line_idx];
    let mut total = 0;
    loop {
        let checker = if part2 {
            |c: char| c == '*'
        } else {
            |c: char| c != '.' && !c.is_numeric()
        };
        let substr = &line[char_idx..line.len()];
        let symbol_idx = match substr.find(checker) {
            Some(e) => char_idx + e,
            None => break,
        };
        let nums = get_numbers(symbol_idx, line_idx, lines);
        if part2 {
            if nums.len() == 2 {
                total += nums.iter().fold(1, |acc, e| acc * e);
            }
        } else {
            total += nums.iter().fold(0, |acc, e| acc + e);
        }
        char_idx = symbol_idx + 1;
    }
    total
}

#[test]
fn part1() {
    let file = File::open("./input.txt").unwrap();
    let mut sum = 0;
    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|e| e.unwrap())
        .collect();
    for line_idx in 0..lines.len() {
        let num = check_line(line_idx, &lines, false);
        if num > 0 {
            sum += num;
        }
    }
    println!("Sum is {}", sum);
}

#[test]
fn part2() {
    let file = File::open("./input.txt").unwrap();
    let mut sum = 0;
    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|e| e.unwrap())
        .collect();
    for line_idx in 0..lines.len() {
        let num = check_line(line_idx, &lines, true);
        if num > 0 {
            sum += num;
        }
    }
    println!("Sum is {}", sum);
}
