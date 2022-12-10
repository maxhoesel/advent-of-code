use nom::{
    branch::alt, bytes::complete::tag, character::complete::newline, combinator::map, multi::many1,
    sequence::delimited, Finish, IResult,
};

use crate::interpreter::Command;

pub fn command_list(input: &str) -> Result<Vec<Command>, nom::error::Error<&str>> {
    many1(command)(input).finish().map(|(_, r)| r)
}

fn command(input: &str) -> IResult<&str, Command> {
    alt((noop, addx))(input)
}

fn noop(input: &str) -> IResult<&str, Command> {
    let p = tag("noop\n");
    map(p, |_| Command::Noop)(input)
}

fn addx(input: &str) -> IResult<&str, Command> {
    let p = delimited(tag("addx "), nom::character::complete::i32, newline);
    map(p, Command::Addx)(input)
}
