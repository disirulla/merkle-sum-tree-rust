use halo2_experiments::chips::poseidon::spec::MySpec;
use halo2_gadgets::poseidon::primitives::{self as poseidon, ConstantLength};
use halo2_proofs::halo2curves::pasta::Fp;

pub fn poseidon(input_left: Fp, input_right: Fp) -> Fp {

    const WIDTH: usize = 5;
    const RATE: usize = 4; 
    const L: usize = 4; 

    let hash = poseidon::Hash::<_, MySpec<Fp, WIDTH, RATE>, ConstantLength<L>, WIDTH, RATE>::init().hash([input_left, input_right, Fp::from(0), Fp::from(0)]);

    hash
}