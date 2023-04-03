// proof_verification.rs

use crate::{Node, Entry, MerkleProof};
use crate::utils::poseidon;

fn create_middle_node(child_l: Node, child_r: Node) -> Node {
    Node {
        hash: poseidon(child_l.hash, child_r.hash),
    }
}

pub fn verify_proof(proof: &MerkleProof) -> bool {
    let mut node = proof.entry.compute_leaf();

    for i in 0..proof.sibling_hashes.len() {
        let sibling_node = Node {
            hash: proof.sibling_hashes[i],
        };

        if proof.path_indices[i] == 0.into() {
            node = create_middle_node(node, sibling_node);
        } else {
            node = create_middle_node(sibling_node, node);
        }
    }

    proof.root_hash == node.hash
}
