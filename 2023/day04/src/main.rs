use anyhow::anyhow;
use anyhow::Result;
use nom::bytes::complete::*;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::combinator::*;
use nom::error::ErrorKind;
use nom::multi::separated_list1;
use nom::sequence::*;
use nom::Finish;
use std::collections::HashSet;

use nom::IResult;

const TEST: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

struct CardSet {
    cards: Vec<Card>,
}
impl CardSet {
    fn parse(input: &str) -> Result<CardSet> {
        let cards: Vec<Card> = input
            .lines()
            .map(|line| {
                Card::parse(line)
                    .finish()
                    .map(|r| r.1)
                    .map_err(|e| anyhow!("Could not parse input: {e}"))
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;
        Ok(CardSet { cards })
    }
    fn total_value(&self) -> u64 {
        self.cards.iter().map(|card| card.value()).sum()
    }
}

struct Card {
    id: u64,
    winning_numbers: HashSet<u64>,
    own_numbers: HashSet<u64>,
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
            2_u64.pow(u32::try_from(wins.saturating_sub(1)).expect("Value too large for u32!"))
        }
    }
}

fn main() -> Result<()> {
    let test_set = CardSet::parse(TEST)?;
    let input_set = CardSet::parse(INPUT)?;

    println!("Total value of test cards: {}", test_set.total_value());
    println!("Total value of input cards: {}", input_set.total_value());

    Ok(())
}
