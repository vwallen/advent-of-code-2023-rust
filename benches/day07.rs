use adventofcode_2023::day07::*;

const INPUT:&str = "day07.txt";

fn main() {
    divan::main();
}

#[divan::bench(name="part 1")]
fn bench_part_1() {
    if let Ok(mut input) = prepare(INPUT) {
        part_1(&mut input);
    }
}

#[divan::bench(name="part 2")]
fn bench_part_2() {
    if let Ok(mut input) = prepare(INPUT) {
        part_2(&mut input);
    }
}