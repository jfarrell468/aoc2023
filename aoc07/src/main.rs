use std::cmp::Ordering;
use std::collections::HashMap;

use anyhow::Result;
use util::input_lines;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

type Hand = [u8; 5];
trait HandTrait {
    fn parse(h: &str) -> Hand;
    fn rank(&self) -> HandRank;
    fn greater(&self, other: &Self) -> bool;
}

fn char_to_u8(c: char) -> u8 {
    match c {
        '2'..='9' => c as u8 - '0' as u8,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Out of range"),
    }
}

impl HandTrait for Hand {
    fn parse(h: &str) -> Hand {
        assert_eq!(5, h.len());
        let chars = h.chars().collect::<Vec<_>>();
        [
            char_to_u8(chars[0]),
            char_to_u8(chars[1]),
            char_to_u8(chars[2]),
            char_to_u8(chars[3]),
            char_to_u8(chars[4]),
        ]
    }

    fn rank(&self) -> HandRank {
        let (mut most_same, mut counts) =
            self.iter()
                .copied()
                .fold((0, HashMap::new()), |(mut most_same, mut map), val| {
                    most_same = std::cmp::max(
                        most_same,
                        *map.entry(val).and_modify(|f| *f += 1).or_insert(1),
                    );
                    (most_same, map)
                });
        match most_same {
            1 => HandRank::HighCard,
            2 => match counts.len() {
                4 => HandRank::Pair,
                3 => HandRank::TwoPair,
                _ => panic!("This should never happen"),
            },
            3 => match counts.len() {
                3 => HandRank::ThreeOfAKind,
                2 => HandRank::FullHouse,
                _ => panic!("This should never happen"),
            },
            4 => HandRank::FourOfAKind,
            5 => HandRank::FiveOfAKind,
            _ => panic!("This should never happen"),
        }
    }

    fn greater(&self, other: &Self) -> bool {
        self.rank() > other.rank() || (self.rank() == other.rank() && self > other)
    }
}

fn part1(lines: &Vec<String>) -> Result<u32> {
    let mut hands_and_bids = lines
        .iter()
        .map(|line| {
            let mut iter = line.split_whitespace();
            (
                Hand::parse(iter.next().unwrap()),
                iter.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    hands_and_bids.sort_by(|a, b| {
        if a.0.greater(&b.0) {
            Ordering::Greater
        } else if a.0 == b.0 {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    });
    Ok(hands_and_bids
        .iter()
        .enumerate()
        .map(|(rank, hand_and_bid)| (rank as u32 + 1) * hand_and_bid.1)
        .sum())
}

fn part2(lines: &Vec<String>) -> Result<u32> {
    Ok(0)
}

fn main() -> Result<()> {
    let lines = input_lines();
    println!("Part 1: {}", part1(&lines)?);
    println!("Part 2: {}", part2(&lines)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    fn lines() -> Vec<String> {
        indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "}
        .lines()
        .map(|x| x.to_string())
        .collect()
    }

    #[test]
    fn test_char_to_u8() {
        assert_eq!(char_to_u8('5'), 5);
        assert_eq!(char_to_u8('T'), 10);
        assert_eq!(char_to_u8('J'), 11);
        assert_eq!(char_to_u8('Q'), 12);
        assert_eq!(char_to_u8('K'), 13);
        assert_eq!(char_to_u8('A'), 14);
    }

    #[test]
    fn test_hand() {
        assert_eq!(Hand::parse("32T3K"), [3, 2, 10, 3, 13]);

        assert_eq!(Hand::parse("32T3K").rank(), HandRank::Pair);
        assert_eq!(Hand::parse("T55J5").rank(), HandRank::ThreeOfAKind);
        assert_eq!(Hand::parse("KK677").rank(), HandRank::TwoPair);
        assert_eq!(Hand::parse("KTJJT").rank(), HandRank::TwoPair);
        assert_eq!(Hand::parse("QQQJA").rank(), HandRank::ThreeOfAKind);

        assert!(Hand::parse("QQQJA").greater(&Hand::parse("KK677")));
        assert!(Hand::parse("KK677").greater(&Hand::parse("KTJJT")));
        assert!(Hand::parse("QQQJA").greater(&Hand::parse("T55J5")));
    }

    #[test]
    fn test_hand_rank() {
        assert!(HandRank::FourOfAKind > HandRank::FullHouse);
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(&lines())?, 6440);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&lines())?, 0);
        Ok(())
    }
}
