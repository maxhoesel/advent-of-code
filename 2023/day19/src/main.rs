use std::{collections::HashMap, ops::Range, str::FromStr};

use anyhow::{anyhow, Context};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, digit1, multispace1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    Finish,
};

type Rating = u16;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Category {
    X,
    M,
    A,
    S,
}
impl TryFrom<char> for Category {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'x' => Ok(Category::X),
            'm' => Ok(Category::M),
            'a' => Ok(Category::A),
            's' => Ok(Category::S),
            e => Err(anyhow!("{e} is not a valid category")),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Part {
    x: Rating,
    m: Rating,
    a: Rating,
    s: Rating,
}
impl Part {
    fn rating(&self) -> u32 {
        self.x as u32 + self.m as u32 + self.a as u32 + self.s as u32
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct PartsRange {
    x: Range<Rating>,
    m: Range<Rating>,
    a: Range<Rating>,
    s: Range<Rating>,
}
impl PartsRange {
    fn num_entries(&self) -> u64 {
        self.x.len() as u64 * self.m.len() as u64 * self.a.len() as u64 * self.s.len() as u64
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Action {
    Forward(String),
    Finish(Outcome),
}
impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Action::Finish(Outcome::Rejected)),
            "A" => Ok(Action::Finish(Outcome::Accepted)),
            name => Ok(Action::Forward(name.to_string())),
        }
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Outcome {
    Rejected,
    Accepted,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Workflow<'a> {
    name: &'a str,
    checks: Vec<(Condition, Action)>,
    fallback: Action,
}
impl Workflow<'_> {
    fn apply(&self, part: &Part) -> Action {
        for (check, action) in &self.checks {
            if check.matches(part) {
                return action.clone();
            }
        }
        self.fallback.clone()
    }
    fn possible_outcomes(&self, parts_range: PartsRange) -> Vec<(PartsRange, Action)> {
        let mut split_actions = vec![];
        let mut current_range = parts_range;
        for (check, action) in &self.checks {
            let (split, new_range) = check.matching_range_subsection(current_range.clone());
            if let Some(split_range) = split {
                split_actions.push((split_range, action.clone()));
            }
            if current_range.num_entries() == 0 {
                return split_actions;
            }
            current_range = new_range;
        }
        if current_range.num_entries() > 0 {
            split_actions.push((current_range, self.fallback.clone()));
        }
        split_actions
    }
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
struct Condition {
    left: Category,
    op: std::cmp::Ordering,
    right: Rating,
}
impl Condition {
    fn matches(&self, part: &Part) -> bool {
        let left_val = match self.left {
            Category::X => part.x,
            Category::M => part.m,
            Category::A => part.a,
            Category::S => part.s,
        };
        left_val.cmp(&self.right) == self.op
    }
    /// Returns the subsection of a range that matches this condition, as well as the reduced original range.
    /// if nothing in the range matches, None is returned instead
    fn matching_range_subsection(
        &self,
        mut parts_range: PartsRange,
    ) -> (Option<PartsRange>, PartsRange) {
        (
            match self.left {
                Category::X if parts_range.x.contains(&self.right) => match self.op {
                    std::cmp::Ordering::Less => {
                        let mut split = parts_range.clone();
                        split.x = parts_range.x.start..self.right;
                        parts_range.x.start = self.right;
                        Some(split)
                    }
                    std::cmp::Ordering::Equal => todo!(),
                    std::cmp::Ordering::Greater => {
                        let mut split = parts_range.clone();
                        split.x = self.right + 1..parts_range.x.end;
                        parts_range.x.end = self.right + 1;
                        Some(split)
                    }
                },
                Category::M if parts_range.m.contains(&self.right) => match self.op {
                    std::cmp::Ordering::Less => {
                        let mut split = parts_range.clone();
                        split.m = parts_range.m.start..self.right;
                        parts_range.m.start = self.right;
                        Some(split)
                    }
                    std::cmp::Ordering::Equal => todo!(),
                    std::cmp::Ordering::Greater => {
                        let mut split = parts_range.clone();
                        split.m = self.right + 1..parts_range.m.end;
                        parts_range.m.end = self.right + 1;
                        Some(split)
                    }
                },
                Category::A if parts_range.a.contains(&self.right) => match self.op {
                    std::cmp::Ordering::Less => {
                        let mut split = parts_range.clone();
                        split.a = parts_range.a.start..self.right;
                        parts_range.a.start = self.right;
                        Some(split)
                    }
                    std::cmp::Ordering::Equal => todo!(),
                    std::cmp::Ordering::Greater => {
                        let mut split = parts_range.clone();
                        split.a = self.right + 1..parts_range.a.end;
                        parts_range.a.end = self.right + 1;
                        Some(split)
                    }
                },
                Category::S if parts_range.s.contains(&self.right) => match self.op {
                    std::cmp::Ordering::Less => {
                        let mut split = parts_range.clone();
                        split.s = parts_range.s.start..self.right;
                        parts_range.s.start = self.right;
                        Some(split)
                    }
                    std::cmp::Ordering::Equal => todo!(),
                    std::cmp::Ordering::Greater => {
                        let mut split = parts_range.clone();
                        split.s = self.right + 1..parts_range.s.end;
                        parts_range.s.end = self.right + 1;
                        Some(split)
                    }
                },
                _ => None,
            },
            parts_range,
        )
    }
}

const START_WORKFLOW: &str = "in";
const MAX_VAL: Rating = 4000;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PartsPile<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
    parts: Vec<Part>,
}
impl PartsPile<'_> {
    fn find_accepted_ratings_combination_count(&self) -> u64 {
        let start = PartsRange {
            x: 1..MAX_VAL + 1,
            m: 1..MAX_VAL + 1,
            a: 1..MAX_VAL + 1,
            s: 1..MAX_VAL + 1,
        };
        let mut accepted_ranges = vec![];
        let mut possible_outcomes = self.workflows[START_WORKFLOW].possible_outcomes(start);
        while let Some((todo_range, todo_action)) = possible_outcomes.pop() {
            if todo_range.num_entries() == 0 {
                continue;
            }
            match todo_action {
                Action::Forward(workflow) => {
                    possible_outcomes.extend(
                        self.workflows[workflow.as_str()]
                            .possible_outcomes(todo_range)
                            .into_iter(),
                    );
                }
                Action::Finish(Outcome::Accepted) => {
                    accepted_ranges.push(todo_range);
                }
                Action::Finish(Outcome::Rejected) => (),
            }
        }
        accepted_ranges
            .iter()
            .map(|range| range.num_entries())
            .sum()
    }

