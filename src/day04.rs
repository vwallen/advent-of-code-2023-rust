
use crate::read_input_lines;
use anyhow::Result;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
pub struct Card {
    winners: HashSet<usize>,
    numbers_held: HashSet<usize>,
}
impl Card {
    fn matches(&self) -> usize {
        self.winners.intersection(&self.numbers_held).count()
    }

    fn score(&self) -> usize {
        let count = self.matches() as u32;
        if count > 0 {
            2_usize.pow(count - 1)
        } else { 0 }
    }
}
impl FromStr for Card {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Sample: "Card  1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
        let (_, values) = s.trim().split_once(":").unwrap();
        let (winner_string, number_string) = values.split_once("|").unwrap();
        let winners:HashSet<usize> = winner_string
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();
        let numbers_held:HashSet<usize> = number_string
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();
        Ok(Card {
            winners,
            numbers_held,
        })
    }
}

pub fn prepare(file_name: &str) -> Result<Vec<Card>> {
    let input = read_input_lines(file_name);
    let mut cards:Vec<Card> = Vec::new();
    for (_, line) in input.iter().enumerate() {
        cards.push(line.parse()?)
    }
    Ok(cards)
}

pub fn part_1(cards: &Vec<Card>) -> Option<usize> {
    let total = cards.iter().map(|c| {
        c.score()
    }).sum();
    Some(total)
}

pub fn part_2(cards: &Vec<Card>) -> Option<usize> {
    let mut won_cards:Vec<usize> = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let start = i + 1;
        let bound = i + card.matches();
        for j in start..=bound {
            won_cards[j] += won_cards[i];
        }
    }
    Some(won_cards.iter().sum::<usize>())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day04-example.txt") {
            assert_eq!(part_1(&input), Some(13))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day04-example.txt") {
            assert_eq!(part_2(&input), Some(30))
        }
    }
}