use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;
use nom::bytes::complete::*;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::combinator::*;
use nom::error::ErrorKind;
use nom::multi::separated_list1;
use nom::sequence::*;
use nom::Finish;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use nom::IResult;

const TEST: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

struct CardSet {
    cards: HashMap<u64, Card>,
}
impl CardSet {
    fn parse(input: &str) -> Result<CardSet> {
        let cards: HashMap<u64, Card> = input
            .lines()
            .map(|line| {
                Card::parse(line)
                    .finish()
                    .map(|r| (r.1.id, r.1))
                    .map_err(|e| anyhow!("Could not parse input: {e}"))
            })
            .collect::<Result<HashMap<_, _>, anyhow::Error>>()?;
        Ok(CardSet { cards })
    }
    fn total_value(&self) -> u64 {
        self.cards.iter().map(|card| card.1.value()).sum()
    }
    fn card_for_cards(&self) -> u64 {
        let mut total_card_count = 0;
        let mut heap = BinaryHeap::new();
        for card in self.cards.values() {
            heap.push(Reverse(card));
        }

        while let Some(Reverse(card)) = heap.pop() {
            total_card_count += 1;
            let new_cards = card
                .win_more_cards()
                .iter()
                .filter_map(|id| self.cards.get(id))
                .collect_vec();
            for card in new_cards {
                heap.push(Reverse(card));
            }
        }

        total_card_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    id: u64,
    winning_numbers: HashSet<u64>,
    own_numbers: HashSet<u64>,
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
impl Card {
    fn parse(input: &str) -> IResult<&str, Card> {
        let parser = tuple((
            delimited(
                terminated(tag("Card"), space1),
                map_res(digit1, str::parse),
                terminated(tag(":"), space1),
            ),
            terminated(
                separated_list1(space1, map_res(digit1, str::parse)),
                delimited(space1, tag("|"), space1),
            ),
            separated_list1(space1, map_res(digit1, str::parse)),
        ));
        map_res(parser, |(id_str, winning_nums, own_nums)| {
            Ok::<_, ErrorKind>(Card {
                id: id_str,
                winning_numbers: HashSet::from_iter(winning_nums),
                own_numbers: HashSet::from_iter(own_nums),
            })
        })(input)
    }

    fn value(&self) -> u64 {
        let wins = self.own_numbers.intersection(&self.winning_numbers).count();
        if wins == 0 {
            0
        } else if wins == 1 {
            1
        } else {
            2_u64.pow(u32::try_from(wins.saturating_sub(1)).expect("Value too large for u64!"))
        }
    }
    fn win_more_cards(&self) -> Vec<u64> {
        let wins = self.own_numbers.intersection(&self.winning_numbers).count();
        if wins == 0 {
            return vec![];
        }

        (1..=wins)
            .map(|i| self.id + u64::try_from(i).expect("Too many wins for u64"))
            .collect_vec()
    }
}

fn main() -> Result<()> {
    let test_set = CardSet::parse(TEST)?;
    let input_set = CardSet::parse(INPUT)?;

    println!("Total value of test cards: {}", test_set.total_value());

    println!("Total value of input cards: {}", input_set.total_value());

    println!(
        "Total number of won test cards: {}",
        test_set.card_for_cards()
    );

    println!(
        "Total number of won input cards: {}",
        input_set.card_for_cards()
    );

    Ok(())
}
