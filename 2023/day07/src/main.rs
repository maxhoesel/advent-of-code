use std::collections::HashSet;

use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct PlayedHand {
    hand: Hand,
    bet: u64,
}
impl PlayedHand {
    fn parse(input: &str) -> Result<PlayedHand> {
        let (hand, bet) = input.split_once(' ').ok_or(anyhow!("Invalid input"))?;
        Ok(PlayedHand {
            bet: bet.parse::<u64>()?,
            hand: Hand::parse(&hand.chars().collect_vec())?,
        })
    }
}
impl PartialOrd for PlayedHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PlayedHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct Hand {
    cards: Vec<Card>,
    kind: HandKind,
}
impl Hand {
    fn parse(input: &[char]) -> Result<Hand> {
        let cards = input
            .iter()
            .map(|c| Card::try_from(*c))
            .collect::<Result<Vec<Card>>>()?;
        Ok(Hand {
            cards: cards.clone(),
            kind: HandKind::from_cards(&cards)?,
        })
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind.cmp(&other.kind) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {
                let Some(first_diff) = self
                    .cards
                    .iter()
                    .zip(&other.cards)
                    .find(|(us, them)| us != them)
                else {
                    return std::cmp::Ordering::Equal;
                };
                first_diff.0.cmp(first_diff.1)
            }
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
enum HandKind {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}
impl HandKind {
    fn from_cards(cards: &[Card]) -> Result<HandKind> {
        let joker_num = cards.iter().filter(|c| **c == Card::Joker).count();
        let set: HashSet<&Card> = HashSet::from_iter(cards);
        let counts = cards.iter().counts();
        let unique_count = counts.values().sorted().collect_vec();
        match set.len() {
            1 => Ok(HandKind::FiveOfAKind),
            2 => {
                if joker_num > 0 {
                    Ok(HandKind::FiveOfAKind)
                } else {
                    match unique_count[..] {
                        [2, 3] => Ok(HandKind::FullHouse),
                        [1, 4] => Ok(HandKind::FourOfAKind),
                        _ => unreachable!(),
                    }
                }
            }
            3 => match unique_count[..] {
                [1, 1, 3] => match joker_num {
                    0 => Ok(HandKind::ThreeOfAKind),
                    1 | 3 => Ok(HandKind::FourOfAKind),
                    _ => unreachable!(),
                },
                [1, 2, 2] => match joker_num {
                    0 => Ok(HandKind::TwoPair),
                    1 => Ok(HandKind::FullHouse),
                    2 => Ok(HandKind::FourOfAKind),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            4 => match joker_num {
                // always [1, 1, 1, 2]
                0 => Ok(HandKind::OnePair),
                1 | 2 => Ok(HandKind::ThreeOfAKind),
                _ => unreachable!(),
            },
            5 => match joker_num {
                0 => Ok(HandKind::HighCard),
                1 => Ok(HandKind::OnePair),
                _ => unreachable!(),
            },
            _ => Err(anyhow!("Too many cards!")),
        }
    }
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11, // part1
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    One = 1,
    Joker = 0, // part2
}
impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(s: char) -> Result<Card, Self::Error> {
        match s.to_ascii_lowercase() {
            'a' => Ok(Card::Ace),
            'k' => Ok(Card::King),
            'q' => Ok(Card::Queen),
            'j' => Ok(Card::Joker), // uncomment for part1
            //'j' => Ok(Card::Jack), // comment for part1
            't' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            '1' => Ok(Card::One),
            e => Err(anyhow!("Invalid card name {e}")),
        }
    }
}
impl TryFrom<&char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: &char) -> std::prelude::v1::Result<Self, Self::Error> {
        Card::try_from(*value)
    }
}

const TEST: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let test_hands = TEST
        .lines()
        .map(PlayedHand::parse)
        .collect::<Result<Vec<_>>>()?;

    let test_winnings: u64 = test_hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, hand)| hand.bet * (i as u64 + 1))
        .sum();
    println!("Winnings from tests: {test_winnings}");

    let input_hands = INPUT
        .lines()
        .map(PlayedHand::parse)
        .collect::<Result<Vec<_>>>()?;

    let input_winnings: u64 = input_hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, hand)| hand.bet * (i as u64 + 1))
        .sum();
    println!("Winnings from input: {input_winnings}");

    Ok(())
}
