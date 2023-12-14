use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};

type Field = Vec<Vec<u8>>;

const ROCK: u8 = b'O';
const AIR: u8 = b'.';

#[derive(Copy, Clone, Eq, PartialEq)]
enum Dir {
    North,
    West,
    South,
    East,
}

fn main() {}

fn move_rock(x: usize, y: usize, dir: Dir, field: &mut Field) {
    let mut new_y = y;
    let mut new_x = x;
    let row_limit = field.len() - 1;
    let col_limit = field[0].len() - 1;
    loop {
        let previous_pos = (new_y, new_x);
        match dir {
            Dir::North => {
                if new_y == 0 {
                    break;
                }
                new_y -= 1
            }
            Dir::West => {
                if new_x == 0 {
                    break;
                }
                new_x -= 1
            }
            Dir::South => {
                if new_y == row_limit {
                    break;
                }
                new_y += 1
            }
            Dir::East => {
                if new_x == col_limit {
                    break;
                }
                new_x += 1
            }
        }
        if field[new_y][new_x] != AIR {
            break;
        }
        field[new_y][new_x] = ROCK;
        field[previous_pos.0][previous_pos.1] = AIR;
    }
}

fn shift(field: &mut Field, dir: Dir) {
    // reverse iteration direction on south and east movement
    let ystart = match dir {
        Dir::South => field.len() - 1,
        _ => 0,
    };
    let xstart = match dir {
        Dir::East => field.len() - 1,
        _ => 0,
    };
    let mut y = ystart;
    while y < field.len() {
        let mut x = xstart;
        while x < field[0].len() {
            if field[y][x] == ROCK {
                move_rock(x, y, dir, field);
            }
            if dir == Dir::East {
                if x == 0 {
                    break;
                }
                x -= 1;
            } else {
                x += 1;
            }
        }
        if dir == Dir::South {
            if y == 0 {
                break;
            }
            y -= 1;
        } else {
            y += 1;
        }
    }
}

fn cycle_field(field: &mut Field) {
    shift(field, Dir::North);
    shift(field, Dir::West);
    shift(field, Dir::South);
    shift(field, Dir::East);
}

fn run(file: File, part2: bool) {
    let mut field: Field = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().bytes().collect())
        .collect();

    if part2 {
        let total_cycles = 1000000000;
        let mut seen_configs = HashMap::new();
        let mut fields_by_cycle = HashMap::new();
        seen_configs.insert(hash_field(&field), 0);
        fields_by_cycle.insert(0, field.clone());
        for cycle in 0..total_cycles {
            cycle_field(&mut field);
            fields_by_cycle.insert(cycle + 1, field.clone());
            if let Some(prev_cycle) = seen_configs.insert(hash_field(&field), cycle + 1) {
                // We've been in this config before which means the answer is inside the loop
                // Find the remaining iterations we need after repeating the loop as many times as possible
                let repeat_start = prev_cycle;
                let remainder = (total_cycles - repeat_start) % (cycle - prev_cycle + 1);
                let final_field = prev_cycle + remainder;
                field = fields_by_cycle.remove(&final_field).unwrap();
                break;
            }
        }
    } else {
        shift(&mut field, Dir::North);
    }
    println!("Sum is {}", score(field));
}

fn score(field: Field) -> usize {
    let mut sum = 0;
    for y in 0..field.len() {
        for x in 0..field[0].len() {
            if field[y][x] == ROCK {
                sum += field[0].len() - y;
            }
        }
    }
    sum
}

fn hash_field(field: &Field) -> u64 {
    let mut hasher = DefaultHasher::new();
    field.hash(&mut hasher);
    hasher.finish()
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
