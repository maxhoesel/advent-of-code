use std::{collections::HashMap, fs::read_to_string};

use color_eyre::{eyre::eyre, Result};
use day7::{filetree::DirTree, parser::parse_terminal};
use itertools::Itertools;

const LIMIT: u64 = 100000;
const NEEDED_FREE: u64 = 30000000;
const TOTAL_FS: u64 = 70000000;

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let input = read_to_string("input.txt")?;

    let commands = match parse_terminal(&input) {
        Ok(c) => c,
        Err(e) => return Err(eyre!("Parse Error: {}", e)),
    };

    let tree = DirTree::build(&commands);
    let under_limit_total: u64 = tree
        .get()
        .traverse_pre_order(tree.get().root_node_id().unwrap())
        .unwrap()
        .filter_map(|n| {
            let size = tree.get_dir_size(n);
            if size <= LIMIT {
                Some(size)
            } else {
                None
            }
        })
        .sum();
    println!("Total under limit: {}", under_limit_total);

    let total_used: u64 =
        tree.get_dir_size(tree.get().get(tree.get().root_node_id().unwrap()).unwrap());
    let delete_at_least = NEEDED_FREE - (TOTAL_FS - total_used);
    let delete_dir_with_size: u64 = tree
        .get()
        .traverse_pre_order(tree.get().root_node_id().unwrap())
        .unwrap()
        .filter_map(|n| {
            let size = tree.get_dir_size(n);
            if size >= delete_at_least {
                Some(size)
            } else {
                None
            }
        })
        .sorted()
        .next()
        .unwrap();
    println!("Delete with size: {}", delete_dir_with_size);

    Ok(())
}
