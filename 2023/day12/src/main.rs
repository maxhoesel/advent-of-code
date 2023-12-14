use std::{fmt::Display, ops::Deref, str::FromStr};

use anyhow::{anyhow, Result};
use futures::future::{join_all, try_join_all};
use itertools::{Itertools, Unfold};
use nom::{
    bytes::complete::tag,
    character::complete::space1,
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    Finish,
};
use rayon::prelude::*;
use strum::{EnumIter, IntoEnumIterator};
use tokio::try_join;

const OPERATIONAL: char = '.';
const DAMAGED: char = '#';
const UNKNOWN: char = '?';

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum CorruptedSpring {
    Known(Spring),
    Unknown,
}
impl TryFrom<char> for CorruptedSpring {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            UNKNOWN => Ok(CorruptedSpring::Unknown),
            other => Ok(CorruptedSpring::Known(Spring::try_from(other)?)),
        }
    }
}
impl From<CorruptedSpring> for char {
    fn from(value: CorruptedSpring) -> Self {
        match value {
            CorruptedSpring::Known(s) => s.into(),
            CorruptedSpring::Unknown => UNKNOWN,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CorruptedSpringLine {
    springs: Vec<CorruptedSpring>,
    damaged_segments: Vec<u32>,
}
impl FromStr for CorruptedSpringLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut parser = separated_pair(
            many1(map_res(nom::character::complete::anychar, |c| {
                CorruptedSpring::try_from(c)
            })),
            space1,
            separated_list1(tag(","), nom::character::complete::u32),
        );
        let (springs, damanged_lenghts) = parser(s)
            .finish()
            .map_err(|e: nom::error::Error<&str>| anyhow!("Parse error: {e}"))?
            .1;

        Ok(CorruptedSpringLine {
            springs,
            damaged_segments: damanged_lenghts,
        })
    }
}
impl CorruptedSpringLine {
    fn unfold(&mut self, factor: usize) {
        self.springs = self.springs.repeat(factor);
        self.damaged_segments = self.damaged_segments.repeat(factor);
    }

    async fn possible_repairs(&self) -> Vec<SpringLine> {
        let unknown_springs_count = self
            .springs
            .iter()
            .filter(|s| **s == CorruptedSpring::Unknown)
            .count();

        if unknown_springs_count == 0 {
            // hey, we're actually a normal line!
            return vec![];
        }

        let possible_replacements = std::iter::repeat(Spring::iter())
            .take(unknown_springs_count)
            .multi_cartesian_product()
            .collect_vec();

        let all_replacements = possible_replacements
            .par_iter()
            .filter_map(|replacements| {
                let mut count = 0;
                let mut try_line = vec![];
                for spring in &self.springs {
                    try_line.push(match spring {
                        CorruptedSpring::Known(s) => *s,
                        CorruptedSpring::Unknown => {
                            count += 1;
                            replacements[count - 1]
                        }
                    })
                }
                SpringLine::try_from_parts(try_line, self.damaged_segments.clone()).ok()
            })
            .collect::<Vec<_>>();

        all_replacements
    }

    async fn possible_repairs_count(&self) -> usize {
        self.possible_repairs().await.len()
    }
}
impl Display for CorruptedSpringLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.springs
                .iter()
                .map(|c| { char::from(*c) })
                .collect::<String>(),
            self.damaged_segments
                .iter()
                .map(|e| e.to_string())
                .join(",")
        )
    }
}

