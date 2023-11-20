use crate::chips::{FieldChip, FieldConfig, FieldInstructions};

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Circuit, ConstraintSystem, Error},
};

// Constant value that is set deterministically based on circuit size and parameters
// DON'T MODIFY UNLESS THE CIRCUIT IS CHANGED AND THEN SOMETHING CRASHES
pub const K: u32 = 4;

#[derive(Default, Clone)]
pub struct KYCCircuit<F: FieldExt> {
    user_age: Value<F>,
    user_country: Value<F>,
}

impl<F: FieldExt> KYCCircuit<F> {
    const PRIVATE_COUNTRIES_AMOUNT: usize = 1;
    const PRIVATE_AGE_AMOUNT: usize = 1;
    const PUBLIC_COUNTRIES_AMOUNT: usize = 3;
    const PUBLIC_AGE_AMOUNT: usize = 1;

    pub fn new(user_age: Value<F>, user_country: Value<F>) -> Self {
        Self {
            user_age,
            user_country,
        }
    }
}

impl<F: FieldExt> Circuit<F> for KYCCircuit<F> {
    type Config = FieldConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            user_age: Value::unknown(),
            user_country: Value::unknown(),
        }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let instance_columns = Self::PUBLIC_AGE_AMOUNT + Self::PUBLIC_COUNTRIES_AMOUNT;
        let advice_columns =
            Self::PRIVATE_AGE_AMOUNT + Self::PRIVATE_COUNTRIES_AMOUNT + instance_columns;

        let advice = (0..advice_columns)
            .map(|_| meta.advice_column())
            .collect::<Vec<_>>();

        let instance = (0..instance_columns)
            .map(|_| meta.instance_column())
            .collect::<Vec<_>>();

        FieldChip::configure(meta, &advice, &instance)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let field_chip = FieldChip::<F>::construct(config);

        // Load age values into the circuit
        let mut geq_chip_values: Vec<<FieldChip<F> as FieldInstructions<F>>::Num> = vec![];
        for val in field_chip
            .load_geq_chip_values(layouter.namespace(|| "load geq chip values"), self.user_age)
        {
            geq_chip_values.push(val?);
        }
        let user_age = geq_chip_values[0].clone();
        let allowed_age = geq_chip_values[1].clone();

        // Load country values into the circuit
        let mut contained_chip_values: Vec<<FieldChip<F> as FieldInstructions<F>>::Num> = vec![];
        for val in field_chip.load_contained_chip_values(
            layouter.namespace(|| "load contained chip values"),
            self.user_country,
            Self::PUBLIC_COUNTRIES_AMOUNT,
        ) {
            contained_chip_values.push(val?);
        }
        let user_country = contained_chip_values[0].clone();
        let allowed_countries = &contained_chip_values[1..=Self::PUBLIC_COUNTRIES_AMOUNT];

        // Check age and country
        field_chip.geq(
            layouter.namespace(|| "age >= allowed_age"),
            user_age,
            allowed_age,
        )?;
        field_chip.contained(
            layouter.namespace(|| "country in allowed_countries"),
            user_country,
            allowed_countries,
        )?;

        Ok(())
    }
}
