use std::fmt::Display;

use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending},
    combinator::map,
    error::Error,
    multi::many1,
    sequence::{delimited, pair},
    Finish, IResult,
};

#[derive(Debug)]
pub struct Instruction {
    pub from: usize,
    pub to: usize,
    pub amount: usize,
}
impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {} -> {}", self.from + 1, self.amount, self.to + 1)
    }
}

pub fn parse_instructions(input: &str) -> Result<(&str, Vec<Instruction>), Error<&str>> {
    instruction_lines(input).finish()
}

fn instruction_lines(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(instruction_line)(input)
}

fn instruction_line(input: &str) -> IResult<&str, Instruction> {
    map(pair(amount, from_to), |(amount, (from, to))| Instruction {
        // Input indexes by 1, our Stack vec starts at 0
        from: (from - 1) as usize,
        to: (to - 1) as usize,
        amount: amount as usize,
    })(input)
}

fn amount(input: &str) -> IResult<&str, u32> {
    delimited(tag("move "), nom::character::complete::u32, char(' '))(input)
}

fn from_to(input: &str) -> IResult<&str, (u32, u32)> {
    pair(from, to)(input)
}

fn from(input: &str) -> IResult<&str, u32> {
    delimited(tag("from "), nom::character::complete::u32, char(' '))(input)
}

fn to(input: &str) -> IResult<&str, u32> {
    delimited(tag("to "), nom::character::complete::u32, line_ending)(input)
}