    fn evaluate_part(&self, part: &Part) -> Outcome {
        let mut action = self.workflows.get(START_WORKFLOW).unwrap().apply(part);
        while let Action::Forward(to) = action {
            action = self.workflows.get(to.as_str()).unwrap().apply(part);
        }
        if let Action::Finish(outcome) = action {
            return outcome;
        }
        unreachable!()
    }

    /// Returns (accepted, rejected)
    fn all_accepted_parts_rating_sum(&self) -> u64 {
        self.parts
            .iter()
            .filter(|part| self.evaluate_part(part) == Outcome::Accepted)
            .map(|p| p.rating() as u64)
            .sum()
    }

    fn parse(input: &str) -> PartsPile<'_> {
        let mut workflows_parser = separated_list1(
            newline,
            pair(
                alpha1,
                delimited(
                    tag("{"),
                    pair(
                        separated_list1(
                            tag(","),
                            separated_pair(
                                tuple((
                                    map_res(anychar, Category::try_from),
                                    map_res(anychar, |cmp| match cmp {
                                        '<' => Ok(std::cmp::Ordering::Less),
                                        '=' => Ok(std::cmp::Ordering::Equal),
                                        '>' => Ok(std::cmp::Ordering::Greater),
                                        e => Err(anyhow!("Not a valid comparsion operator: {e}")),
                                    }),
                                    map_res(digit1, |right: &str| right.parse::<Rating>()),
                                )),
                                tag(":"),
                                map_res(alpha1, |act: &str| act.parse::<Action>()),
                            ),
                        ),
                        preceded(
                            tag(","),
                            map_res(alpha1, |fallback: &str| fallback.parse::<Action>()),
                        ),
                    ),
                    tag("}"),
                ),
            ),
        );
        let (rest, workflows) = workflows_parser(input)
            .finish()
            .map_err(|e: nom::error::Error<&str>| anyhow!("Error parsing: {e}"))
            .context("Parsing workflows")
            .expect("parsing error");
        let workflows = workflows
            .iter()
            .map(|(name, (checks_raw, fallback))| Workflow {
                name,
                checks: checks_raw
                    .iter()
                    .map(|(condition, action)| {
                        (
                            Condition {
                                left: condition.0,
                                op: condition.1,
                                right: condition.2,
                            },
                            action.clone(),
                        )
                    })
                    .collect_vec(),
                fallback: fallback.clone(),
            })
            .map(|workflow| (workflow.name, workflow))
            .collect();

        let mut parts_parser = preceded(
            multispace1,
            separated_list1(
                newline,
                delimited(
                    tag("{"),
                    tuple((
                        separated_pair(
                            anychar,
                            tag("="),
                            map_res(digit1, |dig: &str| dig.parse::<Rating>()),
                        ),
                        tag(","),
                        separated_pair(
                            anychar,
                            tag("="),
                            map_res(digit1, |dig: &str| dig.parse::<Rating>()),
                        ),
                        tag(","),
                        separated_pair(
                            anychar,
                            tag("="),
                            map_res(digit1, |dig: &str| dig.parse::<Rating>()),
                        ),
                        tag(","),
                        separated_pair(
                            anychar,
                            tag("="),
                            map_res(digit1, |dig: &str| dig.parse::<Rating>()),
                        ),
                    )),
                    tag("}"),
                ),
            ),
        );
        let parts = parts_parser(rest)
            .finish()
            .map_err(|e: nom::error::Error<&str>| anyhow!("Error parsing: {e}"))
            .context("Parsing parts")
            .expect("parsing error")
            .1
            .iter()
            .map(|(x, _, m, _, a, _, s)| Part {
                x: x.1,
                m: m.1,
                a: a.1,
                s: s.1,
            })
            .collect_vec();

        PartsPile { workflows, parts }
    }
}

const TEST: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

fn main() {
    let test_pile = PartsPile::parse(TEST);
    println!(
        "Test pile sum: {}",
        test_pile.all_accepted_parts_rating_sum()
    );

    let input_pile = PartsPile::parse(INPUT);
    println!(
        "Input pile sum: {}",
        input_pile.all_accepted_parts_rating_sum()
    );

    // part2
    println!(
        "Test pile total accepted combinations: {}",
        test_pile.find_accepted_ratings_combination_count()
    );
    println!(
        "Input pile total accepted combinations: {}",
        input_pile.find_accepted_ratings_combination_count()
    );
}
