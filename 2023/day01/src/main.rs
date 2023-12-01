use std::collections::HashMap;

use anyhow::{Context, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::prelude::*;

const TEST1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
const TEST2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
const INPUT: &str = include_str!("input1.txt");

lazy_static! {
    static ref WRITTEN_DIGITS: HashMap<&'static str, u64> = {
        let mut m = HashMap::new();
        m.insert("zero", 0);
        m.insert("one", 1);
        m.insert("two", 2);
        m.insert("three", 3);
        m.insert("four", 4);
        m.insert("five", 5);
        m.insert("six", 6);
        m.insert("seven", 7);
        m.insert("eight", 8);
        m.insert("nine", 9);
        m
    };
}

fn find_sum_numbers_only(input: &str) -> u64 {
    input
        .par_lines()
        .map(|line| {
            let first = line
                .chars()
                .find(|c| c.is_ascii_digit())
                .context(format!("Reading line {line}"))
                .unwrap();
            let last = line
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .context(format!("Reading line {line}"))
                .unwrap();

            format!("{first}{last}").parse::<u64>().unwrap()
        })
        .sum()
}

fn find_first_num_or_written(line: &str) -> u64 {
    for i in 0..line.len() {
        let (_, teststr) = line.split_at(i);
        if let Some(num) = teststr.chars().next().unwrap().to_digit(10) {
            return num.into();
        }
        for digit in WRITTEN_DIGITS.iter() {
            if teststr.starts_with(digit.0) {
                return *digit.1;
            }
        }
    }
    panic!("No valid digit found in line {line}");
}

fn find_last_num_or_written(line: &str) -> u64 {
    let mut last_match: Option<u64> = None;
    for i in 0..line.len() {
        let (_, teststr) = line.split_at(i);
        if let Some(num) = teststr.chars().next().unwrap().to_digit(10) {
            last_match = Some(num.into());
        }
        for digit in WRITTEN_DIGITS.iter() {
            if teststr.starts_with(digit.0) {
                last_match = Some(*digit.1);
            }
        }
    }
    last_match
        .context(format!("Reading last match for line {line}"))
        .unwrap()
}

fn find_sum_num_or_written(input: &str) -> u64 {
    input
        .par_lines()
        .map(|line| {
            let first = find_first_num_or_written(line);
            let last = find_last_num_or_written(line);
            format!("{}{}", first, last).parse::<u64>().unwrap()
        })
        .sum()
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", find_sum_numbers_only(TEST1));
    println!("{}", find_sum_numbers_only(INPUT));
    println!("{}", find_sum_num_or_written(TEST2));
    println!("{}", find_sum_num_or_written(INPUT));
    Ok(())
}
