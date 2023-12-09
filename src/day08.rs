use std::collections::HashMap;
use std::str::FromStr;
use std::cmp::{max, min};
use crate::read_input_lines;
use anyhow::Result;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Node {
    id:String,
    left:String,
    right:String,
}
impl FromStr for Node {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shorter = s.replace(")", "");
        let (id, tail) = shorter.split_once(" = (").unwrap();
        let (left, right) = tail.split_once(", ").unwrap();
        Ok(Node{
            id: id.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        })
    }
}

pub fn prepare(file_name: &str) -> Result<(Vec<char>, HashMap<String, Node>)> {
    let input = read_input_lines(file_name);
    let cycle:Vec<char> = input
        .first()
        .unwrap()
        .trim()
        .chars()
        .collect();
    let nodes:HashMap<String, Node> = input
        .iter().skip(2)
        .filter_map(|line| line.parse().ok())
        .map(|node:Node| (node.id.clone(), node))
        .collect();
    Ok((cycle, nodes))
}

pub fn steps_from(instructions:&Vec<char>, nodes:&HashMap<String, Node>, start:String, end:&str) -> Option<usize> {
    let repeat = instructions.len();
    let mut step:usize = 0;
    let mut next = &start;
    while let Some(node) = nodes.get(next) {
        let instruction = instructions[step % repeat];
        next = match instruction {
            'L' => &node.left,
            'R' => &node.right,
            _  => unreachable!(),
        };
        step += 1;
        if next.ends_with(end) { break; }
    }
    Some(step)
}

pub fn part_1((instructions, nodes):&(Vec<char>, HashMap<String, Node>)) -> Option<usize> {
    steps_from(instructions, nodes, "AAA".to_string(), "ZZZ")
}

fn gcd(a:usize, b:usize) -> usize {
    let mut m = max(a, b);
    let mut n = min(a, b);
    while m != 0 {
        if m < n {
            (m, n) = (n, m);
        }
        m = m % n;
    }
    n
}

fn lcm(a:usize, b:usize) -> usize {
    a * b / gcd(a, b)
}

pub fn part_2((instructions, nodes):&(Vec<char>, HashMap<String, Node>)) -> Option<usize> {
    let start_nodes:Vec<&Node> = nodes
        .values()
        .filter(|n| n.id.ends_with("A"))
        .collect();
    let paths:Vec<usize> = start_nodes
        .iter()
        .map(|n| steps_from(&instructions, &nodes, n.id.clone(), "Z").unwrap())
        .collect();
    let mut steps:usize = paths[0];
    for path in paths.iter().skip(1) {
        steps = lcm(steps, *path);
    }
    Some(steps)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        if let Ok((instructions, nodes)) = prepare("day08-example-1.txt") {
            assert_eq!(instructions, vec!['R', 'L']);
            assert_eq!(nodes["AAA"],  Node { id: "AAA".to_string(), left: "BBB".to_string(), right: "CCC".to_string() });
        }
        if let Ok((instructions, nodes)) = prepare("day08-example-2.txt") {
            assert_eq!(instructions, vec!['L', 'L', 'R']);
            assert_eq!(nodes["ZZZ"],  Node { id: "ZZZ".to_string(), left: "ZZZ".to_string(), right: "ZZZ".to_string() });
        }
    }

    #[test]
    fn test_part_1() {
        if let Ok((instructions, nodes)) = prepare("day08-example-1.txt") {
            assert_eq!(part_1(&(instructions, nodes)), Some(2))
        }
        if let Ok((instructions, nodes)) = prepare("day08-example-2.txt") {
            assert_eq!(part_1(&(instructions, nodes)), Some(6))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok((instructions, nodes)) = prepare("day08-example-3.txt") {
            assert_eq!(part_2(&(instructions, nodes)), Some(6))
        }
    }
}