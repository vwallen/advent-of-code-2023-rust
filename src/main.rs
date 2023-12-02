use colored::Colorize;

pub mod day01;

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
        println!("🎁 {}", day01_part1.unwrap());
        println!("🎁 {}", day01_part2.unwrap());
    }

    println!("{} {} {}", "----------".red(), "Day  2".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day  3".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day  4".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day  5".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day  6".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day  7".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day  8".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day  9".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day 10".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day 11".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day 12".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day 13".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day 14".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day 15".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day 16".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day 17".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day 18".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day 19".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day 20".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day 21".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day 22".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day 23".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day 24".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{} {} {}", "----------".red(), "Day 25".bright_green(), "----------".red());
    println!("🎁");
    println!("🎁");

    println!("{}", "============================".bright_red());
}
