use std::{collections::HashMap, fmt::Display, fs, hash::Hash};

use color_eyre::{
    eyre::{eyre, Context, ContextCompat},
    Result,
};
use colored::{Color, ColoredString, Colorize};
use day8::grid::TreeGrid;
use itertools::Itertools;
use log::debug;

struct Visibility(Vec<Vec<bool>>);
impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for v in row {
                write!(
                    f,
                    "{}",
                    match v {
                        true => "X",
                        false => "-",
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct VisibilityScore(Vec<Vec<usize>>);
struct VisibilityColor(Vec<Vec<Color>>);
impl Display for VisibilityColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for v in row {
                let txt = "X".color(*v);
                write!(f, "{}", txt)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let input = fs::read_to_string("input.txt").wrap_err("Reading input.txt")?;

    let mut rows = Vec::new();
    for row in input.lines() {
        let mut rvec = Vec::with_capacity(row.len());
        for c in row.chars() {
            rvec.push(u8::try_from(
                c.to_digit(10)
                    .wrap_err(eyre!("Parsing char {} as height", c))?,
            )?);
        }
        rows.push(rvec);
    }

    let grid = TreeGrid::new(rows);

    let mut vis_map = Visibility(Vec::with_capacity(grid.height()));
    for row in 0..grid.height() {
        let mut vis_row = Vec::with_capacity(grid.width());
        for col in 0..grid.width() {
            match grid.at(row, col) {
                Some(t) => vis_row.push(t.is_visible(&grid)),
                None => vis_row.push(false),
            }
        }
        vis_map.0.push(vis_row);
    }

    println!("Visibility:\n{}", vis_map);

    let total: usize = vis_map.0.iter().flatten().map(|v| usize::from(*v)).sum();
    println!("Total amount of visible trees: {}", total);

    let mut score_map = VisibilityScore(Vec::with_capacity(grid.height()));
    for row in 0..grid.height() {
        let mut score_row = Vec::with_capacity(grid.width());
        for col in 0..grid.width() {
            match grid.at(row, col) {
                Some(t) => score_row.push(t.visibility_score(&grid)),
                None => score_row.push(0),
            }
        }
        score_map.0.push(score_row);
    }

    let vis_scores = score_map.0.iter().flatten().sorted().collect_vec();
    let highest = vis_scores.last().unwrap();
    println!("Highest Visibility: {}", highest);

    let colors = vec![Color::Red, Color::Yellow, Color::Green, Color::Blue];
    let unique_scores = vis_scores.iter().unique().collect_vec();
    let color_ranges: HashMap<_, &Color> = unique_scores
        .chunks(unique_scores.len() / colors.len())
        .enumerate()
        .map(|(i, chunk)| {
            (
                chunk.first().unwrap()..=chunk.last().unwrap(),
                colors.get(i).unwrap_or(&Color::White),
            )
        })
        .collect::<HashMap<_, _>>();
    debug!("{:?}", color_ranges);

    let mut color_map = VisibilityColor(Vec::with_capacity(grid.height()));
    for row in 0..grid.height() {
        let mut color_row: Vec<Color> = Vec::with_capacity(grid.width());
        for col in 0..grid.width() {
            match score_map.0.get(row).unwrap_or(&vec![]).get(col) {
                Some(score) => {
                    for r in &color_ranges {
                        if r.0.contains(&&&score) {
                            color_row.push(*r.1.to_owned());
                            break;
                        }
                    }
                }
                None => color_row.push(Color::Black),
            }
        }
        color_map.0.push(color_row);
    }

    println!("Visibility Score:\n{}", color_map);

    Ok(())
}
