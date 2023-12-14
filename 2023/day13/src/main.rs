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

    fn find_mirror_line(&self, clear_smudge: bool) -> usize {
        fn inner(field: &[&str], clear_smudge: bool) -> Option<usize> {
            for i in 0..=field.len() / 2 {
                let before_range = 0..i;
                let after_range = i..i + i;
                if before_range.is_empty() || after_range.is_empty() {
                    continue; // at very start or end
                }

                let before = &field[before_range];
                let after = &field[after_range];

                if clear_smudge {
                    // look for a minimal difference
                    let before = before.join("\n");
                    let after = after.iter().rev().cloned().collect_vec().join("\n");
                    if before
                        .chars()
                        .zip(after.chars())
                        .filter(|(before_c, after_c)| before_c != after_c)
                        .exactly_one()
                        .is_err()
                    {
                        continue; // no single smudge to clean
                    }
                } else {
                    // check if we have a mirror
                    if !before
                        .iter()
                        .zip(after.iter().rev())
                        .all(|(before, after)| before == after)
                    {
                        continue; // no reflection
                    }
                }

                return Some(i);
            }
            None
        }

        if let Some(pos) = inner(&self.field, clear_smudge) {
            return pos * ROW_MULTIPLIER;
        } else if let Some(pos) = inner(
            &self.field.iter().rev().copied().collect_vec(),
            clear_smudge,
        ) {
            return (self.field.len() - pos) * ROW_MULTIPLIER;
        } else {
            let transposed = self.transpose();
            let transposed_ref: Vec<&str> = transposed.iter().map(String::as_ref).collect();
            if let Some(pos) = inner(&transposed_ref, clear_smudge) {
                return pos;
            } else if let Some(pos) = inner(
                &transposed_ref.iter().rev().copied().collect_vec(),
                clear_smudge,
            ) {
                return transposed.len() - pos;
            }
        }
        panic!("No mirror found for field {:?}", self);
    }
}

const TEST: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

fn main() {
    let test_fields = TEST
        .split("\n\n")
        .map(|l| MirrorField::new(l))
        .collect_vec();
    let test_mirror_positions = test_fields
        .iter()
        .map(|f| f.find_mirror_line(false))
        .collect_vec();
    println!("Test: {:?}", test_mirror_positions);
    let test_mirror_sum: usize = test_mirror_positions.iter().sum();
    println!("Test Sum: {}", test_mirror_sum);

    let fields = INPUT
        .split("\n\n")
        .map(|l| MirrorField::new(l))
        .collect_vec();
    let mirror_positions = fields
        .iter()
        .map(|f| f.find_mirror_line(false))
        .collect_vec();
    println!("Input: {:?}", mirror_positions);
    let mirror_sum: usize = mirror_positions.iter().sum();
    println!("Input Sum: {}", mirror_sum);

    let test_smudge_lines = test_fields
        .iter()
        .map(|f| f.find_mirror_line(true))
        .collect_vec();
    println!("Test Smudges: {:?}", test_smudge_lines);
    let test_smudge_sum: usize = test_smudge_lines.iter().sum();
    println!("Test Smudges Sum: {}", test_smudge_sum);

    let smudge_lines = fields
        .iter()
        .map(|f| f.find_mirror_line(true))
        .collect_vec();
    println!("Input Smudges: {:?}", smudge_lines);
    let smudge_sum: usize = smudge_lines.iter().sum();
    println!("Input Smudges Sum: {}", smudge_sum);
}
