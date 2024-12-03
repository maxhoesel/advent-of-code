use color_eyre::eyre::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, one_of},
    combinator::{eof, map_res, peek},
    error::ErrorKind,
    multi::{many0, many1, many_m_n, many_till},
    sequence::{delimited, preceded, separated_pair, terminated},
    Finish, IResult,
};

const DONT: &str = "don't()";
const DO: &str = "do()";

fn active_corrupt(input: &str) -> IResult<&str, ()> {
    map_res(
        many_till(
            anychar,
            peek(alt((
                tag(DONT),
                map_res(MulInstruction::parse, |_| Ok::<_, ErrorKind>("")),
            ))),
        ),
        |_| Ok::<_, ErrorKind>(()),
    )(input)
}

#[derive(Debug, Clone)]
struct Program {
    active_segments: Vec<ActiveSegment>,
}
impl Program {
    fn parse(input: &str) -> IResult<&str, Program> {
        map_res(
            many1(terminated(
                ActiveSegment::parse,
                many_till(anychar, alt((tag(DO), eof))),
            )),
            |segments| {
                Ok::<_, ErrorKind>(Program {
                    active_segments: segments,
                })
            },
        )(input)
    }
    fn run(&self) -> u64 {
        self.active_segments
            .iter()
            .fold(0, |acc, seg| acc + seg.run())
    }
}

#[derive(Debug, Clone)]
struct ActiveSegment {
    mul_instructions: Vec<MulInstruction>,
}
impl ActiveSegment {
    fn parse(input: &str) -> IResult<&str, ActiveSegment> {
        map_res(
            terminated(
                many0(preceded(active_corrupt, MulInstruction::parse)),
                preceded(active_corrupt, alt((tag(DONT), eof))),
            ),
            |instrs| {
                Ok::<_, ErrorKind>(ActiveSegment {
                    mul_instructions: instrs,
                })
            },
        )(input)
    }
    fn run(&self) -> u64 {
        self.mul_instructions
            .iter()
            .fold(0, |acc, instr| acc + instr.run())
    }
}

#[derive(Debug, Clone)]
struct MulInstruction {
    a: MulNumber,
    b: MulNumber,
}
impl MulInstruction {
    fn parse(input: &str) -> IResult<&str, MulInstruction> {
        map_res(
            delimited(
                tag("mul("),
                separated_pair(MulNumber::parse, tag(","), MulNumber::parse),
                tag(")"),
            ),
            |instr| {
                Ok::<_, ErrorKind>(MulInstruction {
                    a: instr.0,
                    b: instr.1,
                })
            },
        )(input)
    }
    fn run(&self) -> u64 {
        self.a.value() as u64 * self.b.value() as u64
    }
}

#[derive(Debug, Clone)]
struct MulNumber(u16);
impl MulNumber {
    fn parse(input: &str) -> IResult<&str, MulNumber> {
        map_res(many_m_n(1, 3, one_of("0123456789")), |s| {
            Ok::<_, ErrorKind>(MulNumber(
                s.into_iter().collect::<String>().parse::<u16>().unwrap(),
            ))
        })(input)
    }
    fn value(&self) -> u16 {
        self.0
    }
}

const INPUT: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (_, program) = Program::parse(INPUT).finish()?;
    dbg!(&program);
    println!("{}", program.run());
    Ok(())
}
