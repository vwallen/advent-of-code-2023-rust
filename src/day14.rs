use std::collections::HashSet;
use crate::read_input_lines;
use anyhow::Result;
use colored::Colorize;

pub fn prepare(file_name: &str) -> Result<Vec<Vec<char>>> {
    let input = read_input_lines(file_name);
    let mut grid:Vec<Vec<char>> = vec![];
    for (y, line) in input.iter().enumerate() {
        if grid.len() == 0 {
            grid = vec![vec!['.'; line.len()]; input.len()];
        }
        for (x, ch) in line.chars().enumerate() {
            grid[x][y] = ch;
        }
    }
    Ok(grid)
}

pub fn rotate_map(map:&Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut grid:Vec<Vec<char>> = vec![];
    let h = map.len();
    for (x, line) in map.iter().enumerate() {
        if grid.len() == 0 {
            grid = vec![vec!['.'; line.len()]; map.len()];
        }
        for (y, ch) in line.iter().enumerate() {
            grid[h - (y + 1)][x] = ch.clone();
        }
    }
    grid
}

pub fn settle_map(map:&Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut settled = map.clone();
    for col in settled.iter_mut() {
        col.split_mut(|ch| ch == &'#')
            .for_each(|segment| {
                segment.sort_by(|a, b| b.partial_cmp(&a).unwrap())
            });
    }
    settled
}

pub fn cycle_map(map:&Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = settle_map(&map);
    let w = settle_map(&rotate_map(&n));
    let s = settle_map(&rotate_map(&w));
    rotate_map(&settle_map(&rotate_map(&s)))
}

pub fn weigh_map(map:&Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for col in map.iter() {
        for (i, ch) in col.iter().enumerate() {
            if ch == &'O' {
                count += col.len() - i;
            }
        }
    }
    count
}

pub fn print_map(map:&Vec<Vec<char>>) {
    for (_x, col) in map.iter().enumerate() {
        print!("\n\t{}", " ".on_cyan());
        for (_y, ch) in col.iter().enumerate() {
            match ch {
                'O' => print!("{}", "● ".bright_white().on_cyan()),
                '#' => print!("{}", "■ ".black().on_cyan()),
                 _  => print!("{}", "  ".on_cyan()),
            };
        }
    }
    print!("\n\n");
}

pub fn part_1(input: &Vec<Vec<char>>) -> Option<usize> {
    let grid:Vec<Vec<char>> = settle_map(&input);
    let count = weigh_map(&grid);
    Some(count)
}

pub fn part_2(input: &Vec<Vec<char>>) -> Option<usize> {

    let mut map = input.clone();
    let mut map_set:HashSet<Vec<Vec<char>>> = HashSet::new();

    map_set.insert(map.clone());

    let mut count  = 0;
    let mut start  = 0;
    let mut period = 0;

    let cycles = 1_000_000_000;

    // find the starting point and period
    // at which the map state repeats
    for i in 0..cycles {
        map = cycle_map(&map);
        count = weigh_map(&map);
        if map_set.contains(&map) {
            if start > 0 {
                period = i - start;
                break;
            }
            map_set.clear();
            start = i;
        }
        map_set.insert(map.clone());
    }

    // start again, looping only for the remainder
    // of the loops needed to get to a billion
    map = input.clone();
    for _ in 0..(start + ((cycles - start) % period)) {
        map = cycle_map(&map);
        count = weigh_map(&map);
    }

    Some(count)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[ignore]
    fn test_prepare() {
        if let Ok(input) = prepare("day14-example.txt") {
            print_map(&input);
        }
    }

    #[test]
    #[ignore]
    fn test_rotate() {
        if let Ok(input) = prepare("day14-example.txt") {
            let mut map = input.clone();
            print_map(&map);
            for _ in 0..3 {
                map = cycle_map(&map);
                print_map(&map);
            }
        }
    }

    #[test]
    #[ignore]
    fn test_draw_map() {
        if let Ok(input) = prepare("day14.txt") {
            print_map(&input);
        }
    }

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day14-example.txt") {
            assert_eq!(part_1(&input), Some(136))
        }
    }

    #[test]
    fn test_part_1_puzzle() {
        if let Ok(input) = prepare("day14.txt") {
            assert_eq!(part_1(&input), Some(112046))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day14-example.txt") {
            assert_eq!(part_2(&input), Some(64))
        }
    }

    #[test]
    fn test_part_2_puzzle() {
        if let Ok(input) = prepare("day14.txt") {
            assert_eq!(part_2(&input), Some(104619))
        }
    }
}