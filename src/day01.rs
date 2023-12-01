use std::collections::HashMap;
use adventofcode_2023::read_input_lines;
use anyhow::Result;

pub fn prepare(file_name: &str) -> Result<Vec<String>> {
    let input = read_input_lines(file_name);
    Ok(input)
}

pub fn part_1(_input: &Vec<String>) -> Option<u32> {
    let mut total:u32 = 0;
    for line in _input.iter() {
        let digits:Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        total += 10 * first + last
    }
    Some(total)
}

pub fn part_2(_input: &Vec<String>) -> Option<u32> {
    let digit_map:HashMap<&str, u32> = HashMap::from([
        ("1", 1), ("one", 1),
        ("2", 2), ("two", 2),
        ("3", 3), ("three", 3),
        ("4", 4), ("four", 4),
        ("5", 5), ("five", 5),
        ("6", 6), ("six", 6),
        ("7", 7), ("seven", 7),
        ("8", 8), ("eight", 8),
        ("9", 9), ("nine", 9),
    ]);
    let mut total:u32 = 0;
    for line in _input.iter() {
        let mut digits:Vec<u32> = vec![0; line.len()];
        for code in digit_map.keys() {
            for (index, _) in line.match_indices(code) {
                digits[index] = *digit_map.get(code).unwrap();
            }
        }
        let first = digits.iter().find(|&&n| n != 0).unwrap();
        let last = digits.iter().rfind(|&&n| n != 0).unwrap();
        total += 10 * first + last;
    }
    Some(total)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day01-example-1.txt") {
            assert_eq!(part_1(&input), Some(142))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day01-example-2.txt") {
            assert_eq!(part_2(&input), Some(281))
        }
    }
}
