use rayon::prelude::*;
use tokio::fs::read_to_string;

enum Play {
    Rock,
    Paper,
    Scissors,
}
impl From<char> for Play {
    fn from(c: char) -> Self {
        match c {
            'A' => Play::Rock,
            'B' => Play::Paper,
            'C' => Play::Scissors,
            _ => panic!(),
        }
    }
}
enum Goal {
    Lose,
    Tie,
    Win,
}
impl From<char> for Goal {
    fn from(c: char) -> Self {
        match c {
            'X' => Goal::Lose,
            'Y' => Goal::Tie,
            'Z' => Goal::Win,
            _ => panic!(),
        }
    }
}

#[tokio::main]
async fn main() {
    let input = read_to_string("input.txt").await.unwrap();

    let score: usize = input
        .par_lines()
        .map(|l| {
            let play: Play = l.chars().next().unwrap().into();
            let goal: Goal = l.chars().nth(2).unwrap().into();
            match (play, goal) {
                (Play::Rock, Goal::Lose) => 3,     // Scissors
                (Play::Rock, Goal::Tie) => 4,      // Rock
                (Play::Rock, Goal::Win) => 8,      // Paper
                (Play::Paper, Goal::Lose) => 1,    // Rock
                (Play::Paper, Goal::Tie) => 5,     // Paper
                (Play::Paper, Goal::Win) => 9,     // Scissors
                (Play::Scissors, Goal::Lose) => 2, // Paper
                (Play::Scissors, Goal::Tie) => 6,  // Scissors
                (Play::Scissors, Goal::Win) => 7,  // Rock
            }
        })
        .sum();

    println!("Total score: {}", score);
}
