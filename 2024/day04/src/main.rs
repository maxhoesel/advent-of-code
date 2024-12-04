use std::sync::LazyLock;

use itertools::Itertools;
use nalgebra::{DMatrix, Matrix3};

const INPUT: &str = include_str!("input.txt");
const SEARCH: &str = "XMAS";
const SEARCH_BACKWARDS: &str = "SAMX";

static X_MAS: LazyLock<Vec<Matrix3<char>>> = LazyLock::new(|| {
    vec![
        Matrix3::from_iterator(['M', '.', 'M', '.', 'A', '.', 'S', '.', 'S']),
        Matrix3::from_iterator(['M', '.', 'S', '.', 'A', '.', 'M', '.', 'S']),
        Matrix3::from_iterator(['S', '.', 'M', '.', 'A', '.', 'S', '.', 'M']),
        Matrix3::from_iterator(['S', '.', 'S', '.', 'A', '.', 'M', '.', 'M']),
    ]
});
const X_MAS_IDXS: &[(usize, usize)] = &[(0, 0), (0, 2), (1, 1), (2, 0), (2, 2)];

fn main() {
    let size = INPUT.lines().count();
    let mut chars = INPUT.to_string();
    chars.retain(|c| "XMAS".contains(c));
    let chars = chars.chars().collect_vec();

    let left_right = {
        let mut lines = vec![];
        for row in 0..size {
            let offset = size * row;
            lines.push(chars[offset..offset + size].iter().collect::<String>());
        }
        lines
    };
    let top_bottom = {
        let mut cols = vec![];
        for col in 0..size {
            let mut col_str = String::new();
            for row in 0..size {
                let offset = size * row;
                col_str.push(chars[col + offset]);
            }
            cols.push(col_str);
        }
        cols
    };
    let topleft_downright = {
        let mut diags = vec![];
        // first loop, bottom left half and prime diagonal
        for diag in 0..size {
            let mut diag_str = String::new();
            let mut row = size - 1 - diag;
            let mut col = 0;
            while row < size {
                diag_str.push(chars[row * size + col]);
                row += 1;
                col += 1;
            }
            diags.push(diag_str);
        }

        // second loop, top-right half
        for diag in 1..size {
            let mut diag_str = String::new();
            let mut row = 0;
            let mut col = diag;
            while col < size {
                diag_str.push(chars[row * size + col]);
                row += 1;
                col += 1;
            }
            diags.push(diag_str);
        }
        diags
    };
    let downleft_topright = {
        let mut diags = vec![];
        // first loop, top left half and prime diagonal
        for diag in 0..size {
            let mut diag_str = String::new();
            let mut row = diag as isize;
            let mut col = 0;
            while row >= 0 {
                diag_str.push(chars[row as usize * size + col]);
                row -= 1;
                col += 1;
            }
            diags.push(diag_str);
        }

        // second loop, bottom-right half
        for diag in 1..size {
            let mut diag_str = String::new();
            let mut row = size - 1;
            let mut col = diag;
            while col < size {
                diag_str.push(chars[row * size + col]);
                row -= 1;
                col += 1;
            }
            diags.push(diag_str);
        }
        diags
    };

    let total = top_bottom
        .iter()
        .chain(left_right.iter())
        .chain(topleft_downright.iter())
        .chain(downleft_topright.iter())
        .fold(0, |acc, text| {
            acc + text.matches(SEARCH).count() + text.matches(SEARCH_BACKWARDS).count()
        });
    println!("{total}");

    let mat = DMatrix::from_iterator(size, size, chars);
    let mut found = 0;
    for row in 1..size - 1 {
        for col in 1..size - 1 {
            let view = mat.view((row - 1, col - 1), (3, 3));
            for mat in &*X_MAS {
                let mut matched = true;
                for idx in X_MAS_IDXS {
                    if view.get(*idx).expect("View") != mat.get(*idx).expect("lil guy") {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    found += 1;
                    break;
                }
            }
        }
    }
    println!("{found}");
}
