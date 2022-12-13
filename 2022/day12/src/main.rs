use std::{collections::HashMap, fmt::Display, fs::read_to_string, vec};

use bimap::BiMap;
use color_eyre::{eyre::Context, Result};
use day12::grid::{Cell, Grid};
use itertools::Itertools;
use petgraph::{
    algo::{astar, dijkstra},
    data::Build,
    dot::{Config, Dot},
    graph::Node,
    graph::NodeIndex,
    prelude::DiGraph,
    stable_graph::IndexType,
};

const LOWEST_ELEVATION: u8 = 'a' as u8;

#[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
struct Path<'a>(Vec<&'a Cell>);
impl<'a> Display for Path<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, c) in self.0.iter().enumerate() {
            if i + 1 == self.0.len() {
                write!(f, "{}", c)?
            } else {
                write!(f, "{} -> ", c)?
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let input = read_to_string("input.txt").wrap_err("Reading input.txt")?;

    let height = input.lines().count();
    let width = input.lines().take(1).map(|l| l.len()).sum();
    let mut grid = Grid::new(width, height);

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let elevation = match c {
                'S' => {
                    grid.set_start(x, y);
                    b'a'
                }
                'E' => {
                    grid.set_end(x, y);
                    b'z'
                }
                c if c.is_ascii_lowercase() => c as u8,
                _ => panic!(),
            };
            grid.insert_cell(x, y, elevation);
        }
    }

    let mut graph: DiGraph<Cell, u32> = DiGraph::new();
    let mut node_cells: BiMap<&Cell, NodeIndex> = BiMap::new();
    let start = graph.add_node(*grid.start().unwrap());
    node_cells.insert(grid.start().unwrap(), start);
    let end = graph.add_node(*grid.end().unwrap());
    node_cells.insert(grid.end().unwrap(), end);
    for c in grid
        .cells()
        .filter(|c| c != &grid.start().unwrap() && c != &grid.end().unwrap())
    {
        let idx = graph.add_node(*c);
        node_cells.insert(c, idx);
    }

    let edges = node_cells
        .iter()
        .flat_map(|(cell, idx)| {
            let top_edge = grid.cell_top(cell).map(|to| {
                (
                    *idx,
                    *node_cells.get_by_left(to).unwrap(),
                    cell.cost_to(to).map(|c| c as u32),
                )
            });
            let bot_edge = grid.cell_bot(cell).map(|to| {
                (
                    *idx,
                    *node_cells.get_by_left(to).unwrap(),
                    cell.cost_to(to).map(|c| c as u32),
                )
            });
            let left_edge = grid.cell_left(cell).map(|to| {
                (
                    *idx,
                    *node_cells.get_by_left(to).unwrap(),
                    cell.cost_to(to).map(|c| c as u32),
                )
            });
            let right_edge = grid.cell_right(cell).map(|to| {
                (
                    *idx,
                    *node_cells.get_by_left(to).unwrap(),
                    cell.cost_to(to).map(|c| c as u32),
                )
            });
            vec![top_edge, bot_edge, left_edge, right_edge]
                .into_iter()
                .flatten()
                .filter_map(|edge| edge.2.map(|cost| (edge.0, edge.1, cost)))
                .collect_vec()
        })
        .collect_vec();

    graph.extend_with_edges(edges);

    let res = astar(&graph, start, |n| n == end, |e| *e.weight(), |_| 0).unwrap();
    println!("Total Cost: {}", res.0);

    let path = Path(
        res.1
            .iter()
            .map(|idx| *node_cells.get_by_right(idx).unwrap())
            .collect_vec(),
    );

    println!("Path up the hill: {}", path);

    let trail_starts = node_cells
        .iter()
        .filter(|(cell, _)| cell.elevation == LOWEST_ELEVATION)
        .collect_vec();
    let trail_tracks = trail_starts
        .iter()
        .filter_map(|(cell, _)| {
            astar(
                &graph,
                node_cells.get_by_left(*cell).unwrap().to_owned(),
                |n| n == end,
                |e| *e.weight(),
                |_| 0,
            )
            .map(|r| r.0)
        })
        .sorted()
        .collect_vec();
    let best_trail = trail_tracks.first().unwrap();
    println!("Best hiking trail cost: {}", best_trail);

    Ok(())
}
