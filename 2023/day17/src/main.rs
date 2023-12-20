use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use petgraph::{
    data::Build, dot::Dot, graph::Node, graphmap::GraphMap, Directed, Direction::Outgoing,
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

fn crooked_dijkstra(graph: &CrucibleGraph, start: Position, goal: Position) -> u32 {
    let mut found: HashMap<Position, HashMap<NodeVariant, VariantData>> = HashMap::new();
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
    found.insert(start, start_map);

    let mut visited: HashSet<(Position, NodeVariant)> = HashSet::new();

    loop {
        // find the next lowest value between horizontal and vertical
        let (current_pos, current_variant, current_variant_data) = found
            .iter()
            .flat_map(|(pos, variants)| {
                variants.iter().filter_map(|(variant, node)| {
                    if !visited.contains(&(*pos, *variant)) {
                        Some((*pos, *variant, *node))
                    } else {
                        None
                    }
                })
            })
            .min_by_key(|entry| entry.2.cost)
            .unwrap();

        if found.contains_key(&goal)
            && current_variant_data.cost
                > found
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

            // for each one: variant visited? => skip. then, variant exists? => if not add, else compare costs
            for candiate_variant in possible_variants {
                if visited.contains(&(to, candiate_variant)) {
                    // this variant has already been visited, no way it could be any better
                    continue;
                }
                if let Some(neigh_current) = found.get_mut(&to) {
                    // Insert the variant if it doesn't exist, else replace it if our cost is lower
                    neigh_current
                        .entry(candiate_variant)
                        .and_modify(|data| {
                            if current_variant_data.cost + cost < data.cost {
                                data.cost = current_variant_data.cost + cost;
                                data.predecessor = dir.reverse();
                            }
                        })
                        .or_insert(VariantData {
                            cost: current_variant_data.cost + cost,
                            predecessor: dir.reverse(),
                        });
                }
                // node has never been visited before, insert it
                let mut new_node_map = HashMap::new();
                new_node_map.insert(
                    candiate_variant,
                    VariantData {
                        cost: current_variant_data.cost + cost,
                        predecessor: dir.reverse(),
                    },
                );
                found.insert(to, new_node_map);
            }
        }
        visited.insert((current_pos, current_variant));
    }

    // Backtrack and find *one* possible path with the lowest cost
    found
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
const TEST2: &str = include_str!("test2.txt");

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

    let mut test2_graph: CrucibleGraph = GraphMap::new();
    fill_graph(&mut test2_graph, TEST2);

    println!(
        "{}",
        crooked_dijkstra(
            &test2_graph,
            Position { row: 0, col: 0 },
            Position { row: 0, col: 7 }
        )
    );
}
