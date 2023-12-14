use std::time::SystemTime;
use crate::read_input_lines;
use anyhow::Result;

#[derive(Debug)]
pub struct Record {
    pattern: String,
    sequence: Vec<usize>,
    needed: usize,
}
impl Record {

    pub fn total_permutations(&self) -> usize {
        self.count_permutation(0, 0, 0, vec![0])
    }

    pub fn count_permutation(&self, pos:usize, slots_used:usize, slots_filled:usize, mut sequence:Vec<usize>) -> usize {
        // if there are no more positions to check
        // either we met requirements or not
        if pos == self.pattern.len() {
            if sequence.last().unwrap() == &0 { sequence.pop(); }
            return if slots_filled == self.needed && self.sequence.starts_with(&sequence) { 1 } else { 0 }
        };

        // get the next token to match
        let next:char = self.pattern.chars().nth(pos).unwrap();

        // get the current working grouping
        let curr:usize = sequence.pop().unwrap();

        match next {
            '#' => {
                // does the sequence so far match the required sequence?
                // then keep going, otherwise, no valid options here
                if self.sequence.starts_with(&sequence) {
                    // add to the current working grouping
                    sequence.push(curr + 1);
                    self.count_permutation(pos + 1, slots_used, slots_filled + 1, sequence.clone())
                } else { 0 }
            },

            '.' => {
                // if the current working group is not empty
                // stop adding to it, and start a new one
                sequence.push(curr);
                if curr > 0 {
                    sequence.push(0);
                }
                self.count_permutation(pos + 1, slots_used, slots_filled, sequence)
            },

            '?' => {
                let hash = {
                    let mut next_sequence = sequence.clone();
                    // does the sequence so far match the required sequence?
                    // then keep going, otherwise, no valid options here
                    if self.sequence.starts_with(&sequence) {
                        // add to the current working grouping
                        next_sequence.push(curr + 1);
                        self.count_permutation(pos + 1, slots_used, slots_filled + 1, next_sequence)
                    } else { 0 }
                };
                let dot = {
                    let mut next_sequence = sequence.clone();
                    // if the current working group is not empty
                    // stop adding to it, and start a new one
                    next_sequence.push(curr);
                    if curr > 0 {
                        next_sequence.push(0);
                    }
                    self.count_permutation(pos + 1, slots_used, slots_filled, next_sequence)
                };

                hash + dot
            },

            _ => unreachable!(),
        }
    }

    pub fn to_long(&self) -> Record {
        Record {
            pattern:  vec![self.pattern.clone(); 5].join("?"),
            sequence: vec![self.sequence.clone(); 5].concat(),
            needed:   vec![self.sequence.clone(); 5].concat().iter().sum(),
        }
    }
}

pub fn prepare(file_name: &str) -> Result<Vec<Record>> {
    let input = read_input_lines(file_name);
    let records:Vec<Record> = input
        .iter()
        .map(|line| {
            let (pattern, sequence) = line.split_once(" ").unwrap();
            Record{
                pattern:  pattern.to_string(),
                sequence: sequence.split(",").map(|s| s.parse().unwrap()).collect(),
                needed:   sequence.split(",").map(|s| s.parse::<usize>().unwrap()).sum(),
            }
        })
        .collect();
    Ok(records)
}

pub fn part_1(input: &Vec<Record>) -> Option<usize> {
    let mut count = 0;
    for (_i, record) in input.iter().enumerate() {
        let valid = record.total_permutations();
        count += valid;
    }
    Some(count)
}

pub fn part_2(input: &Vec<Record>) -> Option<usize> {
    let mut count = 0;
    for (_i, record) in input.iter().enumerate() {

        let start = SystemTime::now();

        let valid = record.to_long().total_permutations();
        count += valid;

        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        println!("{_i}: {:-<25}> {valid: >2}  {}.{}s", record.pattern, duration.as_secs(), duration.as_millis());
    }
    Some(count)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day12-example.txt") {
            assert_eq!(part_1(&input), Some(21))
        }
    }

    #[test]
    fn test_part_1_parts() {
        if let Ok(input) = prepare("day12-example.txt") {
            assert_eq!(input[0].total_permutations(),  1);
            assert_eq!(input[1].total_permutations(),  4);
            assert_eq!(input[2].total_permutations(),  1);
            assert_eq!(input[3].total_permutations(),  1);
            assert_eq!(input[4].total_permutations(),  4);
            assert_eq!(input[5].total_permutations(), 10);
        }
    }

    #[test]
    fn test_part_1_puzzle() {
        if let Ok(input) = prepare("day12.txt") {
            assert_eq!(part_1(&input), Some(7771))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day12-example.txt") {
            assert_eq!(part_2(&input), Some(525152))
        }
    }

    #[test]
    #[ignore]
    fn test_part_2_puzzle() {
        if let Ok(input) = prepare("day12.txt") {
            assert_eq!(part_2(&input), Some(0))
        }
    }
}