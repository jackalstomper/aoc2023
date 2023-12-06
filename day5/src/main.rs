use std::fs::File;
use std::io::{self, BufRead};
use std::thread;

type Int = u64;
type Levels = [Vec<SeedRange>; 7];

#[derive(Debug)]
struct SeedRange {
    source: Int,
    dest: Int,
    len: Int,
}

impl SeedRange {
    fn resolve(&self, id: Int) -> Option<Int> {
        if id >= self.source && id < self.source + self.len {
            let idx = id - self.source;
            return Some(self.dest + idx);
        }
        None
    }
}

fn resolve(id: Int, levels: &Levels) -> Int {
    let mut current_id = id;
    for level in levels {
        for seed_range in level {
            if let Some(i) = seed_range.resolve(current_id) {
                current_id = i;
                break;
            }
        }
    }
    current_id
}

fn parse_input(file: File) -> (Vec<Int>, Levels) {
    let mut category_idx = 0;
    let mut seeds: Vec<Int> = Vec::new();
    const VEC: Vec<SeedRange> = Vec::new();
    let mut levels: Levels = [VEC; 7];
    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        if line.is_empty() {
            continue;
        }
        if line.contains(':') {
            if category_idx == 0 {
                seeds = line
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split(' ')
                    .filter(|c| !c.is_empty())
                    .map(|c| c.parse().unwrap())
                    .collect();
            }
            category_idx += 1;
            continue;
        }
        let nums: Vec<Int> = line.split(' ').map(|c| c.parse().unwrap()).collect();
        if let [dest, source, len] = nums[..] {
            levels[category_idx - 2].push(SeedRange { source, dest, len });
        }
    }
    (seeds, levels)
}

#[test]
fn part1() {
    let file = File::open("./input.txt").unwrap();
    let (seeds, levels) = parse_input(file);
    let mut min = Int::MAX;
    for seed in seeds {
        min = resolve(seed, &levels).min(min);
    }
    println!("Min: {}", min);
}

// Part 2
fn main() {
    let file = File::open("./input.txt").unwrap();
    let (seeds, levels) = parse_input(file);
    let mut pairs = Vec::new();
    let mut i = 0;
    while i < seeds.len() {
        if i + 1 >= seeds.len() {
            break;
        }
        if let [seed_id, len] = seeds[i..i + 2] {
            pairs.push((seed_id, len));
        }
        i += 2;
    }
    // Brute force. I paid for those cores so I'm using them.
    thread::scope(|scope| {
        let mut futures = Vec::new();
        for pair in pairs {
            let l = &levels;
            futures.push(scope.spawn(move || {
                let mut local_min = Int::MAX;
                let seed_range = pair.0..pair.0 + pair.1;
                for seed in seed_range {
                    local_min = resolve(seed, l).min(local_min);
                }
                local_min
            }));
        }
        let mut final_min = Int::MAX;
        for future in futures {
            let thread_min = future.join().unwrap();
            final_min = thread_min.min(final_min);
        }
        println!("Min: {final_min}");
    });
}
