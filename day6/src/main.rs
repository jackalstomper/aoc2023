use std::fs::File;
use std::io::{self, BufRead};

type Int = i64;

fn main() {}

struct Race {
    time: Int,
    distance: Int,
}

impl Race {
    fn range(&self) -> Int {
        let mut count = 0;
        for t in 1..self.time {
            let length = -t * t + self.time * t;
            if length > self.distance {
                count += 1;
            }
        }
        count
    }
}

fn parse_input_part1(file: File) -> Vec<Race> {
    let lines: Vec<Vec<Int>> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split(':')
                .nth(1)
                .unwrap()
                .split(' ')
                .filter(|e| !e.is_empty())
                .map(|e| e.parse().unwrap())
                .collect()
        })
        .collect();
    (0..lines[0].len())
        .map(|i| Race {
            time: lines[0][i],
            distance: lines[1][i],
        })
        .collect()
}

fn parse_input_part2(file: File) -> Race {
    let lines: Vec<Int> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split(':')
                .nth(1)
                .unwrap()
                .replace(" ", "")
                .parse()
                .unwrap()
        })
        .collect();
    Race {
        time: lines[0],
        distance: lines[1],
    }
}

#[test]
fn part1() {
    let file = File::open("./input.txt").unwrap();
    let races = parse_input_part1(file);
    let result = races.iter().fold(1, |acc, e| acc * e.range());
    println!("{}", result);
}

#[test]
fn part2() {
    let file = File::open("./input.txt").unwrap();
    let race = parse_input_part2(file);
    println!("{}", race.range());
}
