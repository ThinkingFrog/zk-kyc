use crate::chips::{
    contained_chip::{ContainedChip, ContainedConfig, ContainedInstructions},
    geq_chip::{GeqChip, GeqConfig, GeqInstructions},
    number::Number,
};
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::Chip,
    circuit::{Layouter, Value},
    plonk::{Advice, Column, ConstraintSystem, Error, Instance},
};
use std::marker::PhantomData;

pub trait FieldInstructions<F: FieldExt>: Chip<F> {
    type Num;

    fn load_geq_chip_values(
        &self,
        layouter: impl Layouter<F>,
        private_value: Value<F>,
    ) -> [Result<Self::Num, Error>; 2];

    fn load_contained_chip_values(
        &self,
        layouter: impl Layouter<F>,
        private_value: Value<F>,
        public_values_amount: usize,
    ) -> Vec<Result<Self::Num, Error>>;

    fn geq(
        &self,
        layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error>;

    fn contained(
        &self,
        layouter: impl Layouter<F>,
        elem: Self::Num,
        arr: &[Self::Num],
    ) -> Result<Self::Num, Error>;
}

#[derive(Clone, Debug)]
pub struct FieldConfig {
    pub geq_config: GeqConfig,
    pub contained_config: ContainedConfig,
}

pub struct FieldChip<F: FieldExt> {
    config: FieldConfig,
    _marker: PhantomData<F>,
    geq_chip: GeqChip<F>,
    contained_chip: ContainedChip<F>,
}

impl<F: FieldExt> Chip<F> for FieldChip<F> {
    type Config = FieldConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: FieldExt> FieldChip<F> {
    pub fn construct(config: <Self as Chip<F>>::Config) -> Self {
        let geq_chip = GeqChip::construct(config.geq_config.clone());
        let contained_chip = ContainedChip::construct(config.contained_config.clone());

        Self {
            config,
            _marker: PhantomData,
            geq_chip,
            contained_chip,
        }
    }

    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: &[Column<Advice>],
        instance: &[Column<Instance>],
    ) -> <Self as Chip<F>>::Config {
        let geq_config = GeqChip::configure(meta, [advice[0], advice[1]], instance[0]);
        let contained_config = ContainedChip::configure(meta, &advice[2..], &instance[1..]);

        FieldConfig {
            geq_config,
            contained_config,
        }
    }
}

impl<F: FieldExt> FieldInstructions<F> for FieldChip<F> {
    type Num = Number<F>;

    fn load_geq_chip_values(
        &self,
        mut layouter: impl Layouter<F>,
        private_value: Value<F>,
    ) -> [Result<Self::Num, Error>; 2] {
        let private_res = self.geq_chip.load_private(
            layouter.namespace(|| "load geq chip private value"),
            private_value,
        );
        let public_res = self
            .geq_chip
            .load_public(layouter.namespace(|| "load geq chip public value"));

        [private_res, public_res]
    }

    fn load_contained_chip_values(
        &self,
        mut layouter: impl Layouter<F>,
        private_value: Value<F>,
        public_values_amount: usize,
    ) -> Vec<Result<Self::Num, Error>> {
        let mut results = vec![];

        results.push(self.contained_chip.load_private(
            layouter.namespace(|| "load contained chip private value"),
            private_value,
        ));

        for idx in 0..public_values_amount {
            results.push(self.contained_chip.load_public(
                layouter.namespace(|| format!("load contained chip public value {}", idx + 1)),
                idx,
            ));
        }

        results
    }

    fn geq(
        &self,
        layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error> {
        let config = self.config();

        let geq_chip = GeqChip::<F>::construct(config.geq_config.clone());

        geq_chip.geq(layouter, a, b)
    }

    fn contained(
        &self,
        layouter: impl Layouter<F>,
        elem: Self::Num,
        arr: &[Self::Num],
    ) -> Result<Self::Num, Error> {
        let config = self.config();

        let contained_chip = ContainedChip::<F>::construct(config.contained_config.clone());

        contained_chip.contained(layouter, elem, arr)
    }
}
