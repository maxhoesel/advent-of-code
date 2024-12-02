use std::ops::Range;

use itertools::Itertools;

const INPUT: &str = include_str!("INPUT.txt");

const ALLOWED_DIFF: Range<u32> = 1..4;

fn main() {
    let safe1 = INPUT
        .lines()
        .filter(|report| {
            let values = report
                .split_whitespace()
                .map(|v| v.parse::<i32>().expect("cannot parse into i32"))
                .collect_vec();
            let mut monotony_iter = values.windows(2);
            let init = monotony_iter.next().unwrap();
            let ascending = init[1] > init[0];
            values
                .windows(2)
                .all(|pair| ALLOWED_DIFF.contains(&pair[0].abs_diff(pair[1])))
                && monotony_iter.all(|pair| {
                    if ascending {
                        pair[1] > pair[0]
                    } else {
                        pair[1] < pair[0]
                    }
                })
        })
        .count();

    println!("Part1: {safe1}");

    let safe2 = INPUT
        .lines()
        .filter(|report| {
            let values = report
                .split_whitespace()
                .map(|v| v.parse::<i32>().expect("cannot parse into i32"))
                .collect_vec();

            let mut monotony_iter = values.windows(2);
            let init = monotony_iter.next().unwrap();
            let ascending = init[1] > init[0];
            let all_ok = values
                .windows(2)
                .all(|pair| ALLOWED_DIFF.contains(&pair[0].abs_diff(pair[1])))
                && monotony_iter.all(|pair| {
                    if ascending {
                        pair[1] > pair[0]
                    } else {
                        pair[1] < pair[0]
                    }
                });
            if all_ok {
                true
            } else {
                for i in 0..values.len() {
                    let mut values_dampened = values.clone();
                    values_dampened.remove(i);

                    let mut monotony_iter = values_dampened.windows(2);
                    let init = monotony_iter.next().unwrap();
                    let ascending = init[1] > init[0];
                    let ok = values_dampened
                        .windows(2)
                        .all(|pair| ALLOWED_DIFF.contains(&pair[0].abs_diff(pair[1])))
                        && monotony_iter.all(|pair| {
                            if ascending {
                                pair[1] > pair[0]
                            } else {
                                pair[1] < pair[0]
                            }
                        });
                    if ok {
                        return true;
                    }
                }
                false
            }
        })
        .count();
    println!("Part2: {safe2}");
}
