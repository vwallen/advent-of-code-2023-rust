
use adventofcode_2023::read_input_lines;
use anyhow::Result;

pub fn prepare(file_name: &str) -> Result<Vec<String>> {
    let input = read_input_lines(file_name);
    Ok(input)
}

pub fn part_1(_input: &Vec<String>) -> Option<usize> {
    None
}

pub fn part_2(_input: &Vec<String>) -> Option<usize> {
    None
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[ignore]
    fn test_part_1() {
        if let Ok(input) = prepare("day01-example.txt") {
            assert_eq!(part_1(&input), Some(1))
        }
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        if let Ok(input) = prepare("day01-example.txt") {
            assert_eq!(part_2(&input), Some(1))
        }
    }
}