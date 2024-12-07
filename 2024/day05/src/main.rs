use std::collections::HashMap;

use color_eyre::eyre::Result;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const SAMPLE: &str = include_str!("sample.txt");

fn main() -> Result<()> {
    let (rules, updates) = INPUT.split_once("\n\n").unwrap();
    let (befores, afters) = rules
        .lines()
        .map(|l| {
            let (before, after) = l.split_once("|").unwrap();
            (before.parse::<u8>().unwrap(), after.parse::<u8>().unwrap())
        })
        .fold(
            (HashMap::new(), HashMap::new()),
            |(mut befores, mut afters), rule| {
                befores
                    .entry(rule.1)
                    .and_modify(|nums: &mut Vec<u8>| nums.push(rule.0))
                    .or_insert_with(|| vec![rule.0]);
                afters
                    .entry(rule.0)
                    .and_modify(|nums: &mut Vec<u8>| nums.push(rule.1))
                    .or_insert_with(|| vec![rule.1]);
                (befores, afters)
            },
        );
    let updates = updates
        .lines()
        .map(|l| {
            l.split(",")
                .map(|num| num.parse::<u8>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut total: u64 = 0;
    for update in updates {
        let mut valid = true;
        'update_check: for (idx, page) in update.iter().enumerate() {
            for preceding_page in update[0..idx].iter() {
                let Some(forbidden) = befores.get(preceding_page) else {
                    continue;
                };
                if forbidden.contains(page) {
                    valid = false;
                    break 'update_check;
                }
            }
            for terminating_page in update[idx + 1..update.len()].iter() {
                let Some(forbidden) = afters.get(terminating_page) else {
                    continue;
                };
                if forbidden.contains(page) {
                    valid = false;
                    break 'update_check;
                }
            }
        }
        if valid {
            total += update[update.len() / 2] as u64;
        }
    }

    println!("{total}");
    Ok(())
}
