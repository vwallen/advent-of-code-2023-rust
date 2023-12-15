use adventofcode_2023::day15::*;

const INPUT:&str = "day15.txt";

fn main() {
    divan::main();
}

#[divan::bench(name="part 1")]
fn bench_part_1() {
    if let Ok(input) = prepare(INPUT) {
        part_1(&input);
    }
}

#[divan::bench(name="part 2")]
fn bench_part_2() {
    if let Ok(input) = prepare(INPUT) {
        part_2(&input);
    }
}