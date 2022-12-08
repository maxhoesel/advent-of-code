use std::fmt::Display;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::character::complete::char as nomchar;
use nom::character::complete::line_ending;
use nom::character::complete::u64 as nom64;
use nom::combinator::map;
use nom::error::Error;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::Err;
use nom::{sequence::separated_pair, IResult};

#[derive(Clone, Debug)]
pub enum Command {
    ChangeDir(Move),
    List(Vec<Listing>),
}
impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::ChangeDir(d) => write!(
                f,
                "cd {}",
                match d {
                    Move::To(d) => d,
                    Move::Out => "..",
                }
            ),
            Command::List(l) => write!(f, "ls: {:?}", l),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Move {
    To(String),
    Out,
}

#[derive(Clone, Debug)]
pub enum Listing {
    File(File),
    Dir(String),
}

#[derive(Clone, Debug)]
pub struct File {
    pub name: String,
    pub size: u64,
}

pub fn parse_terminal(i: &str) -> Result<Vec<Command>, Err<Error<&str>>> {
    many1(command)(i).map(|(_, a)| a)
}

fn command(i: &str) -> IResult<&str, Command> {
    alt((cd, ls))(i)
}

fn cd(i: &str) -> IResult<&str, Command> {
    let p = delimited(tag("$ cd "), take_until1("\n"), line_ending);
    map(p, |path: &str| {
        if path == ".." {
            Command::ChangeDir(Move::Out)
        } else {
            Command::ChangeDir(Move::To(path.to_string()))
        }
    })(i)
}

fn ls(i: &str) -> IResult<&str, Command> {
    let p = preceded(tag("$ ls\n"), many0(alt((ls_dir, ls_file))));
    map(p, Command::List)(i)
}

fn ls_file(i: &str) -> IResult<&str, Listing> {
    let p = separated_pair(
        nom64::<&str, _>,
        nomchar(' '),
        terminated(take_until1("\n"), line_ending),
    );
    map(p, |(size, name)| {
        Listing::File(File {
            name: name.to_string(),
            size,
        })
    })(i)
}

fn ls_dir(i: &str) -> IResult<&str, Listing> {
    let p = delimited(tag("dir "), take_until1("\n"), line_ending);
    map(p, |dir: &str| Listing::Dir(dir.to_string()))(i)
}
