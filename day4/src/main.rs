use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {}

type Int = usize;

struct Card {
    match_count: Int,
    copies: Int,
}

fn parse_nums(nums: &str) -> HashSet<Int> {
    nums.split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part2_sum(mut cards: Vec<Card>) -> Int {
    let mut count = 0;
    for i in 0..cards.len() {
        let card = &mut cards[i];
        let start = i + 1;
        let end = start + card.match_count;
        let copies = card.copies;
        count += copies;
        for i in start..end {
            cards[i].copies += copies;
        }
    }
    count
}

fn run_game(lines: &Vec<String>, part2: bool) -> Int {
    let mut part1_score = 0;
    let cards: Vec<_> = lines
        .iter()
        .map(|line| {
            let mut parts = line.split(':').nth(1).unwrap().split('|');
            let winning = parse_nums(parts.next().unwrap());
            let nums = parse_nums(parts.next().unwrap());
            let match_count = nums.iter().fold(0, |acc, num| {
                if !winning.contains(num) {
                    return acc;
                }
                acc + 1
            });
            if match_count > 0 {
                part1_score += 2usize.pow((match_count - 1) as u32)
            }
            Card {
                match_count,
                copies: 1,
            }
        })
        .collect();
    if part2 {
        part2_sum(cards)
    } else {
        part1_score
    }
}

#[test]
fn part1() {
    let file = File::open("./input.txt").unwrap();
    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let sum = run_game(&lines, false);
    println!("Sum is {}", sum);
}

#[test]
fn part2() {
    let file = File::open("./input.txt").unwrap();
    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let count = run_game(&lines, true);
    println!("Card count is {}", count);
}
