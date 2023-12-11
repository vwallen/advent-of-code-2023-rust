use std::collections::HashMap;
use crate::read_input_lines;
use anyhow::Result;
use itertools::Itertools;

pub fn prepare(file_name: &str) -> Result<HashMap<(isize,isize), (isize,isize)>> {
    let input = read_input_lines(file_name);
    let mut y_offset:isize = 0;
    let mut x_empty:Vec<isize> = Vec::new();
    let mut galaxies:HashMap<(isize,isize), (isize,isize)> = HashMap::new();

    // gather up the galaxies, tracking the y offset and
    // using a place holder for the x offset
    // track which columns are empty
    for (y, line) in input.iter().enumerate() {
        if y == 0 {
            // fill the x_empty vec with 1s
            // equal to the width of the map
            x_empty.resize(line.len(), 1);
        }
        let mut y_increment = 1;
        for (x, _) in line.match_indices("#") {
            y_increment = 0; // non-empty row, do not increment y offset
            x_empty[x]  = 0; // non-empty col, do not increment x offset
            galaxies.insert((x as isize, y as isize), (0, y_offset));
        }
        y_offset += y_increment
    }

    // calculate x offsets based on empty cols
    let x_offsets:Vec<isize> = x_empty
        .iter()
        .enumerate()
        .map(|(i, _)| x_empty[0..i].iter().sum()) // x offset = sum of non-empty cols
        .collect();

    // remap the galaxy positions and offsets
    // including the gathered x offset information
    let mut galaxy_map:HashMap<(isize,isize), (isize,isize)> = HashMap::new();
    for (galaxy, offsets) in galaxies.iter_mut() {
        galaxy_map.insert(
            galaxy.clone(),
            (x_offsets[galaxy.0 as usize], offsets.1.clone()),
        );
    }

    Ok(galaxy_map)
}

pub fn expand(galaxies: &HashMap<(isize,isize), (isize,isize)>, expand_by:isize) -> Result<Vec<(isize,isize)>> {
    // map the galaxy position and offset to
    // new positions based on expansion distance
    let galaxies_expanded:Vec<(isize,isize)> = galaxies
        .iter()
        .map(|(galaxy, offsets)|
            (galaxy.0 + offsets.0 * (expand_by - 1), // reduce expand by to account for existing col
             galaxy.1 + offsets.1 * (expand_by - 1)) // reduce expand by to account for existing row
        )
        .collect();
    Ok(galaxies_expanded)
}

pub fn count_distances(galaxies_expanded: &Vec<(isize,isize)>) -> isize {
    // total the cab distance for each combination
    // of expanded galaxy positions
    galaxies_expanded
        .iter()
        .combinations(2)
        .map(|pair| (pair[0].0 - pair[1].0).abs() + (pair[0].1 - pair[1].1).abs()) // x diff + y diff
        .sum()
}

pub fn part_1(galaxies: &HashMap<(isize,isize), (isize,isize)>) -> Option<isize> {
    let expand_by = 2;
    let distances:isize = count_distances(&expand(galaxies, expand_by).unwrap());
    Some(distances)
}

pub fn part_2(galaxies: &HashMap<(isize,isize), (isize,isize)>) -> Option<isize> {
    let expand_by = 1_000_000;
    let distances:isize = count_distances(&expand(galaxies, expand_by).unwrap());
    Some(distances)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day11-example.txt") {
            assert_eq!(part_1(&input), Some(374));
        }
    }

    #[test]
    fn test_part_1_puzzle() {
        if let Ok(input) = prepare("day11.txt") {
            assert_eq!(part_1(&input), Some(9556712));
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day11-example.txt") {
            // directly test smaller expansion distances
            assert_eq!(count_distances(&expand(&input, 10).unwrap()), 1030);
            assert_eq!(count_distances(&expand(&input, 100).unwrap()), 8410);
        }
    }

    #[test]
    fn test_part_2_puzzle() {
        if let Ok(input) = prepare("day11.txt") {
            assert_eq!(part_2(&input), Some(678626199476));
        }
    }
}