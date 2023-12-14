use crate::read_input_lines;
use anyhow::Result;
use cached::proc_macro::cached;

#[derive(Debug)]
pub struct Record {
    pattern: String,
    sequence: Vec<usize>,
    needed: usize,
}

pub fn to_long(record:&Record) -> Record {
    Record {
        pattern:  vec![record.pattern.clone(); 5].join("?"),
        sequence: vec![record.sequence.clone(); 5].concat(),
        needed:   vec![record.sequence.clone(); 5].concat().iter().sum(),
    }
}

pub fn total_permutations(record:&Record) -> usize {
    count_permutation(record.pattern.clone(), record.needed.clone(), record.sequence.clone(), 0, 0, 0, vec![0])
}

// without the function caching macro
// this would take ~100 hours :(
// all arguments must be immutable and passed by value
// for the caching to work
#[cached]
pub fn count_permutation(
    pattern:String,
    needed:usize,
    required:Vec<usize>,
    pos:usize,
    slots_used:usize,
    slots_filled:usize,
    mut sequence:Vec<usize>) -> usize
{
    // if there are no more positions to check
    // either we met requirements or not
    if pos == pattern.len() {
        // it's possible to end with a trailing 0 in the found sequence
        // if so, truncate the sequence
        if sequence.last().unwrap() == &0 { sequence.pop(); }
        return if slots_filled == needed && required.starts_with(&sequence) { 1 } else { 0 }
    };

    // get the current working grouping
    // pop the working value to make matching the sequences easier
    let curr:usize = sequence.pop().unwrap();

    // get the next token to match
    let next:char = pattern.chars().nth(pos).unwrap();

    match next {
        '#' => {
            // does the sequence so far match the required sequence?
            // then keep going, otherwise, no valid options here
            if required.starts_with(&sequence) {
                // add to the current working grouping
                sequence.push(curr + 1);
                count_permutation(pattern.clone(), needed.clone(), required.clone(), pos + 1, slots_used, slots_filled + 1, sequence.clone())
            } else { 0 }
        },

        '.' => {
            // if the current working group is not empty
            // stop adding to it, and start a new one
            sequence.push(curr);
            if curr > 0 {
                sequence.push(0);
            }
            count_permutation(pattern.clone(), needed.clone(), required.clone(), pos + 1, slots_used, slots_filled, sequence)
        },

        '?' => {
            // if a wild card, try both options
            let octothorpe = {
                let mut next_sequence = sequence.clone();
                // does the sequence so far match the required sequence?
                // then keep going, otherwise, no valid options here
                if required.starts_with(&sequence) {
                    // add to the current working grouping
                    next_sequence.push(curr + 1);
                    count_permutation(pattern.clone(), needed.clone(), required.clone(), pos + 1, slots_used, slots_filled + 1, next_sequence)
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
                count_permutation(pattern.clone(), needed.clone(), required.clone(), pos + 1, slots_used, slots_filled, next_sequence)
            };

            octothorpe + dot
        },

        _ => unreachable!(),
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
        let valid = total_permutations(&record);
        count += valid;
    }
    Some(count)
}

pub fn part_2(input: &Vec<Record>) -> Option<usize> {
    let mut count = 0;
    for (_i, record) in input.iter().enumerate() {
        let valid = total_permutations(&to_long(&record));
        count += valid;
    }
    Some(count)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part_1_parts() {
        if let Ok(input) = prepare("day12-example.txt") {
            assert_eq!(total_permutations(&input[0]),  1);
            assert_eq!(total_permutations(&input[1]),  4);
            assert_eq!(total_permutations(&input[2]),  1);
            assert_eq!(total_permutations(&input[3]),  1);
            assert_eq!(total_permutations(&input[4]),  4);
            assert_eq!(total_permutations(&input[5]), 10);
        }
    }

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day12-example.txt") {
            assert_eq!(part_1(&input), Some(21))
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
    fn test_part_2_puzzle() {
        if let Ok(input) = prepare("day12.txt") {
            assert_eq!(part_2(&input), Some(10_861_030_975_833))
        }
    }
}