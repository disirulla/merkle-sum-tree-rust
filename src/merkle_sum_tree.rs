use crate::{Entry, Node, MerkleProof};
use crate::utils::{parse_csv_to_entries, build_merkle_tree_from_entries};
use std::path::Path;

pub struct MerkleSumTree {
    root: Node,
    nodes: Vec<Vec<Node>>,
    depth: usize,
    entries: Vec<Entry>,
}

impl MerkleSumTree {
    pub const MAX_DEPTH: usize = 32;

    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let entries = parse_csv_to_entries(path)?;
        let depth = (entries.len() as f64).log2().ceil() as usize;

        if depth < 1 || depth > Self::MAX_DEPTH {
            return Err("The tree depth must be between 1 and 32".into());
        }

        let mut nodes = vec![];
        let root = build_merkle_tree_from_entries(&entries, depth, &mut nodes)?;

        Ok(MerkleSumTree {
            root,
            nodes,
            depth,
            entries,
        })
    }

    // Getter methods
    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn depth(&self) -> &usize {
        &self.depth
    }

    pub fn leaves(&self) -> &[Node] {
        &self.nodes[0]
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
    

    // Implement the rest of the methods
}
