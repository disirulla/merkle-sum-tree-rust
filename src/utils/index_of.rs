use crate::{Entry, Node};

pub fn index_of(username: &str, balance: u64, nodes: &Vec<Vec<Node>>) -> Option<usize> {
    let entry = Entry::new(username.to_string(), balance).unwrap();
    let leaf = entry.compute_leaf();
    let leaf_hash = leaf.hash;

    nodes[0].iter().position(|node| node.hash == leaf_hash)
}
