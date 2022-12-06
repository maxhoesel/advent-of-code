use std::fmt::Display;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, line_ending},
    combinator::map,
    error::Error,
    multi::{many1, many_till},
    sequence::{delimited, terminated},
    Finish, IResult,
};

pub type Crate = char;

#[derive(Clone)]
pub struct Stack(pub Vec<Crate>);
impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().fold(Ok(()), |result, new| {
            result.and_then(|_| write!(f, "[{}]", new))
        })
    }
}

pub fn parse_stacks(input: &str) -> Result<(&str, Vec<Stack>), Error<&str>> {
    let (rest, mut lines) = crate_lines(input).finish()?;

    lines.reverse();

    let stacks = lines
        .get(0)
        .unwrap()
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            Stack(
                lines
                    .iter()
                    .map(|l| l.get(idx).unwrap())
                    .map_while(|c| c.to_owned())
                    .collect_vec(),
            )
        })
        .collect_vec();

    Ok((rest, stacks))
}

fn crate_lines(input: &str) -> IResult<&str, Vec<Vec<Option<Crate>>>> {
    many1(crate_line)(input)
}

fn crate_line(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
    many_till(
        terminated(crate_or_empty, char(' ')),
        terminated(crate_or_empty, line_ending),
    )(input)
    .map(|(rest, (mut main, last))| {
        main.push(last);
        (rest, main)
    })
}

fn crate_or_empty(input: &str) -> IResult<&str, Option<Crate>> {
    map(alt((crate_box, empty)), |s: char| {
        if s.is_whitespace() {
            None
        } else {
            Some(s)
        }
    })(input)
}

fn crate_box(input: &str) -> IResult<&str, char> {
    delimited(char('['), anychar, char(']'))(input)
}

fn empty(input: &str) -> IResult<&str, char> {
    tag("   ")(input).map(|(rest, _)| (rest, ' '))
}
