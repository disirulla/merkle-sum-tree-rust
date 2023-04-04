use halo2_experiments::chips::poseidon::spec::MySpec;
use halo2_gadgets::poseidon::primitives::{self as poseidon, ConstantLength};
use halo2_proofs::halo2curves::pasta::Fp;

pub fn poseidon(l1: Fp, l2: Fp, r1: Fp, r2: Fp) -> Fp {

    const WIDTH: usize = 5;
    const RATE: usize = 4; 
    const L: usize = 4; 

    let hash = poseidon::Hash::<_, MySpec<Fp, WIDTH, RATE>, ConstantLength<L>, WIDTH, RATE>::init().hash([l1, l2, r1, r2]);

    hash
}