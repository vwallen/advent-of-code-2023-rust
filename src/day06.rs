
use adventofcode_2023::read_input_lines;
use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
pub struct Race {
    time:usize,
    record:usize,
}
impl Race {
    fn run(&self, pressed:usize) -> usize {
        if pressed >= self.time { 0 }
        else {
            pressed * (self.time - pressed)
        }
    }

    fn find_wins(&self) -> Vec<usize> {
        (0..=self.time)
            .map(|n| self.run(n))
            .filter(|n| *n > self.record)
            .collect()
    }
}


pub fn prepare_1(file_name: &str) -> Result<Vec<Race>> {
    let input = read_input_lines(file_name);
    let times:Vec<usize>   = input[0].split_once(":").unwrap().1.split_whitespace().filter_map(|n|n.parse().ok()).collect();
    let records:Vec<usize> = input[1].split_once(":").unwrap().1.split_whitespace().filter_map(|n|n.parse().ok()).collect();
    let races:Vec<Race>    = times.iter().enumerate().map(|(i, time)| Race {time:*time, record:records[i]}).collect();
    Ok(races)
}

pub fn prepare_2(file_name: &str) -> Result<Race> {
    let input = read_input_lines(file_name);
    let time:usize   = input[0].split_once(":").unwrap().1.split_whitespace().join("").parse().unwrap();
    let record:usize = input[1].split_once(":").unwrap().1.split_whitespace().join("").parse().unwrap();
    Ok(Race{time, record})
}

pub fn part_1(races: &Vec<Race>) -> Option<usize> {
    let wins:Vec<usize> = races.iter().map(|r| r.find_wins().len()).collect();
    Some(wins.iter().product())
}

pub fn part_2(race: &Race) -> Option<usize> {
    Some(race.find_wins().len())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare_1("day06-example.txt") {
            assert_eq!(part_1(&input), Some(288))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare_2("day06-example.txt") {
            assert_eq!(part_2(&input), Some(71503))
        }
    }
}