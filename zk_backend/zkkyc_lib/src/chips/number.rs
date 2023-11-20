use halo2_proofs::{arithmetic::FieldExt, circuit::AssignedCell};

#[derive(Clone, Debug)]
pub struct Number<F: FieldExt>(pub AssignedCell<F, F>);
