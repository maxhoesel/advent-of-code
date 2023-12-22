use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use indicatif::ProgressBar;
use itertools::Itertools;
use petgraph::{
    data::Build,
    dot::Dot,
    graph::Node,
    graphmap::GraphMap,
    Directed,
    Direction::{Incoming, Outgoing},
};

const MAX_STRAIGHT_STEPS: u32 = 3;

use ringbuffer::{AllocRingBuffer, RingBuffer};
use strum::IntoEnumIterator;
use util::grid::{Direction, Orientation, Position, SparseGrid};

type CrucibleGraph = GraphMap<Position, (u32, Direction), Directed>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct NodeVariant {
    orientation: Orientation,
    straight_steps: u32,
}
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct VariantData {
    cost: u32,
    predecessor: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct ByCostEntry {
    cost: u32,
    pos: Position,
    variant: NodeVariant,
}
impl PartialOrd for ByCostEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ByCostEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn crooked_dijkstra(graph: &CrucibleGraph, start: Position, goal: Position) -> u32 {
    let mut found_variants: HashMap<Position, HashMap<NodeVariant, VariantData>> = HashMap::new();
    let mut start_map = HashMap::new();
    start_map.insert(
        NodeVariant {
            orientation: Orientation::Horizontal,
            straight_steps: 0,
        },
        VariantData {
            cost: 0,
            predecessor: Direction::Left, // meaningless
        },
    );
    start_map.insert(
        NodeVariant {
            orientation: Orientation::Vertical,
            straight_steps: 0,
        },
        VariantData {
            cost: 0,
            predecessor: Direction::Left, // meaningless
        },
    );
    found_variants.insert(start, start_map);
    let mut variants_by_cost: BinaryHeap<Reverse<ByCostEntry>> = BinaryHeap::new();
    variants_by_cost.push(Reverse(ByCostEntry {
        cost: 0,
        pos: start,
        variant: NodeVariant {
            orientation: Orientation::Horizontal,
            straight_steps: 0,
        },
    }));
    variants_by_cost.push(Reverse(ByCostEntry {
        cost: 0,
        pos: start,
        variant: NodeVariant {
            orientation: Orientation::Vertical,
            straight_steps: 0,
        },
    }));

    let mut visited_variants: HashSet<(Position, NodeVariant)> = HashSet::new();

    loop {
        // find the next lowest value between horizontal and vertical
        let (current_pos, current_variant, current_variant_data) = loop {
            let Some(Reverse(next)) = variants_by_cost.pop() else {
                panic!("Graph divided");
            };
            if !visited_variants.contains(&(next.pos, next.variant))
                    // if the cost has diverged, this implies that the found variant has been updated since, ignore it
                    && found_variants
                        .get(&next.pos)
                        .unwrap()
                        .get(&next.variant)
                        .unwrap()
                        .cost
                        == next.cost
            {
                break (
                    next.pos,
                    next.variant,
                    *found_variants
                        .get(&next.pos)
                        .unwrap()
                        .get(&next.variant)
                        .unwrap(),
                );
            }
        };

        if found_variants.contains_key(&goal)
            && current_variant_data.cost
                > found_variants
                    .get(&goal)
                    .unwrap()
                    .iter()
                    .max_by_key(|goal| goal.1.cost)
                    .map(|goal| goal.1.cost)
                    .unwrap()
        {
            // no more improvement paths available, stop
            break;
        }

        for (_, to, (cost, dir)) in graph
            .edges_directed(current_pos, Outgoing)
            // we only care about edges that are relevant to our variants orientation
            .filter(|(_, _, (_, dir))| current_variant.orientation == dir.orientation())
        {
            // construct the possible variants for the connecting node
            let mut possible_variants = vec![
                // We can always flip direction
                NodeVariant {
                    orientation: current_variant.orientation.flip(),
                    straight_steps: 0,
                },
            ];
            if current_variant.straight_steps + 1 < MAX_STRAIGHT_STEPS {
                // the connecting node could also move in the same direction as us if we're below the step limit
                possible_variants.push(NodeVariant {
                    orientation: current_variant.orientation,
                    straight_steps: current_variant.straight_steps + 1,
                });
            }
            for candiate_variant in possible_variants {
                if visited_variants.contains(&(to, candiate_variant)) {
                    // this variant has already been visited, no way it could be any better
                    continue;
                }

                found_variants
                    .entry(to)
                    .and_modify(|neigh_current| {
                        // Insert the variant if it doesn't exist, else replace it if our cost is lower
                        neigh_current
                            .entry(candiate_variant)
                            .and_modify(|data| {
                                if current_variant_data.cost + cost < data.cost {
                                    data.cost = current_variant_data.cost + cost;
                                    data.predecessor = dir.reverse();
                                    variants_by_cost.push(Reverse(ByCostEntry {
                                        cost: current_variant_data.cost + cost,
                                        pos: to,
                                        variant: candiate_variant,
                                    }));
                                }
                            })
                            .or_insert_with(|| {
                                variants_by_cost.push(Reverse(ByCostEntry {
                                    cost: current_variant_data.cost + cost,
                                    pos: to,
                                    variant: candiate_variant,
                                }));
                                VariantData {
                                    cost: current_variant_data.cost + cost,
                                    predecessor: dir.reverse(),
                                }
                            });
                    })
                    .or_insert_with(|| {
                        // node has never been visited before, insert it
                        let mut new_node_map = HashMap::new();
                        new_node_map.insert(
                            candiate_variant,
                            VariantData {
                                cost: current_variant_data.cost + cost,
                                predecessor: dir.reverse(),
                            },
                        );
                        variants_by_cost.push(Reverse(ByCostEntry {
                            cost: current_variant_data.cost + cost,
                            pos: to,
                            variant: candiate_variant,
                        }));
                        new_node_map
                    });
            }
        }
        visited_variants.insert((current_pos, current_variant));
    }

    // Backtrack and find *one* possible path with the lowest cost
    found_variants
        .get(&goal)
        .unwrap()
        .values()
        .min_by_key(|variant| variant.cost)
        .map(|variant| variant.cost)
        .unwrap()
}

fn fill_graph(graph: &mut CrucibleGraph, input: &str) {
    let mut grid = SparseGrid::new();
    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, num) in row.char_indices() {
            grid.put(
                Position {
                    row: row_idx,
                    col: col_idx,
                },
                num.to_digit(10).unwrap(),
            );
        }
    }
    for (row_idx, row) in input.lines().enumerate() {
        // connect up, except for outer edges of course
        for (col_idx, num) in row.char_indices() {
            let pos = Position {
                row: row_idx,
                col: col_idx,
            };
            for dir in util::grid::Direction::iter() {
                if let Some(neigh) = grid.neighbour(&pos, dir) {
                    graph.add_edge(neigh.pos, pos, (num.to_digit(10).unwrap(), dir.reverse()));
                    graph.add_edge(pos, neigh.pos, (*neigh.element, dir));
                }
            }
        }
    }
}

const TEST: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut test_graph: CrucibleGraph = GraphMap::new();
    fill_graph(&mut test_graph, TEST);

    println!(
        "{}",
        crooked_dijkstra(
            &test_graph,
            Position { row: 0, col: 0 },
            Position { row: 12, col: 12 }
        )
    );

    let mut input_graph: CrucibleGraph = GraphMap::new();
    fill_graph(&mut input_graph, INPUT);

    println!(
        "{}",
        crooked_dijkstra(
            &input_graph,
            Position { row: 0, col: 0 },
            Position { row: 140, col: 140 }
        )
    );
}
