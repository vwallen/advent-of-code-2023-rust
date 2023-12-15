use indexmap::IndexMap;
use crate::read_input_lines;
use anyhow::Result;

#[derive(Debug)]
pub enum LensOp {
    Remove((usize, String)),
    Insert((usize, String, usize)),
}

pub fn prepare(file_name: &str) -> Result<Vec<String>> {
    let output = read_input_lines(file_name).first().unwrap().split(",").map(str::to_string).collect();
    Ok(output)
}

pub fn hash_seq(seq:&str) -> usize {
    seq.chars().fold(0, |acc, ch| {
        (((ch.to_ascii_lowercase() as usize) + acc) * 17) % 256
    })
}

pub fn part_1(input: &Vec<String>) -> Option<usize> {
    let output = input.iter().fold(0, |acc, seq| acc + hash_seq(seq));
    Some(output)
}

pub fn part_2(input: &Vec<String>) -> Option<usize> {

    // parse the sequence into ops
    let steps:Vec<LensOp> = input
        .iter()
        .map(|seq| {
            if let Some((label, focal_length)) = seq.split_once('=') {
                LensOp::Insert((hash_seq(&label), label.to_string(), focal_length.parse().unwrap()))
            } else {
                let label = seq.replace("-", "");
                LensOp::Remove((hash_seq(&label), label))
            }
        })
        .collect();

    // get thee a hashmap of hashmaps
    // use IndexMap to preserve insertion order
    let mut boxes:IndexMap<usize, IndexMap<String, usize>> = IndexMap::new();

    // process the steps
    for step in steps.iter() {
        match step {
            LensOp::Remove((id, label)) => {
                if let Some(lenses) = boxes.get_mut(id) {
                    lenses.shift_remove(label);
                }
            },
            LensOp::Insert((id, label, focal_length)) => {
                if let Some(lenses) = boxes.get_mut(id) {
                    lenses.insert(label.clone(), focal_length.clone());
                } else {
                    boxes.insert(id.clone(), IndexMap::from([(label.clone(), focal_length.clone())]));
                }
            }
        }
    }

    let total:usize = boxes
        .iter()
        .fold(0, |total, (box_index, lenses)| {
            total + lenses
                .iter()
                .enumerate()
                .fold(0, |total, (slot_index, (_, focal_length))| {
                    total + (box_index + 1) * (slot_index + 1) * focal_length
                })
        });

    Some(total)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash_seq("rn=1"), 30);
        assert_eq!(hash_seq("ab=5"), 197);
        assert_eq!(hash_seq("qp-"),  14);
    }

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day15-example.txt") {
            assert_eq!(part_1(&input), Some(1320))
        }
    }

    #[test]
    #[ignore]
    fn test_part_1_puzzle() {
        if let Ok(input) = prepare("day15.txt") {
            assert_eq!(part_1(&input), Some(515210))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day15-example.txt") {
            assert_eq!(part_2(&input), Some(145))
        }
    }

    #[test]
    #[ignore]
    fn test_part_2_puzzle() {
        if let Ok(input) = prepare("day15.txt") {
            assert_eq!(part_2(&input), Some(246762))
        }
    }
}