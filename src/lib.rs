#[macro_use] extern crate sscanf;

use std::env;
use std::fs;

pub mod util;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;

pub fn read_input(file_name: &str) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("input").join(file_name);
    fs::read_to_string(filepath).expect("Unable to open input file")
}

pub fn read_input_lines(file_name: &str) -> Vec<String> {
    read_input(file_name).lines().map(String::from).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = read_input("test.txt");
        assert_eq!(input.lines().next().unwrap(), "I am a test")
    }

    #[test]
    fn test_read_input_lines() {
        let input = read_input_lines("test.txt");
        assert_eq!(input[0], "I am a test")
    }
}