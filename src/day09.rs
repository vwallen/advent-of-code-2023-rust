
use adventofcode_2023::read_input_lines;
use anyhow::Result;
use itertools::Itertools;

pub fn prepare(file_name: &str) -> Result<Vec<Vec<isize>>> {
    let input = read_input_lines(file_name);
    let output = input
        .iter()
        .map(|line| {
            line.split(" ")
                .map(|w| w.parse().unwrap())
                .collect()
        })
        .collect();
    Ok(output)
}

pub fn extrapolate(input: &Vec<Vec<isize>>) -> Option<(isize, isize)> {
    let mut tails:Vec<isize> = vec![];
    let mut heads:Vec<isize> = vec![];
    let mut pyramid:Vec<Vec<isize>> = vec![];
    for row in input.iter() {
        let mut pyramid_row = row.clone();
        while pyramid_row.iter().any(|n| *n != 0) {
            pyramid.push(pyramid_row.clone());
            pyramid_row = pyramid_row
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect();
        }
        tails.push(
            pyramid
                .iter()
                .map(|row| row.last().unwrap())
                .sum()
        );
        heads.push(
            pyramid
                .iter()
                .rev()
                .map(|row| *row.first().unwrap())
                .reduce(|a, b| {
                    b - a
                })
                .unwrap()
        );
        pyramid.clear();
    }
    Some((heads.iter().sum(), tails.iter().sum()))
}

pub fn part_1(input: &Vec<Vec<isize>>) -> Option<isize> {
    if let Some((_, tails)) = extrapolate(input) {
        Some(tails)
    } else { None }
}

pub fn part_2(input: &Vec<Vec<isize>>) -> Option<isize> {
    if let Some((heads, _)) = extrapolate(input) {
        Some(heads)
    } else { None }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day09-example.txt") {
            assert_eq!(part_1(&input), Some(114))
        }
    }

    #[test]
    #[ignore]
    fn test_part_1_puzzle() {
        if let Ok(input) = prepare("day09.txt") {
            assert_eq!(part_1(&input), Some(2008960228))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day09-example.txt") {
            assert_eq!(part_2(&input), Some(2))
        }
    }

    #[test]
    #[ignore]
    fn test_part_2_puzzle() {
        if let Ok(input) = prepare("day09.txt") {
            assert_eq!(part_2(&input), Some(1097))
        }
    }
}