use crate::chips::number::Number;
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Chip, Layouter, Region, Value},
    plonk::{Advice, Column, ConstraintSystem, Error, Expression, Instance, Selector},
    poly::Rotation,
};
use std::marker::PhantomData;

pub trait ContainedInstructions<F: FieldExt>: Chip<F> {
    type Num;

    fn load_private(&self, layouter: impl Layouter<F>, value: Value<F>)
        -> Result<Self::Num, Error>;

    fn load_public(&self, layouter: impl Layouter<F>, column: usize) -> Result<Self::Num, Error>;

    fn contained(
        &self,
        layouter: impl Layouter<F>,
        elem: Self::Num,
        arr: &[Self::Num],
    ) -> Result<Self::Num, Error>;
}

#[derive(Clone, Debug)]
pub struct ContainedConfig {
    advice: Vec<Column<Advice>>,
    instance: Vec<Column<Instance>>,
    s_contained: Selector,
}

pub struct ContainedChip<F: FieldExt> {
    config: ContainedConfig,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> Chip<F> for ContainedChip<F> {
    type Config = ContainedConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: FieldExt> ContainedChip<F> {
    pub fn construct(config: <Self as Chip<F>>::Config) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: &[Column<Advice>],
        instance: &[Column<Instance>],
    ) -> <Self as Chip<F>>::Config {
        for col in instance.iter() {
            meta.enable_equality(*col);
        }
        for col in advice.iter() {
            meta.enable_equality(*col);
        }

        let s_contained = meta.selector();

        meta.create_gate("contained", |meta| {
            let s_contained = meta.query_selector(s_contained);
            let out = meta.query_advice(advice[0], Rotation::next());

            vec![s_contained * (out - Expression::Constant(F::one()))]
        });

        ContainedConfig {
            advice: advice.to_vec(),
            instance: instance.to_vec(),
            s_contained,
        }
    }
}

impl<F: FieldExt> ContainedInstructions<F> for ContainedChip<F> {
    type Num = Number<F>;

    fn load_private(
        &self,
        mut layouter: impl Layouter<F>,
        value: Value<F>,
    ) -> Result<Self::Num, Error> {
        let config = self.config();

        layouter.assign_region(
            || "load private",
            |mut region| {
                region
                    .assign_advice(|| "private input", config.advice[0], 0, || value)
                    .map(Number)
            },
        )
    }

    fn load_public(
        &self,
        mut layouter: impl Layouter<F>,
        column: usize,
    ) -> Result<Self::Num, Error> {
        let config = self.config();

        layouter.assign_region(
            || "load public",
            |mut region| {
                region
                    .assign_advice_from_instance(
                        || "public input",
                        config.instance[column],
                        0,
                        config.advice[column + 1],
                        0,
                    )
                    .map(Number)
            },
        )
    }

    fn contained(
        &self,
        mut layouter: impl Layouter<F>,
        elem: Self::Num,
        arr: &[Self::Num],
    ) -> Result<Self::Num, Error> {
        let config = self.config();
        let mut contained = Value::known(F::zero());

        layouter.assign_region(
            || "contained",
            |mut region: Region<'_, F>| {
                config.s_contained.enable(&mut region, 0)?;

                elem.0
                    .copy_advice(|| "desired element", &mut region, self.config.advice[0], 0)?;
                for (i, el) in arr.iter().enumerate() {
                    el.0.copy_advice(
                        || format!("array element {}", i),
                        &mut region,
                        self.config.advice[i + 1],
                        0,
                    )?;
                }

                for el in arr.iter() {
                    elem.0.value().zip(el.0.value()).map(|(x, y)| {
                        if x == y {
                            contained = Value::known(F::one());
                        }
                    });
                }

                region
                    .assign_advice(
                        || "desired element in array",
                        config.advice[0],
                        1,
                        || contained,
                    )
                    .map(Number)
            },
        )
    }
}