struct CorruptedSpringField {
    lines: Vec<CorruptedSpringLine>,
}
impl CorruptedSpringField {
    async fn reconstruct_field(&self) -> Vec<(&CorruptedSpringLine, Vec<SpringLine>)> {
        let line_sets = join_all(
            self.lines
                .par_iter()
                .map(|line| line.possible_repairs())
                .collect::<Vec<_>>(),
        )
        .await;
        line_sets
            .into_iter()
            .enumerate()
            .map(|(i, set)| (&self.lines[i], set))
            .collect_vec()
    }
    fn unfold(&mut self, factor: usize) {
        for line in &mut self.lines {
            line.unfold(factor);
        }
    }
    async fn reconstruction_lines_sum(&self) -> usize {
        join_all(
            self.lines
                .par_iter()
                .map(|line| line.possible_repairs_count())
                .collect::<Vec<_>>(),
        )
        .await
        .iter()
        .sum()
    }
}
impl FromStr for CorruptedSpringField {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(CorruptedSpringField {
            lines: s
                .lines()
                .map(|l| l.parse::<CorruptedSpringLine>())
                .collect::<Result<Vec<_>>>()?,
        })
    }
}
impl Display for CorruptedSpringField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in &self.lines {
            l.fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
enum Spring {
    Damaged,
    Operational,
}
impl TryFrom<char> for Spring {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            OPERATIONAL => Ok(Spring::Operational),
            DAMAGED => Ok(Spring::Damaged),
            e => Err(anyhow!("not a valid spring: {e}")),
        }
    }
}
impl From<Spring> for char {
    fn from(value: Spring) -> Self {
        match value {
            Spring::Damaged => DAMAGED,
            Spring::Operational => OPERATIONAL,
        }
    }
}
impl From<&Spring> for char {
    fn from(value: &Spring) -> Self {
        (*value).into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SpringLine {
    springs: Vec<Spring>,
    damaged_segments: Vec<u32>,
}
impl SpringLine {
    fn try_from_parts(springs: Vec<Spring>, damaged_lenghts: Vec<u32>) -> Result<SpringLine> {
        let damaged_spring_segment_lenghts = springs
            .iter()
            .map(|spring| (spring, 1_u32))
            .coalesce(|(prev, m), (current, n)| {
                if prev == current {
                    Ok((prev, m + n))
                } else {
                    Err(((prev, m), (current, n)))
                }
            })
            .filter_map(|seq| {
                if seq.0 == &Spring::Damaged {
                    Some(seq.1)
                } else {
                    None
                }
            })
            .collect_vec();

        if damaged_spring_segment_lenghts == damaged_lenghts {
            Ok(SpringLine {
                springs: springs.clone(),
                damaged_segments: damaged_lenghts,
            })
        } else {
            Err(anyhow!("Damaged segments and springs don't match!"))
        }
    }
}
impl FromStr for SpringLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut parser = separated_pair(
            many1(map_res(nom::character::complete::anychar, |c| {
                Spring::try_from(c)
            })),
            space1,
            separated_list1(tag(","), nom::character::complete::u32),
        );
        let (springs, damaged_lengths) = parser(s)
            .finish()
            .map_err(|e: nom::error::Error<&str>| anyhow!("Parse error: {e}"))?
            .1;
        SpringLine::try_from_parts(springs, damaged_lengths)
    }
}
impl Display for SpringLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.springs
                .iter()
                .map(|c| { char::from(c) })
                .collect::<String>(),
            self.damaged_segments
                .iter()
                .map(|e| e.to_string())
                .join(",")
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SpringField {
    lines: Vec<SpringLine>,
}
impl FromStr for SpringField {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(SpringField {
            lines: s
                .lines()
                .map(|l| l.parse::<SpringLine>())
                .collect::<Result<Vec<_>>>()?,
        })
    }
}
impl Display for SpringField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in &self.lines {
            l.fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

const COMPLETE_TEST: &str = include_str!("complete_test.txt");
const TEST: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

#[tokio::main]
async fn main() -> Result<()> {
    let complete_field = COMPLETE_TEST.parse::<SpringField>()?;
    println!("{}", complete_field);

    let mut test = TEST.parse::<CorruptedSpringField>()?;
    println!("{}", test);
    let reconstructed = test.reconstruct_field().await;
    for line in &reconstructed {
        println!("Options for Line {}", line.0);
        for option in &line.1 {
            println!("{}", option)
        }
    }
    println!(
        "Possible rearrangements for test: {}",
        test.reconstruction_lines_sum().await
    );

    let input = INPUT.parse::<CorruptedSpringField>()?;
    println!(
        "Possible rearrangements for input: {}",
        input.reconstruction_lines_sum().await
    );

    // Part2

    test.unfold(5);
    println!("{}", test);
    println!(
        "Possible rearrangements for unfolded test: {}",
        test.reconstruction_lines_sum().await
    );

    Ok(())
}
