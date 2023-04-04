mod entry;
mod merkle_sum_tree;
mod utils;
use halo2_proofs::halo2curves::pasta::Fp;

#[derive(Default, Clone)]
pub struct MerkleProof {
    root_hash: Fp,
    entry: Entry,
    sibling_hashes: Vec<Fp>,
    sibling_sums: Vec<Fp>,
    path_indices: Vec<Fp>,
}

#[derive(Default, Clone)]
pub struct Node {
    hash: Fp,
    balance: Fp,
}

pub use entry::Entry;
pub use merkle_sum_tree::MerkleSumTree;
pub use utils::big_intify_username;

#[cfg(test)]
mod tests {

    use super::{MerkleSumTree};


    #[test]
    fn init_mst() {

        // create new merkle tree
        let merkle_tree = MerkleSumTree::new("src/csv/entry_16.csv").unwrap();

        // get root 
        let root = merkle_tree.root();

        // expect root hash to be different than 0
        assert!(root.hash != 0.into());

        // expect root balance to be 554214
        // println root.balance
        println!("root.balance: {:?}", root.balance);

        assert!(root.balance == 556862.into());

        // get proof 
        let proof = merkle_tree.generate_proof(0).unwrap();

        // verify proof
        assert!(merkle_tree.verify_proof(&proof));
    }

}
