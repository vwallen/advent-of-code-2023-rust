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
                        if let Some(a) = self.check_partial(opt.clone() + &".") {
                            opts.push(a);
                        }
                        if let Some(b) = self.check_partial(opt.clone() + &"#") {
                            opts.push(b);
                        }
                        opts
                    })
                },
                _ => unreachable!(),
            }
        }
        options
    }

    pub fn check_partial(&self, permutation:String) -> Option<String> {
        let sequence:Vec<usize> = permutation
            .split(".")
            .filter_map(|s| if s.is_empty() { None } else { Some(s.len())})
            .collect();
        for (i, block) in sequence.iter().enumerate() {
            if let Some(requirement) = self.sequence.get(i) {
                if block > &requirement {
                    return None
                }
            }
        }
        Some(permutation.clone())
    }

    pub fn check_permutation(&self, permutation:&String) -> bool {
        let sequence:Vec<usize> = permutation
            .split(".")
            .filter_map(|s| if s.is_empty() { None } else { Some(s.len())})
            .collect();
        sequence == self.sequence
    }

    pub fn to_long(&self) -> Record {
        Record {
            pattern:  vec![self.pattern.clone(); 5].join("?"),
            sequence: vec![self.sequence.clone(); 5].concat(),
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
            }
        })
        .collect();
    Ok(records)
}

pub fn part_1(input: &Vec<Record>) -> Option<usize> {
    let mut count = 0;
    for (_i, record) in input.iter().enumerate() {
        let valid = record
            .collect_permutations()
            .iter()
            .filter(|perm| record.check_permutation(perm))
            .count();
        count += valid;
        // println!("{i}: {:-<25}> {valid: >2}", record.pattern);
    }
    Some(count)
}

pub fn part_2(input: &Vec<Record>) -> Option<usize> {
    let mut count = 0;
    for (_i, record) in input.iter().enumerate() {
        let long_record = record.to_long();
        let valid = long_record
            .collect_permutations()
            .iter()
            .filter(|perm| long_record.check_permutation(perm))
            .count();
        count += valid;
        // println!("{i}: {:?} -> {valid}", long_record.pattern);
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
    #[ignore]
    fn test_part_1_puzzle() {
        if let Ok(input) = prepare("day12.txt") {
            assert_eq!(part_1(&input), Some(7771))
        }
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        if let Ok(input) = prepare("day12-example.txt") {
            assert_eq!(part_2(&input), Some(525152))
        }
    }
}