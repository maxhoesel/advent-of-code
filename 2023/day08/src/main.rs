use std::fmt::Display;

use anyhow::anyhow;
use anyhow::Result;

use itertools::Itertools;
use nom::character::complete::alphanumeric1;
use nom::character::complete::multispace0;
use nom::character::complete::one_of;
use nom::combinator::map_res;
use nom::error::ErrorKind;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace1, space0, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    Finish,
};
use num::Integer;
use petgraph::visit::EdgeRef;
use petgraph::Direction::Outgoing;
use petgraph::{graphmap::GraphMap, Directed};

const TEST1: &str = include_str!("test1.txt");
const TEST2: &str = include_str!("test2.txt");
const TEST3: &str = include_str!("test3.txt");
const INPUT: &str = include_str!("input.txt");
const TARGET_NODE: &str = "ZZZ";
const START_NODE: &str = "AAA";

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Edge {
    L,
    R,
    Both,
}
impl TryFrom<char> for Edge {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'l' => Ok(Edge::L),
            'r' => Ok(Edge::R),
            e => Err(anyhow!("Cannot convert {e} to Left/Right")),
        }
    }
}
impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Edge::L => "Left",
                Edge::R => "Right",
                Edge::Both => "Left+Right",
            }
        )
    }
}

#[derive(Clone, Debug)]
struct Map<'a> {
    directions: Vec<Edge>,
    graph: GraphMap<&'a str, Edge, Directed>,
}

impl Map<'_> {
    fn parse(input: &str) -> Result<Map<'_>> {
        let (directions, nodes) = terminated(
            separated_pair(
                many1(map_res(one_of("LR"), |r| {
                    Ok::<_, ErrorKind>(Edge::try_from(r).unwrap())
                })),
                multispace1,
                separated_list1(
                    multispace1,
                    separated_pair(
                        alphanumeric1,
                        delimited(space1, tag("="), space1),
                        delimited(
                            tag("("),
                            separated_pair(
                                alphanumeric1,
                                delimited(space0, tag(","), space0),
                                alphanumeric1,
                            ),
                            tag(")"),
                        ),
                    ),
                ),
            ),
            multispace0,
        )(input)
        .finish()
        .map_err(|e: nom::error::Error<&str>| anyhow!("Parse Error: {e}"))?
        .1;

        let mut g = GraphMap::with_capacity(nodes.len(), nodes.len() * 2);
        for (node, (left, right)) in nodes {
            if left == right {
                g.add_edge(node, left, Edge::Both);
            } else {
                g.add_edge(node, left, Edge::L);
                g.add_edge(node, right, Edge::R);
            }
        }
        Ok(Map {
            graph: g,
            directions,
        })
    }
    fn follow_path(&self) -> Result<usize> {
        let mut current_node = START_NODE;
        let mut steps = 0;
        loop {
            for direction in &self.directions {
                current_node = self
                    .graph
                    .edges_directed(current_node, Outgoing)
                    .find(|e| e.weight() == direction || e.weight() == &Edge::Both)
                    .map(|e| e.1)
                    .ok_or(anyhow!(
                        "Could not find next node. At node {current_node}, direction: {direction}"
                    ))?;
                steps += 1;
                if current_node == TARGET_NODE {
                    return Ok(steps);
                }
            }
        }
    }
    fn follow_parallel_bruteforce(&self) -> Result<usize> {
        let mut current_nodes = self
            .graph
            .nodes()
            .filter(|n| n.ends_with('A'))
            .collect_vec();
        let mut steps = 0;
        loop {
            for direction in &self.directions {
                current_nodes = current_nodes
                    .iter()
                    .map(|n| {
                        self.graph
                            .edges_directed(n, Outgoing)
                            .find(|e| e.weight() == direction || e.weight() == &Edge::Both)
                            .map(|e| e.1)
                            .ok_or(anyhow!(
                                "Could not find next node. At node {n}, direction: {direction}"
                            ))
                    })
                    .collect::<Result<Vec<_>>>()?;
                steps += 1;
                if current_nodes.iter().all(|n| n.ends_with('Z')) {
                    return Ok(steps);
                }
            }
        }
    }
    fn follow_parallel_smart(&self) -> Result<usize> {
        let start_nodes = self
            .graph
            .nodes()
            .filter(|n| n.ends_with('A'))
            .collect_vec();

        // Calculate the individual cycle lenghts and then take the least common multiple
        // - this is actually not guaranteed to work for every input,
        // because there is nothing saying that once a node reaches a Z node, it will always reach the same node.
        // I think we'd have to verify that (loop again until we reach the same z node) and make sure that this cycle
        // is the same length as the initial path to the first Z. Then we have a guaranteed cycle and just taking the LCM
        // of all nodes works to get us the first time they all match up.
        Ok(start_nodes.iter().map(|n| {
            let mut current_node = *n;
            let mut steps = 0;
            loop {
                for direction in &self.directions {
                    current_node = self
                        .graph
                        .edges_directed(current_node, Outgoing)
                        .find(|e: &(&str, &str, &Edge)| e.weight() == direction || e.weight() == &Edge::Both)
                        .map(|e| e.1)
                        .ok_or(anyhow!(
                            "Could not find next node. At node {current_node}, direction: {direction}"
                        ))?;
                    steps += 1;
                    if current_node.ends_with('Z') {
                        return Ok(steps);
                    }
                }
            }
        }).collect::<Result<Vec<usize>>>()?.iter().fold(1, |acc, e| acc.lcm(e)))
    }
}

fn main() -> Result<()> {
    let test1_map = Map::parse(TEST1)?;
    println!("{}", test1_map.follow_path()?);

    let test2_map = Map::parse(TEST2)?;
    println!("{}", test2_map.follow_path()?);

    let input_map = Map::parse(INPUT)?;
    println!("{}", input_map.follow_path()?);

    let test3_map = Map::parse(TEST3)?;
    println!("{}", test3_map.follow_parallel_bruteforce()?);
    println!("{}", test3_map.follow_parallel_smart()?);

    //let input_map = Map::parse(INPUT)?;
    //println!("{}", input_map.follow_parallel()?);
    println!("{}", input_map.follow_parallel_smart()?);

    Ok(())
}
