use crate::Node;
use super::hash::poseidon;

pub fn create_middle_node(child_l: &Node, child_r: &Node) -> Node {
    Node {
        hash: poseidon(child_l.hash, child_l.balance, child_r.hash, child_r.balance),
        balance: child_l.balance + child_r.balance,
    }
}