use std::{
    collections::{HashMap, HashSet},
    ops::{Range, RangeInclusive},
};

use itertools::Itertools;
use petgraph::{
    algo::dijkstra,
    data::Build,
    dot::{Config, Dot},
    graphmap::GraphMap,
    visit::{Bfs, IntoEdgesDirected},
    Directed,
    Direction::{Incoming, Outgoing},
};

type BrickId = usize;
type BrickGraph = GraphMap<BrickId, usize, Directed>;

const GROUND_ID: BrickId = 0;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos2d {
    x: usize,
    y: usize,
}
impl From<Pos3d> for Pos2d {
    fn from(value: Pos3d) -> Self {
        Pos2d {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos3d {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Brick {
    id: BrickId,
    x: Range<usize>,
    y: Range<usize>,
    z: Range<usize>,
}
impl Brick {
    fn fields(&self) -> impl Iterator<Item = Pos3d> {
        self.x
            .clone()
            .cartesian_product(self.y.clone())
            .cartesian_product(self.z.clone())
            .map(|((x, y), z)| Pos3d { x, y, z })
    }
    fn bottom(&self) -> impl Iterator<Item = Pos3d> + '_ {
        self.x
            .clone()
            .cartesian_product(self.y.clone())
            .map(|(x, y)| Pos3d {
                x,
                y,
                z: self.z.start,
            })
    }
    fn top(&self) -> impl Iterator<Item = Pos3d> + '_ {
        self.x
            .clone()
            .cartesian_product(self.y.clone())
            .map(|(x, y)| Pos3d {
                x,
                y,
                z: self.z.end - 1,
            })
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    let mut bricks = vec![];
    for (i, line) in input.lines().enumerate() {
        let (starts, ends) = line.split_once('~').unwrap();
        let starts = starts
            .split(',')
            .map(|pos| pos.parse::<usize>().unwrap())
            .collect_vec();
        let ends = ends
            .split(',')
            .map(|pos| pos.parse::<usize>().unwrap())
            .collect_vec();
        let x = starts[0]..(ends[0] + 1);
        let y = starts[1]..(ends[1] + 1);
        let z = starts[2]..(ends[2] + 1);
        bricks.push(Brick {
            x,
            y,
            z,
            id: i + GROUND_ID + 1,
        })
    }
    bricks
}

fn drop_bricks(bricks: &[Brick]) -> Vec<Brick> {
    let mut dropped_bricks = vec![];
    let mut highest_z: HashMap<Pos2d, usize> = HashMap::new();
    for brick in bricks.iter().sorted_by_key(|brick| brick.z.start) {
        let drops_to_z = brick
            .bottom()
            .map(|field| {
                highest_z
                    .get(&Pos2d {
                        x: field.x,
                        y: field.y,
                    })
                    .unwrap_or(&0)
            })
            .max()
            .unwrap()
            + 1;
        //dbg!(&brick);
        //dbg!(drops_to_z);
        //dbg!(&highest_z);
        let dropped_brick = Brick {
            x: brick.x.clone(),
            y: brick.y.clone(),
            z: drops_to_z..(brick.z.end - (brick.z.start - drops_to_z)),
            id: brick.id,
        };
        for field in dropped_brick.top() {
            let current_z = highest_z.get(&field.into()).unwrap_or(&0);
            if field.z > *current_z {
                highest_z.insert(field.into(), field.z);
            }
        }
        dropped_bricks.push(dropped_brick);
    }
    dropped_bricks
}

fn brick_graph(input: &str) -> BrickGraph {
    let bricks = drop_bricks(&parse_input(input));
    let mut field_map = HashMap::new();
    for brick in &bricks {
        for field in brick.fields() {
            field_map.insert(field, brick);
        }
    }

    let mut stack_map: HashMap<&Brick, HashSet<&&Brick>> = HashMap::new();
    for brick in &bricks {
        for top_field in brick.top() {
            if let Some(supported) = field_map.get(&Pos3d {
                x: top_field.x,
                y: top_field.y,
                z: top_field.z + 1,
            }) {
                stack_map.entry(brick).or_default().insert(supported);
            } else if !stack_map.contains_key(brick) {
                stack_map.insert(brick, HashSet::new());
            }
        }
    }

    let mut graph = GraphMap::new();
    // add ground node and basic brick supports
    graph.add_node(GROUND_ID);
    for (bottom_bid, supports) in stack_map {
        graph.add_node(bottom_bid.id);
        if bottom_bid.z.start == 1 {
            // brick is on ground, connect it
            graph.add_edge(GROUND_ID, bottom_bid.id, 1);
        }
        for above_bid in supports {
            graph.add_edge(bottom_bid.id, above_bid.id, 1);
        }
    }
    graph
}

fn get_removeable_bricks(graph: &BrickGraph) -> Vec<BrickId> {
    let mut unsafe_bricks = HashSet::new();
    for node in graph.nodes() {
        let inc_edges = graph.edges_directed(node, Incoming).collect_vec();
        debug_assert!(!inc_edges.is_empty() || node == GROUND_ID);
        if inc_edges.len() == 1 {
            // exactly one brick is supporting this one, we cannot remove that supporting brick
            unsafe_bricks.insert(inc_edges[0].0);
        }
    }
    graph
        .nodes()
        .collect::<HashSet<_>>()
        .difference(&unsafe_bricks)
        .copied()
        .collect_vec()
}

fn disintegration_sum(graph: &BrickGraph) -> HashMap<BrickId, usize> {
    let mut disintegration_counts = HashMap::new();
    for node in graph.nodes() {
        if node == GROUND_ID {
            continue;
        }
        let mut reachable_bricks = HashSet::new();

        // first, find all nodes we can reach from the block to remove
        let mut bfs = Bfs::new(&graph, node);
        while let Some(supported) = bfs.next(graph) {
            reachable_bricks.insert(supported);
        }

        // then, eliminate any bricks that are supported by something else
        let multiply_supported_edge_bricks = reachable_bricks
            .iter()
            .filter(|id| {
                id != &&node
                    && !graph
                        .edges_directed(**id, Incoming)
                        .filter(|e| !reachable_bricks.contains(&e.0))
                        .collect_vec()
                        .is_empty()
            })
            .collect_vec();
        let mut multiply_supported_bricks = HashSet::new();
        for brick in multiply_supported_edge_bricks {
            let mut bfs = Bfs::new(&graph, *brick);
            while let Some(supported) = bfs.next(graph) {
                multiply_supported_bricks.insert(supported);
            }
        }

        disintegration_counts.insert(
            node,
            reachable_bricks
                .difference(&multiply_supported_bricks)
                .count()
                .saturating_sub(1), // -1 because we don't count the removed brick itself as falling
        );
    }
    disintegration_counts
}

const TEST: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

fn main() {
    let g = brick_graph(TEST);
    //println!("{:?}", get_removeable_bricks(&g));
    println!("{:?}", get_removeable_bricks(&g).len());
    //println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
    println!("{:?}", disintegration_sum(&g).values().sum::<usize>());

    let g = brick_graph(INPUT);
    println!("{:?}", get_removeable_bricks(&g).len());
    println!("{:?}", disintegration_sum(&g).values().sum::<usize>());
}
