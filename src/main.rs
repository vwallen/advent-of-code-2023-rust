#[macro_use] extern crate sscanf;
use colored::Colorize;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

// 🎁 == pending
// 🌟 == complete
// ❄️ == incomplete

fn main() {
    println!("{}", "\n\n🎄🎄🎄🎄 Advent of Code 2022 🎄🎄🎄🎄".bright_red());

    println!("{} {} {}", "----------".red(), "Day  1".bright_green(), "----------".red());
    println!("\tTrebuchet?!");
    if let Ok(day01_input) = day01::prepare("day01.txt") {
        let day01_part1 = day01::part_1(&day01_input);
        let day01_part2 = day01::part_2(&day01_input);
        println!("🌟 {}", day01_part1.unwrap());
        println!("🌟 {}", day01_part2.unwrap());
    }

    println!("{} {} {}", "----------".red(), "Day  2".bright_green(), "----------".red());
    println!("\tCube Conundrum");
    if let Ok(day02_input) = day02::prepare("day02.txt") {
        let day02_part1 = day02::part_1(&day02_input);
        let day02_part2 = day02::part_2(&day02_input);
        println!("🌟 {}", day02_part1.unwrap());
        println!("🌟 {}", day02_part2.unwrap());
    }

    println!("{} {} {}", "----------".red(), "Day  3".bright_green(), "----------".red());
    println!("\tGear Ratios");
    if let Ok(day03_input) = day03::prepare("day03.txt") {
        let day03_part1 = day03::part_1(&day03_input);
        let day03_part2 = day03::part_2(&day03_input);
        println!("🌟 {}", day03_part1.unwrap_or(0));
        println!("🌟 {}", day03_part2.unwrap_or(0));
    }
    println!("{} {} {}", "----------".red(), "Day  4".bright_green(), "----------".red());
    println!("\tScratchcards");
    if let Ok(day04_input) = day04::prepare("day04.txt") {
        let day04_part1 = day04::part_1(&day04_input);
        let day04_part2 = day04::part_2(&day04_input);
        println!("🎁 {}", day04_part1.unwrap_or(0));
        println!("🎁 {}", day04_part2.unwrap_or(0));
    }
    println!("{}", "============================".bright_red());
}
