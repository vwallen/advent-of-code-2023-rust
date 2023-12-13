use crate::read_input_lines;
use anyhow::Result;

#[derive(Debug)]
pub struct Record {
    pattern: String,
    sequence: Vec<usize>,
}
impl Record {

    pub fn collect_permutations(&self) -> Vec<String> {
        let mut options:Vec<String> = vec!["".to_string()];
        for ch in self.pattern.chars() {
            options = match ch {
                '.'|'#' => options.iter().map(|opt| opt.clone() + &ch.to_string()).collect(),
                '?' => {
                    options.iter().fold(vec![], |mut opts, opt| {
                        opts.push(opt.clone() + &".");
                        opts.push(opt.clone() + &"#");
                        opts
                    })
                },
                _ => unreachable!(),
            }
            .iter()
            .filter(|opt| self.check_partial(opt))
            .map(String::clone)
            .collect();
        }
        options
    }

    pub fn check_partial(&self, permutation:&String) -> bool {
        let sequence:Vec<usize> = permutation
            .split(".")
            .map(|s| s.len())
            .filter(|s| s > &0_usize)
            .collect();
        for (i, block) in sequence.iter().enumerate() {
            if let Some(requirement) = self.sequence.get(i) {
                if block > &requirement {
                    return false
                }
            }
        }
        true
    }

    pub fn check_permutation(&self, permutation:&String) -> bool {
        let sequence:Vec<usize> = permutation
            .split(".")
            .map(|s| s.len())
            .filter(|s| s > &0_usize)
            .collect();
        sequence == self.sequence
    }
}

pub fn prepare(file_name: &str) -> Result<Vec<Record>> {
    let input = read_input_lines(file_name);
    let records:Vec<Record> = input
        .iter()
        .map(|line| {
            let (pattern, sequence) = line.split_once(" ").unwrap();
            Record{
                pattern: pattern.to_string(),
                sequence: sequence.split(",").map(|s| s.parse().unwrap()).collect(),
            }
        })
        .collect();
    Ok(records)
}

pub fn part_1(input: &Vec<Record>) -> Option<usize> {
    let mut count = 0;
    for record in input.iter() {
        count += record
            .collect_permutations()
            .iter()
            .filter(|perm| record.check_permutation(&perm))
            .count();
    }
    Some(count)
}

pub fn part_2(_input: &Vec<Record>) -> Option<usize> {
    None
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
    fn test_part_1_puzzle() {
        if let Ok(input) = prepare("day12.txt") {
            assert_eq!(part_1(&input), Some(7771))
        }
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        if let Ok(input) = prepare("day12-example.txt") {
            assert_eq!(part_2(&input), None)
        }
    }
}