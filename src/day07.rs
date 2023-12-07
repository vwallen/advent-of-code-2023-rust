use std::str::FromStr;
use adventofcode_2023::read_input_lines;
use anyhow::Result;
use counter::Counter;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Hand {
    code: String,
    cards: Vec<(char, usize)>,
    priority: ([u8; 5], [u8; 5]),
    priority_wild: ([u8; 5], [u8; 5]),
    wager: usize,
}
impl FromStr for Hand {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_str, wager_str) = s.split_once(" ").unwrap();

        // the hand encoded as given, for identification
        let code:String = hand_str.to_string();

        // the hand as cards types, with count
        let counter = hand_str
            .chars()
            .collect::<Counter<_>>();

        let cards:Vec<(char, usize)> = counter.most_common_ordered();
        let jokers:u8 = counter[&'J'].try_into().unwrap();

        // the priority of the hands, based on number of cards
        // (5,0,0,0,0) -> 5 of a kind
        // (4,1,0,0,0) -> 4 of a kind
        // (3,2,0,0,0) -> full house
        // (2,2,1,0,0) -> two pair
        // (2,1,1,1,0) -> one pair
        // (1,1,1,1,1) -> high card
        let mut hand_priority:Vec<u8> = cards
            .iter()
            .map(|(_, b)| *b as u8)
            .collect();
        // pad the remainder with 0s
        hand_priority.resize(5, 0);

        // the priority of the cards as a hand, in order (to break ties)
        let mut card_priority:Vec<u8> = code
            .chars()
            .map(|c| {
                match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    c  => c.to_digit(10).unwrap() as u8,
                }
            }).collect();

        // priority is combined hand priority and card priority
        let priority:([u8; 5], [u8; 5]) = (
            hand_priority.try_into().unwrap(),
            card_priority.try_into().unwrap(),
        );

        // recreate the priority without the jokers
        hand_priority = cards
            .iter()
            .filter(|(a, _)| a != &'J')
            .map(|(_, b)| *b as u8)
            .collect();
        hand_priority.resize(5, 0);
        hand_priority[0] += jokers; // add joker to highest count card

        // reset the card priority for Jokers to 1
        card_priority = priority.1
            .iter()
            .map(|n| {
                match n {
                    11 => 1,
                    n  => *n,
                }
            }).collect();

        let priority_wild:([u8; 5], [u8; 5]) = (
            hand_priority.try_into().unwrap(),
            card_priority.try_into().unwrap(),
        );

        let wager = wager_str.parse().unwrap();

        Ok(Hand{code, cards, priority, priority_wild, wager})
    }
}

pub fn prepare(file_name: &str) -> Result<Vec<Hand>> {
    let input = read_input_lines(file_name);
    let hands:Vec<Hand> = input
        .iter()
        .filter_map(|line| line.parse().ok())
        .collect();
    Ok(hands)
}

pub fn part_1(hands: &mut Vec<Hand>) -> Option<usize> {
    hands.sort_by(|a, b| a.priority.cmp(&b.priority));
    let total = hands
        .iter().enumerate()
        .fold(0, |t, (k,v)| {
            t + v.wager * (k + 1)
        });

    Some(total)
}

pub fn part_2(hands: &mut Vec<Hand>) -> Option<usize> {
    hands.sort_by(|a, b| a.priority_wild.cmp(&b.priority_wild));
    let total = hands
        .iter().enumerate()
        .fold(0, |t, (k,v)| {
            t + v.wager * (k + 1)
        });

    Some(total)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        if let Ok(mut hands) = prepare("day07-example.txt") {
            hands.sort_by(|a, b| a.priority.cmp(&b.priority));
            assert_eq!(hands.len(), 5);
            assert_eq!(hands[0].code, "32T3K");
            assert_eq!(hands[1].code, "KTJJT");
            assert_eq!(hands[2].code, "KK677");
            assert_eq!(hands[3].code, "T55J5");
            assert_eq!(hands[4].code, "QQQJA");

            hands.sort_by(|a, b| a.priority_wild.cmp(&b.priority_wild));
            assert_eq!(hands.len(), 5);
            assert_eq!(hands[0].code, "32T3K");
            assert_eq!(hands[1].code, "KK677");
            assert_eq!(hands[2].code, "T55J5");
            assert_eq!(hands[3].code, "QQQJA");
            assert_eq!(hands[4].code, "KTJJT");
        }
    }

    #[test]
    fn test_wild () {
        let hand1:Hand = "KTJJT 0".parse().unwrap();
        assert_eq!(hand1.priority, ([2, 2, 1, 0, 0], [13, 10, 11, 11, 10]));
        assert_eq!(hand1.priority_wild, ([4, 1, 0, 0, 0], [13, 10, 1, 1, 10]));

        let hand2:Hand = "JA234 0".parse().unwrap();
        assert_eq!(hand2.priority, ([1, 1, 1, 1, 1], [11, 14, 2, 3, 4]));
        assert_eq!(hand2.priority_wild, ([2, 1, 1, 1, 0], [1, 14, 2, 3, 4]));
    }

    #[test]
    fn test_part_1() {
        if let Ok(mut hands) = prepare("day07-example.txt") {
            assert_eq!(part_1(&mut hands), Some(6440))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(mut hands) = prepare("day07-example.txt") {
            assert_eq!(part_2(&mut hands), Some(5905))
        }
    }
}

// 248267890 is too high