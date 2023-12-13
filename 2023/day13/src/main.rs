use std::str::FromStr;

use anyhow::anyhow;
use itertools::Itertools;

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

    fn find_line_pos(&self) -> Option<usize> {
        fn get_possible_mirror<I, J, E>(mut iter_a: I, mut iter_b: J) -> Option<usize>
        where
            E: Sized + Eq,
            I: Iterator<Item = E>,
            J: Iterator<Item = E>,
        {
            let Some(b_elem) = iter_b.next() else {
                return None; // there are no elements in b, no way to sync up
            };
            let mut init_lines = 0;
            // synchronize - walk a until a match occurs
            for a_elem in iter_a.by_ref() {
                init_lines += 1;
                if b_elem == a_elem {
                    break;
                }
            }

            // walk the synchronized bit
            let mut iters = 0;
            for (i, (a, b)) in iter_a.zip(iter_b).enumerate() {
                if a != b {
                    // a line doesn't match => no mirror
                    return None;
                }
                iters = i;
            }

            // at least one iterator has run out, we have reached an end and have some sort of mirroring
            Some(init_lines + iters / 2)
        }

        fn try_orientation(field: &[&str]) -> Option<usize> {
            // attempt 1: extra space at the top
            let top_down = field.iter();
            let bottom_up = field.iter().rev();
            if let Some(mirror_pos) = get_possible_mirror(top_down, bottom_up) {
                // verify that the mirror line is between two lines and not ON the line/edge
                if mirror_pos != 0
                    && mirror_pos != field.len()
                    && field[mirror_pos] == field[mirror_pos - 1]
                {
                    return Some(mirror_pos);
                }
            }

            // attempt 2: extra space at the bottom
            let top_down = field.iter();
            let bottom_up = field.iter().rev();
            if let Some(mut mirror_pos) = get_possible_mirror(bottom_up, top_down) {
                // inverted, so we need to flip the offset
                mirror_pos = field.len() - mirror_pos;
                // verify that the mirror line is between two lines and not ON the line/edge
                if mirror_pos != 0
                    && mirror_pos != field.len()
                    && field[mirror_pos] == field[mirror_pos - 1]
                {
                    return Some(mirror_pos);
                }
            }
            None
        }

        if let Some(pos) = try_orientation(&self.field) {
            return Some(pos * ROW_MULTIPLIER);
        }

        // create a vertical version of our grid to make iteration easier
        let transposed = self.transpose();
        let transposed_refs = transposed.iter().map(String::as_ref).collect_vec();
        try_orientation(&transposed_refs)
    }
}

const TEST: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

fn main() {
    let fields = TEST
        .split("\n\n")
        .map(|l| MirrorField::new(l))
        .collect_vec();
    let mirror_positions = fields
        .iter()
        .filter_map(|f| f.find_line_pos())
        .collect_vec();
    println!("Test: {:?}", mirror_positions);
    let mirror_positions: usize = fields.iter().filter_map(|f| f.find_line_pos()).sum();
    println!("Test Sum: {}", mirror_positions);

    let fields: Vec<MirrorField<'_>> = INPUT
        .split("\n\n")
        .map(|l| MirrorField::new(l))
        .collect_vec();
    let mirror_tests = fields
        .iter()
        .map(|f| (f, f.find_line_pos()))
        .filter(|l| l.1.is_none())
        .collect_vec();
    dbg!(mirror_tests);
    let mirror_positions: usize = fields.iter().filter_map(|f| f.find_line_pos()).sum();
    println!("Input Sum: {}", mirror_positions);
}
