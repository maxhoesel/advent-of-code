use std::{clone, collections::HashMap, fmt::Display, hash::Hash};

use id_tree::{Node, NodeId, Tree, TreeBuilder};
use itertools::Itertools;
use log::debug;

use crate::parser::{Command, Listing, Move};

#[derive(Clone, Debug)]
pub struct Dir {
    pub name: String,
    pub files: HashMap<String, u64>,
}

#[derive(Clone, Debug)]
pub struct DirTree {
    tree: Tree<Dir>,
}
impl DirTree {
    pub fn get(&self) -> &Tree<Dir> {
        &self.tree
    }

    pub fn build(cmds: &[Command]) -> Self {
        use id_tree::InsertBehavior::*;

        let mut tree = Tree::new();

        let mut current = match cmds.get(0).unwrap() {
            Command::ChangeDir(Move::To(root)) => tree
                .insert(
                    Node::new(Dir {
                        name: root.to_string(),
                        ..Default::default()
                    }),
                    AsRoot,
                )
                .unwrap(),
            _ => panic!(),
        };

        for cmd in cmds {
            match cmd {
                Command::ChangeDir(mv) => match mv {
                    Move::To(dest) => {
                        current = tree
                            .insert(
                                Node::new(Dir {
                                    name: dest.to_string(),
                                    ..Default::default()
                                }),
                                UnderNode(&current),
                            )
                            .unwrap()
                    }
                    Move::Out => current = tree.get(&current).unwrap().parent().unwrap().clone(),
                },
                Command::List(listing) => {
                    tree.get_mut(&current).unwrap().data_mut().files = listing
                        .iter()
                        .filter_map(|l| match l {
                            Listing::File(f) => Some((f.name.clone(), f.size)),
                            Listing::Dir(_) => None,
                        })
                        .collect();
                }
            }
        }
        DirTree { tree }
    }

    pub fn get_dir_size(&self, dir: &Node<Dir>) -> u64 {
        let own_files: u64 = dir.data().files.values().sum();
        let subdirs: u64 = dir
            .children()
            .iter()
            .map(|c| self.get_dir_size(self.tree.get(c).unwrap()))
            .sum();
        own_files + subdirs
    }
}

impl Default for Dir {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            files: HashMap::new(),
        }
    }
}
