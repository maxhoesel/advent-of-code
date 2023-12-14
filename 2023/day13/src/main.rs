use std::{collections::binary_heap::Iter, str::FromStr};

use anyhow::anyhow;
use itertools::{Iterate, Itertools};

const ROW_MULTIPLIER: usize = 100;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct MirrorField<'a> {
    field: Vec<&'a str>,
}
impl MirrorField<'_> {
    fn new(input: &str) -> MirrorField<'_> {
        let lines = input.lines().collect_vec();
        MirrorField { field: lines }
    }

    fn transpose(&self) -> Vec<String> {
        let cols = self.field[0].len();

        (0_usize..cols)
            .map(|col_index| {
                self.field
                    .iter()
                    .map(|row| row.chars().nth(col_index).expect("Invalid matrix"))
                    .collect()
            })
            .collect()
    }

    fn find_mirrors(&self) -> usize {
        fn get_mirror<E>(field: &[E], reversed: bool) -> Option<usize>
        where
            E: Sized + Eq,
        {
            // i am tired of this shit, just give me some iterators even it means allocating
            let fwd = field.iter().collect_vec();
            let back = field.iter().rev().collect_vec();
            let mut back_iter = if reversed { fwd.iter() } else { back.iter() };
            let mut fwd_iter = if reversed { back.iter() } else { fwd.iter() };

            let Some(b_elem) = back_iter.next() else {
                return None; // there are no elements in b, no way to sync up
            };

            let mut preceeding_lines = 0;
            while let Some(a_elem) = fwd_iter.next() {
                preceeding_lines += 1;

                if b_elem == a_elem {
                    // possibility for synchronization, try to the end
                    let fwd = fwd_iter.clone();
                    let back = back_iter.clone();
                    let mirror_pos: Option<usize> = {
                        let mut iters = 0;
                        let mut synchronized = true;
                        for (i, (a, b)) in fwd.zip(back).enumerate() {
                            iters = i;
                            if a != b {
                                synchronized = false;
                                break;
                            }
                        }
                        if !synchronized {
                            None
                        } else {
                            let line_pos = if reversed {
                                field.len() - (preceeding_lines + iters / 2)
                            } else {
                                preceeding_lines + iters / 2
                            };
                            if line_pos != 0
                                && line_pos < field.len()
                                && field[line_pos] == field[line_pos - 1]
                            {
                                Some(line_pos)
                            } else {
                                None
                            }
                        }
                    };
                    if let Some(mirror) = mirror_pos {
                        return Some(mirror);
                    }
                }
            }
            None
        }

        if let Some(mirror) = get_mirror(&self.field, false) {
            return mirror * ROW_MULTIPLIER;
        } else if let Some(mirror) = get_mirror(&self.field, true) {
            return mirror * ROW_MULTIPLIER;
        } else {
            let transposed = self.transpose();
            if let Some(mirror) = get_mirror(&transposed, false) {
                return mirror;
            } else if let Some(mirror) = get_mirror(&transposed, true) {
                return mirror;
            }
        }
        panic!("No mirror found");
    }

    fn smudge_line(&self) -> usize {
        dbg!(&self);
        fn get_smudge(field: &[&str], reversed: bool) -> Option<usize> {
            // i am tired of this shit, just give me some iterators even it means allocating
            let fwd = field.iter().collect_vec();
            let back = field.iter().rev().collect_vec();
            let mut back_iter = if reversed { fwd.iter() } else { back.iter() };
            let mut fwd_iter = if reversed { back.iter() } else { fwd.iter() };

            let Some(b_elem) = back_iter.next() else {
                return None; // there are no elements in b, no way to sync up
            };

            let mut preceeding_lines = 0;
            while let Some(a_elem) = fwd_iter.next() {
                preceeding_lines += 1;

                if a_elem == b_elem {
                    continue;
                }

                let diff_idxes = a_elem
                    .char_indices()
                    .zip(b_elem.chars())
                    .filter_map(|zip| if zip.0 .1 != zip.1 { Some(zip.0) } else { None })
                    .collect_vec();
                if diff_idxes.len() != 1 {
                    continue; // no single smudge, can't be
                }
                let diff_elem = diff_idxes[0];
                // create a un-smudged version and store it for loop reference
                let mut cleaned_elem = a_elem.to_string();
                cleaned_elem.replace_range(
                    (diff_elem.0)..=(diff_elem.0),
                    if diff_elem.1 == '#' { "." } else { "#" },
                );

                // possibility for synchronization, try to the end
                let fwd = fwd_iter.clone();
                let back = back_iter.clone();
                let mirror_pos: Option<usize> = {
                    let mut iters = 0;
                    let mut synchronized = true;
                    for (i, (a, b)) in fwd.zip(back).enumerate() {
                        iters = i;
                        // at then end of the iteration, this is where we need to substitute our cleaned line for b
                        if a != b {
                            if iters + preceeding_lines + 1 == field.len()
                                && a == &&cleaned_elem.as_str()
                            {
                            } else {
                                synchronized = false;
                                break;
                            }
                        }
                    }
                    if !synchronized {
                        None
                    } else {
                        let line_pos = if reversed {
                            field.len() - (preceeding_lines + iters / 2)
                        } else {
                            preceeding_lines + iters / 2
                        };
                        let mut tmpfield = field.to_owned();
                        if reversed {
                            tmpfield[field.len() - preceeding_lines] = cleaned_elem.as_str();
                        } else {
                            tmpfield[preceeding_lines - 1] = cleaned_elem.as_str();
                        }

                        if line_pos != 0
                            && line_pos < field.len()
                            && tmpfield[line_pos] == tmpfield[line_pos - 1]
                        {
                            Some(line_pos)
                        } else {
                            None
                        }
                    }
                };
                if let Some(mirror) = mirror_pos {
                    return Some(mirror);
                }
            }
            None
        }

        if let Some(mirror) = get_smudge(&self.field, false) {
            return mirror * ROW_MULTIPLIER;
        } else if let Some(mirror) = get_smudge(&self.field, true) {
            return mirror * ROW_MULTIPLIER;
        } else {
            let transposed = self.transpose();
            let transposed_refs = transposed.iter().map(String::as_ref).collect_vec();
            if let Some(mirror) = get_smudge(&transposed_refs, false) {
                return mirror;
            } else if let Some(mirror) = get_smudge(&transposed_refs, true) {
                return mirror;
            }
        }
        panic!("No mirror found");
    }
}

