use std::fs::File;
use std::io::{self, BufRead};

fn run_game_part1(line: &str) -> i32 {
    let mut parts = line.split(":");
    let game_id = parts.next().unwrap().split(" ").nth(1).unwrap();
    let args = parts.next().unwrap();
    for round in args.split(";") {
        for draw in round.split(",") {
            let mut parts = draw.split(" ");
            let count = parts.nth(1).unwrap().parse::<i32>().unwrap();
            let color = parts.next().unwrap();
            let limit = match color {
                "red" => 12,
                "blue" => 14,
                "green" => 13,
                _ => panic!("invalid"),
            };
            if count > limit {
                return -1;
            }
        }
    }

    game_id.parse().unwrap()
}

fn run_game_part2(line: &str) -> i32 {
    let mut parts = line.split(":");
    let args = parts.nth(1).unwrap();
    let mut bm = 0;
    let mut gm = 0;
    let mut rm = 0;
    for round in args.split(";") {
        for draw in round.split(",") {
            let mut parts = draw.split(" ");
            let count = parts.nth(1).unwrap().parse().unwrap();
            let color = parts.next().unwrap();
            match color {
                "red" => rm = rm.max(count),
                "blue" => bm = bm.max(count),
                "green" => gm = gm.max(count),
                _ => panic!("invalid"),
            }
        }
    }
    bm * gm * rm
}

#[test]
fn part1() {
    let file = File::open("./input.txt").unwrap();
    let mut sum = 0;
    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let id = run_game_part1(&line);
        if id >= 0 {
            sum += id;
        }
    }
    println!("Sum is {}", sum);
}

#[test]
fn part2() {
    let file = File::open("./input.txt").unwrap();
    let mut sum = 0;
    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let ret = run_game_part2(&line);
        sum += ret
    }
    println!("Sum is {}", sum);
}

fn main() {}
