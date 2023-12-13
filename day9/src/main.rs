use std::fs::File;
use std::io::{self, BufRead};

type Int = i32;

fn main() {}

fn predict2(current: Vec<Int>, part2: bool) -> Int {
    let mut diffs = vec![current];
    let mut i = 0;
    while diffs[i].iter().any(|n| *n != 0) {
        let differences: Vec<_> = diffs[i].windows(2).map(|n| n[1] - n[0]).collect();
        diffs.push(differences);
        i = diffs.len() - 1;
    }
    diffs.into_iter().rev().fold(0, |acc, e| {
        if part2 {
            e.first().unwrap() - acc
        } else {
            e.last().unwrap() + acc
        }
    })
}

fn run(file: File, part2: bool) {
    let mut sum = 0;
    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let nums: Vec<Int> = line
            .split(' ')
            .filter(|l| !l.is_empty())
            .map(|p| p.parse().unwrap())
            .collect();
        let next = predict2(nums, part2);
        sum += next;
    }
    println!("Sum is {sum}");
}

#[test]
fn part1() {
    let file = File::open("./input.txt").unwrap();
    run(file, false);
}

#[test]
fn part2() {
    let file = File::open("./input.txt").unwrap();
    run(file, true);
}