const TEST: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

fn main() {
    let test_fields = TEST
        .split("\n\n")
        .map(|l| MirrorField::new(l))
        .collect_vec();
    let test_mirror_positions = test_fields.iter().map(|f| f.find_mirrors()).collect_vec();
    println!("Test: {:?}", test_mirror_positions);
    let test_mirror_sum: usize = test_mirror_positions.iter().sum();
    println!("Test Sum: {}", test_mirror_sum);

    let fields = INPUT
        .split("\n\n")
        .map(|l| MirrorField::new(l))
        .collect_vec();
    let mirror_positions = fields.iter().map(|f| f.find_mirrors()).collect_vec();
    println!("Input: {:?}", mirror_positions);
    let mirror_sum: usize = mirror_positions.iter().sum();
    println!("Input Sum: {}", mirror_sum);

    let test_smudge_lines = test_fields.iter().map(|f| f.smudge_line()).collect_vec();
    println!("Test Smudges: {:?}", test_smudge_lines);
    let test_smudge_sum: usize = test_smudge_lines.iter().sum();
    println!("Test Smudges Sum: {}", test_smudge_sum);

    let smudge_lines = fields.iter().map(|f| f.smudge_line()).collect_vec();
    println!("Input Smudges: {:?}", smudge_lines);
    let smudge_sum: usize = smudge_lines.iter().sum();
    println!("Input Smudges Sum: {}", smudge_sum);
}
