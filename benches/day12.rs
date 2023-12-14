use adventofcode_2023::day12::*;

const INPUT:&str = "day12.txt";

fn main() {
    divan::main();
}

#[divan::bench(name="part 1")]
fn bench_part_1() {
    if let Ok(input) = prepare(INPUT) {
        part_1(&input);
    }
}

#[divan::bench(name="part 2 (sample)", sample_size=1, sample_count=10)]
fn bench_part_2() {
    if let Ok(input) = prepare("day12-example.txt") {
        part_2(&input);
    }
}