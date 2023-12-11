use std::time::SystemTime;
use colored::Colorize;
use adventofcode_2023::*;

// 🎁 == pending
// 🌟 == complete
// ❄️ == incomplete

fn main() {
    println!("{}", "\n\n🎄🎄🎄🎄 Advent of Code 2022 🎄🎄🎄🎄".bright_red());

    println!("{} {} {}", "----------".red(), "Day  1".bright_green(), "----------".red());
    println!("\t🎁 Trebuchet?!");
    if let Ok(input) = day01::prepare("day01.txt") {
        let part1 = day01::part_1(&input);
        let part2 = day01::part_2(&input);
        println!("🌟 {}", part1.unwrap());
        println!("🌟 {}", part2.unwrap());
    }

    println!("{} {} {}", "----------".red(), "Day  2".bright_green(), "----------".red());
    println!("\t🎁 Cube Conundrum");
    if let Ok(input) = day02::prepare("day02.txt") {
        let part1 = day02::part_1(&input);
        let part2 = day02::part_2(&input);
        println!("🌟 {}", part1.unwrap());
        println!("🌟 {}", part2.unwrap());
    }

    println!("{} {} {}", "----------".red(), "Day  3".bright_green(), "----------".red());
    println!("\t🎁 Gear Ratios");
    if let Ok(input) = day03::prepare("day03.txt") {
        let part1 = day03::part_1(&input);
        let part2 = day03::part_2(&input);
        println!("🌟 {}", part1.unwrap_or(0));
        println!("🌟 {}", part2.unwrap_or(0));
    }

    println!("{} {} {}", "----------".red(), "Day  4".bright_green(), "----------".red());
    println!("\t🎁 Scratchcards");
    if let Ok(input) = day04::prepare("day04.txt") {
        let part1 = day04::part_1(&input);
        let part2 = day04::part_2(&input);
        println!("🌟 {}", part1.unwrap_or(0));
        println!("🌟 {}", part2.unwrap_or(0));
    }

    println!("{} {} {}", "----------".red(), "Day  5".bright_green(), "----------".red());
    println!("\t🎁 If You Give a Seed a Fertilizer");
    if let Ok(input) = day05::prepare("day05.txt") {
        let part1 = day05::part_1(&input);

        let start = SystemTime::now();
        let part2 = day05::part_2(&input);
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();

        println!("🌟 {}", part1.unwrap_or(0));
        println!("🌟 {}, {}", part2.unwrap_or(0), format!("{}µ", duration.as_micros()).bright_green());
    }

    println!("{} {} {}", "----------".red(), "Day  6".bright_green(), "----------".red());
    println!("\t🎁 Wait For It!");
    if let Ok(input) = day06::prepare_1("day06.txt") {
        let part1 = day06::part_1(&input);
        println!("🌟 {}", part1.unwrap_or(0));
    }
    if let Ok(input) = day06::prepare_2("day06.txt") {
        let part2 = day06::part_2(&input);
        println!("🌟 {}", part2.unwrap_or(0));
    }

    println!("{} {} {}", "----------".red(), "Day  7".bright_green(), "----------".red());
    println!("\t🎁 Camel Cards");
    if let Ok(mut input) = day07::prepare("day07.txt") {
        let part1 = day07::part_1(&mut input);
        let part2 = day07::part_2(&mut input);
        println!("🌟 {}", part1.unwrap_or(0));
        println!("🌟 {}", part2.unwrap_or(0));
    }

    println!("{} {} {}", "----------".red(), "Day  8".bright_green(), "----------".red());
    println!("\t🎁 Haunted Wasteland");
    if let Ok(input) = day08::prepare("day08.txt") {
        let part1 = day08::part_1(&input);
        let part2 = day08::part_2(&input);
        println!("🌟 {}", part1.unwrap_or(0));
        println!("🌟 {}", part2.unwrap_or(0));
    }

    println!("{} {} {}", "----------".red(), "Day  9".bright_green(), "----------".red());
    println!("\t🎁 Mirage Maintenance");
    if let Ok(input) = day09::prepare("day09.txt") {
        let part1 = day09::part_1(&input);
        let part2 = day09::part_2(&input);
        println!("🌟 {}", part1.unwrap_or(0));
        println!("🌟 {}", part2.unwrap_or(0));
    }

    println!("{} {} {}", "----------".red(), "Day 10".bright_green(), "----------".red());
    println!("\t🎁 Mirage Maintenance");
    if let Ok(input) = day10::prepare("day10.txt") {
        let part1 = day10::part_1(&input);
        let part2 = day10::part_2(&input);
        println!("🌟 {}", part1.unwrap_or(0));
        println!("🌟 {}", part2.unwrap_or(0));
    }

    println!("{} {} {}", "----------".red(), "Day 11".bright_green(), "----------".red());
    println!("\t🎁 Mirage Maintenance");
    if let Ok(input) = day11::prepare("day11.txt") {
        let part1 = day11::part_1(&input);
        let part2 = day11::part_2(&input);
        println!("🌟 {}", part1.unwrap_or(0));
        println!("🌟 {}", part2.unwrap_or(0));
    }

    println!("{}", "============================".bright_red());
}
