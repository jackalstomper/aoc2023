use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

type Int = i32;

fn main() {}

enum Op {
    Gt,
    Lt,
}

struct Instruction {
    op: Op,
    num: Int,
    no_op: bool,
    arg_name: String,
    dest: String,
}

impl Instruction {
    fn new(s: &str) -> Self {
        let parts: Vec<_> = s.split(':').collect();
        if parts.len() == 1 {
            return Instruction {
                op: Op::Gt,
                num: 0,
                no_op: true,
                arg_name: String::new(),
                dest: String::from(&parts[0][0..parts[0].len() - 1]),
            };
        }
        let condition = parts[0];
        let dest = parts[1];
        let arg_name = &condition[0..1];
        let operator = &condition[1..2];
        let num: Int = condition[2..].parse().unwrap();
        let op = match operator {
            ">" => Op::Gt,
            "<" => Op::Lt,
            _ => panic!("unsupported"),
        };
        Instruction {
            op,
            num,
            no_op: false,
            arg_name: String::from(arg_name),
            dest: String::from(dest),
        }
    }

    fn eval<'a>(&'a self, values: &HashMap<String, Int>) -> Option<&'a str> {
        if self.no_op {
            return Some(&self.dest);
        }
        let arg_val = values.get(&self.arg_name).unwrap();
        match self.op {
            Op::Gt => {
                if *arg_val > self.num {
                    return Some(&self.dest);
                }
            }
            Op::Lt => {
                if *arg_val < self.num {
                    return Some(&self.dest);
                }
            }
        }
        None
    }
}

fn parse_value(str: &str) -> HashMap<String, Int> {
    let mut ret = HashMap::new();
    for val in str[1..str.len() - 1].split(',') {
        let id = &val[0..1];
        let num: Int = val[2..].parse().unwrap();
        ret.insert(String::from(id), num);
    }
    ret
}

fn run(file: File) {
    let mut parse_values = false;
    let mut instruction_map = HashMap::new();
    let mut values = Vec::new();
    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        if line.is_empty() {
            parse_values = true;
            continue;
        }
        if parse_values {
            values.push(parse_value(&line));
        } else {
            let mut parts = line.split('{');
            let id = parts.next().unwrap();
            let instructions: Vec<_> = parts
                .next()
                .unwrap()
                .split(',')
                .map(|a| Instruction::new(a))
                .collect();
            instruction_map.insert(String::from(id), instructions);
        }
    }

    let mut sum = 0;
    for value_map in values {
        let mut next_instruction = "in";
        'process: loop {
            let instructions = instruction_map.get(next_instruction).unwrap();
            for instruction in instructions {
                if let Some(dest) = instruction.eval(&value_map) {
                    match dest {
                        "A" => {
                            sum += value_map.values().fold(0, |acc, e| acc + e);
                            break 'process;
                        }
                        "R" => break 'process,
                        e => {
                            next_instruction = e;
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("Sum is {sum}");
}

#[test]
fn part1() {
    let file = File::open("./input.txt").unwrap();
    run(file);
}
