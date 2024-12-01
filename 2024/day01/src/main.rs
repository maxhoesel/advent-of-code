use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let nums = INPUT
        .lines()
        .map(|s| s.split_whitespace())
        .map(|mut p| {
            let a: i64 = p.next().unwrap().parse().unwrap();
            let b: i64 = p.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect_vec();
    let left = nums.iter().map(|(a, _)| *a).sorted().collect_vec();
    let right = nums.iter().map(|(_, b)| *b).sorted().collect_vec();
    let part1: i64 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();
    println!("{part1}");

    let right_appearances = right
        .iter()
        .fold(HashMap::<i64, i64>::new(), |mut acc, elem| {
            acc.entry(*elem).and_modify(|c| *c += 1).or_insert(1);
            acc
        });

    let part2: i64 = left
        .iter()
        .map(|l| l * right_appearances.get(l).unwrap_or(&0))
        .sum();
    println!("{part2}");
}
