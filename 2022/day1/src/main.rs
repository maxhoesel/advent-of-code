use rayon::prelude::*;
use tokio::fs::read_to_string;

#[tokio::main]
async fn main() {
    let input = read_to_string("input.txt").await.unwrap();

    let inventories: Vec<_> = input.split("\n\n").collect();

    let mut calorie_counts = inventories
        .par_iter()
        .map(|elf_inventory| {
            elf_inventory
                .split('\n')
                .map(|line| line.parse::<usize>().unwrap_or(0))
                .sum::<usize>()
        })
        .collect::<Vec<_>>();
    calorie_counts.sort_unstable();

    let top_3 = &calorie_counts[calorie_counts.len() - 3..];

    println!(
        "Highest 3 elf calorie counts: {}",
        top_3.iter().sum::<usize>()
    );
}
