use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

type Int = i64;

struct Galaxy {
    x: Int,
    y: Int,
}

struct Universe {
    xranges: Vec<Range<Int>>,
    yranges: Vec<Range<Int>>,
    galaxies: Vec<Galaxy>,
}

impl Universe {
    fn len(&self) -> usize {
        self.galaxies.len()
    }

    fn new(file: File) -> Self {
        let lines: Vec<_> = io::BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .collect();
        let mut x_list = Vec::new();
        let mut y_list = Vec::new();
        let mut galaxies = Vec::new();
        for i in 0..lines.len() {
            let b = lines[i].as_bytes();
            for j in 0..b.len() {
                if b[j] == '#' as u8 {
                    x_list.push(j as Int);
                    y_list.push(i as Int);
                    galaxies.push(Galaxy {
                        x: j as Int,
                        y: i as Int,
                    });
                }
            }
        }
        x_list.sort();
        y_list.sort();
        let make_range = |list: Vec<Int>| -> Vec<Range<Int>> {
            let mut ranges = Vec::new();
            let mut range_start = list[0];
            for n in list[1..].windows(2) {
                if n[1] - n[0] > 1 {
                    ranges.push(range_start..n[0] + 1);
                    range_start = n[1];
                }
            }
            ranges.push(range_start..*list.last().unwrap() + 1);
            ranges
        };
        Universe {
            xranges: make_range(x_list),
            yranges: make_range(y_list),
            galaxies,
        }
    }

    fn dist(&self, left: usize, right: usize, part2: bool) -> Int {
        let l = &self.galaxies[left];
        let r = &self.galaxies[right];
        let ystart = l.y.min(r.y);
        let yend = l.y.max(r.y);
        let xstart = l.x.min(r.x);
        let xend = l.x.max(r.x);
        let mut ydist = 0;
        let mut xdist = 0;
        for y in ystart..yend {
            if self.is_y_clear(y) {
                ydist += if part2 { 1000000 } else { 2 }; // empty row
                continue;
            }
            ydist += 1;
        }
        for x in xstart..xend {
            if self.is_x_clear(x) {
                xdist += if part2 { 1000000 } else { 2 }; // empty column
                continue;
            }
            xdist += 1;
        }
        ydist + xdist
    }

    fn is_y_clear(&self, y: Int) -> bool {
        for yrange in &self.yranges {
            if yrange.contains(&y) {
                return false;
            }
        }
        true
    }

    fn is_x_clear(&self, x: Int) -> bool {
        for xrange in &self.xranges {
            if xrange.contains(&x) {
                return false;
            }
        }
        true
    }
}

fn main() {}

fn run(file: File, part2: bool) {
    let mut sum = 0;
    let universe = Universe::new(file);
    for i in 0..universe.len() {
        for j in i + 1..universe.len() {
            sum += universe.dist(i, j, part2);
        }
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
