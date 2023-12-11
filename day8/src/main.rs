use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {}

type Id = u32;
fn id_from_str(s: &str) -> Id {
    let mut out = 0;
    let mut i = 2;
    for c in s.chars() {
        let e = c as u32;
        out |= e << (i * 8);
        i -= 1;
    }
    out
}

struct Node {
    id: Id,
    left: Id,
    right: Id,
}

impl From<&String> for Node {
    fn from(s: &String) -> Self {
        let mut parts = s.split(" = ");
        let id = id_from_str(parts.next().unwrap());
        let nodes = parts.next().unwrap();
        let n: Vec<_> = nodes
            .split(", ")
            .map(|n| n.replace("(", "").replace(")", ""))
            .map(|n| id_from_str(&n))
            .collect();
        Node {
            id,
            left: n[0],
            right: n[1],
        }
    }
}

fn run(file: File) {
    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let mut directions = "";
    let mut nodes = HashMap::new();
    for i in 0..lines.len() {
        let line = &lines[i];
        if line.is_empty() {
            continue;
        }
        if i == 0 {
            directions = line;
            continue;
        }
        let node = Node::from(line);
        nodes.insert(node.id, node);
    }

    let mut current_node = nodes.get(&id_from_str("AAA")).unwrap();
    let mut step_count = 1;
    'inf_loop: loop {
        for c in directions.chars() {
            current_node = match c {
                'L' => nodes.get(&current_node.left).unwrap(),
                'R' => nodes.get(&current_node.right).unwrap(),
                _ => panic!("unsupported"),
            };
            if current_node.id == id_from_str("ZZZ") {
                break 'inf_loop;
            }
            step_count += 1;
        }
    }

    println!("Step Count: {step_count}");
}

#[test]
fn part1() {
    let file = File::open("./input.txt").unwrap();
    run(file);
}
