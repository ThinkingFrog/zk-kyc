use crate::chips::number::Number;
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Chip, Layouter, Region, Value},
    plonk::{Advice, Column, ConstraintSystem, Error, Expression, Instance, Selector},
    poly::Rotation,
};
use std::marker::PhantomData;

pub trait GeqInstructions<F: FieldExt>: Chip<F> {
    type Num;

    fn load_private(&self, layouter: impl Layouter<F>, value: Value<F>)
        -> Result<Self::Num, Error>;

    fn load_public(&self, layouter: impl Layouter<F>) -> Result<Self::Num, Error>;

    // Returns a boolean in form of {0, 1} integer indicating whether a >= b, where 1 is True and 0 is False
    fn geq(
        &self,
        layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error>;
}

#[derive(Clone, Debug)]
pub struct GeqConfig {
    advice: [Column<Advice>; 2],
    instance: Column<Instance>,
    s_geq: Selector,
}

pub struct GeqChip<F: FieldExt> {
    config: GeqConfig,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> Chip<F> for GeqChip<F> {
    type Config = GeqConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: FieldExt> GeqChip<F> {
    pub fn construct(config: <Self as Chip<F>>::Config) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: [Column<Advice>; 2],
        instance: Column<Instance>,
    ) -> <Self as Chip<F>>::Config {
        meta.enable_equality(instance);
        for col in advice.iter() {
            meta.enable_equality(*col);
        }
        let s_geq = meta.selector();

        meta.create_gate("geq", |meta| {
            let out = meta.query_advice(advice[0], Rotation::next());
            let s_geq = meta.query_selector(s_geq);

            vec![s_geq * (out - Expression::Constant(F::one()))]
        });

        GeqConfig {
            advice,
            instance,
            s_geq,
        }
    }
}

impl<F: FieldExt> GeqInstructions<F> for GeqChip<F> {
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

    fn load_public(&self, mut layouter: impl Layouter<F>) -> Result<Self::Num, Error> {
        let config = self.config();

        layouter.assign_region(
            || "load public",
            |mut region| {
                region
                    .assign_advice_from_instance(
                        || "public input",
                        config.instance,
                        0,
                        config.advice[1],
                        1,
                    )
                    .map(Number)
            },
        )
    }

    fn geq(
        &self,
        mut layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error> {
        let config = self.config();

        layouter.assign_region(
            || "geq",
            |mut region: Region<'_, F>| {
                config.s_geq.enable(&mut region, 0)?;

                a.0.copy_advice(|| "lhs", &mut region, config.advice[0], 0)?;
                b.0.copy_advice(|| "rhs", &mut region, config.advice[1], 0)?;

                let value = a.0.value().zip(b.0.value()).map(|(x, y)| x >= y);
                let value = value.map(|v| if v { F::one() } else { F::zero() });

                region
                    .assign_advice(|| "lhs >= rhs", config.advice[0], 1, || value)
                    .map(Number)
            },
        )
    }
}
