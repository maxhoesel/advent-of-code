use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use petgraph::{graph::Node, graphmap::GraphMap, Directed, Direction::Outgoing};

use strum::IntoEnumIterator;
use util::grid::{Direction, Orientation, Position, SparseGrid};

type CrucibleGraph = GraphMap<Position, (u32, Direction), Directed>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct NodeVariant {
    orientation: Orientation,
    straight_steps: u32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct VariantData {
    cost: u32,
    predecessor: Direction,
    path: Vec<Direction>,
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

fn crooked_dijkstra(
    graph: &CrucibleGraph,
    start: Position,
    goal: Position,
    min_straight: u32,
    max_straight: u32,
) -> (u32, Vec<Direction>) {
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
            path: vec![],
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
            path: vec![],
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
                    found_variants
                        .get(&next.pos)
                        .unwrap()
                        .get(&next.variant)
                        .unwrap()
                        .clone(),
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

        for (_, to, (cost, dir)) in
            graph
                .edges_directed(current_pos, Outgoing)
                .filter(|(_, _, (_, dir))| {
                    // we only care about edges that are relevant to our variants orientation
                    current_variant.orientation == dir.orientation()
                    // cannot reverse
                    && current_variant_data.predecessor != *dir
                })
        {
            // construct the possible variants for the connecting node
            let mut possible_variants = vec![];
            // we can turn as long as we went our minimum straight distance...
            if current_variant.straight_steps + 1 >= min_straight
                && (
                    // ...and if turning doesn't cause us to overshoot the target.
                    (current_variant.orientation == Orientation::Horizontal
                        && goal.row.abs_diff(to.row) >= min_straight as usize)
                        || (current_variant.orientation == Orientation::Vertical
                            && goal.col.abs_diff(to.col) >= min_straight as usize)
                )
            {
                possible_variants.push(NodeVariant {
                    orientation: current_variant.orientation.flip(),
                    straight_steps: 0,
                })
            }
            if current_variant.straight_steps + 1 < max_straight {
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

                if to == goal && candiate_variant.straight_steps < min_straight {
                    // we cannot end unless we have gone at least min_straight steps
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
                                    path: {
                                        let mut x = current_variant_data.path.clone();
                                        x.push(*dir);
                                        x
                                    },
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
                                path: {
                                    let mut x = current_variant_data.path.clone();
                                    x.push(*dir);
                                    x
                                },
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
    let final_variant = found_variants
        .get(&goal)
        .unwrap()
        .values()
        .min_by_key(|variant| variant.cost)
        .unwrap();
    (final_variant.cost, final_variant.path.clone())
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

fn reconstruct_path<'a>(directions: &'a [Direction], start: &Position) -> SparseGrid<&'a str> {
    let mut path_grid = SparseGrid::new();
    let mut row = 0;
    let mut col = 0;
    path_grid.put(*start, "X");
    for step in directions {
        match step {
            Direction::Up => {
                row -= 1;
                path_grid.put(Position { row, col }, "^");
            }
            Direction::Down => {
                row += 1;
                path_grid.put(Position { row, col }, "v");
            }
            Direction::Left => {
                col -= 1;
                path_grid.put(Position { row, col }, "<");
            }
            Direction::Right => {
                col += 1;
                path_grid.put(Position { row, col }, ">");
            }
        };
    }
    path_grid
}

const TEST: &str = include_str!("test.txt");
const TEST2: &str = include_str!("test2.txt");
const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut test_graph: CrucibleGraph = GraphMap::new();
    fill_graph(&mut test_graph, TEST);
    let test_res1 = crooked_dijkstra(
        &test_graph,
        Position { row: 0, col: 0 },
        Position { row: 12, col: 12 },
        0,
        3,
    );
    println!("Part 1 test: {}", test_res1.0);
    println!(
        "part 1 test Grid:\n{}",
        reconstruct_path(&test_res1.1, &Position { row: 0, col: 0 })
    );

    let test_res2 = crooked_dijkstra(
        &test_graph,
        Position { row: 0, col: 0 },
        Position { row: 12, col: 12 },
        4,
        10,
    );
    println!("Part 2 test: {}", test_res2.0);
    println!(
        "part 2 test Grid:\n{}",
        reconstruct_path(&test_res2.1, &Position { row: 0, col: 0 })
    );

    let mut test2_graph: CrucibleGraph = GraphMap::new();
    fill_graph(&mut test2_graph, TEST2);
    let test2_res = crooked_dijkstra(
        &test2_graph,
        Position { row: 0, col: 0 },
        Position { row: 4, col: 11 },
        4,
        10,
    );
    println!("Part 2 test 2: {}", test2_res.0);
    println!(
        "part 2 test 2 Grid:\n{}",
        reconstruct_path(&test2_res.1, &Position { row: 0, col: 0 })
    );

    let mut input_graph: CrucibleGraph = GraphMap::new();
    fill_graph(&mut input_graph, INPUT);
    let input_res1 = crooked_dijkstra(
        &input_graph,
        Position { row: 0, col: 0 },
        Position { row: 140, col: 140 },
        0,
        3,
    );

    println!("Part 1 result {}", input_res1.0);

    let input_res2 = crooked_dijkstra(
        &input_graph,
        Position { row: 0, col: 0 },
        Position { row: 140, col: 140 },
        4,
        10,
    );

    println!("Part 2 result {}", input_res2.0);
    println!(
        "part 2 Grid:\n{}",
        reconstruct_path(&input_res2.1, &Position { row: 0, col: 0 })
    );
}
