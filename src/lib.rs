mod entry;
mod merkle_sum_tree;
mod utils;
use halo2_proofs::halo2curves::pasta::Fp;

// Define the Node and MerkleProof structs here
pub struct MerkleProof {
    root_hash: Fp,
    entry: Entry,
    sibling_hashes: Vec<Fp>,
    path_indices: Vec<Fp>,
}

#[derive(Default, Clone)]
pub struct Node {
    hash: Fp,
}

pub use entry::Entry;
pub use merkle_sum_tree::MerkleSumTree;
pub use utils::{big_intify_username, poseidon};