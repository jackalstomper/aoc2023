use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};

type Int = i32;

fn main() {}

const CARD_LEN: usize = 5;
type Cards = [Int; CARD_LEN];

#[derive(Eq)]
struct Hand {
    cards: Cards,
    score: Int,
    bet: Int,
}

impl Hand {
    fn tiebreak(&self, other: &Self) -> std::cmp::Ordering {
        for i in 0..CARD_LEN {
            if self.cards[i] > other.cards[i] {
                return Ordering::Greater;
            } else if self.cards[i] < other.cards[i] {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd<Hand> for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.score != other.score {
            if self.score > other.score {
                return Ordering::Greater;
            }
            return Ordering::Less;
        }
        self.tiebreak(other)
    }
}

fn score_hand(cards: &Cards) -> Int {
    let mut counts = [0; 14];
    for c in cards {
        counts[*c as usize - 1] += 1;
    }
    counts[1..].sort_by(|a, b| b.cmp(a));
    counts[1] += counts[0]; // Jokers become the strongest card
    match counts[1] {
        5 => 6,
        4 => 5,
        3 => {
            if counts[2] == 2 {
                4
            } else {
                3
            }
        }
        2 => {
            if counts[2] == 2 {
                2
            } else {
                1
            }
        }
        1 => 0,
        _ => panic!("unsupported"),
    }
}

fn str_to_cards(cards: &str, part2: bool) -> Cards {
    let mut out = [0; CARD_LEN];
    let mut i = 0;
    for c in cards.chars() {
        out[i] = if c.is_ascii_digit() {
            c.to_digit(10).unwrap() as Int
        } else {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => {
                    if part2 {
                        1
                    } else {
                        11
                    }
                }
                'T' => 10,
                _ => panic!("unsupported"),
            }
        };
        i += 1;
    }
    out
}

fn parse_input(file: File, part2: bool) -> Vec<Hand> {
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let mut parts = line.split(' ');
            let hand_part = parts.next().unwrap();
            let bet_part = parts.next().unwrap();
            let cards = str_to_cards(hand_part, part2);
            let score = score_hand(&cards);
            Hand {
                cards,
                score,
                bet: bet_part.parse().unwrap(),
            }
        })
        .collect()
}

fn run_game(file: File, part2: bool) {
    let mut hands = parse_input(file, part2);
    hands.sort();
    let mut sum = 0;
    for i in 0..hands.len() {
        let value = hands[i].bet * (i as Int + 1);
        sum += value;
    }
    println!("Sum is {sum}");
}

#[test]
fn part1() {
    let file = File::open("./input.txt").unwrap();
    run_game(file, false);
}

#[test]
fn part2() {
    let file = File::open("./input.txt").unwrap();
    run_game(file, true);
}
