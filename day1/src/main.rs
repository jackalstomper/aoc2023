use onig::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {}

fn s_to_c(s: &str) -> char {
    let c = s.chars().next().unwrap();
    if c.is_ascii_digit() {
        return c;
    }

    match s {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => panic!("invalid"),
    }
}

fn get_digit(s: &str, pattern: &Regex) -> i32 {
    let caps: Vec<_> = pattern
        .captures_iter(&s)
        .map(|m| m.at(1))
        .map(|c| c.unwrap())
        .collect();
    let d1 = caps.get(0).unwrap();
    let d2 = caps.get(caps.len() - 1).unwrap();
    let digit: String = [s_to_c(d1), s_to_c(d2)].iter().collect();
    digit.parse().unwrap()
}

#[test]
fn part1() {
    let file = File::open("./input.txt").unwrap();
    let pattern = Regex::new(r"([1-9])").unwrap();
    let mut sum = 0;
    for line in io::BufReader::new(file).lines() {
        let l = line.unwrap();
        sum += get_digit(&l, &pattern);
    }
    println!("{}", sum);
}

#[test]
fn part2() {
    let file = File::open("./input.txt").unwrap();
    let mut sum = 0;
    let pattern = Regex::new(r"(?=([1-9]|one|two|three|four|five|six|seven|eight|nine))").unwrap();
    for line in io::BufReader::new(file).lines() {
        let l = line.unwrap();
        sum += get_digit(&l, &pattern);
    }
    println!("{}", sum);
}
