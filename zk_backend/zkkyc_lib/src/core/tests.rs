use super::circuit::KYCCircuit;

use halo2_proofs::{
    circuit::Value,
    dev::{MockProver, VerifyFailure},
    pasta::Fp,
};

const K: u32 = 4;

fn run_mock_prover(
    circuit: KYCCircuit<Fp>,
    public_inputs: Vec<Vec<Fp>>,
) -> Result<(), Vec<VerifyFailure>> {
    let prover = MockProver::run(K, &circuit, public_inputs).unwrap();
    prover.verify()
}

#[test]
fn private_all_fit_public() {
    let public_age = 18;
    let public_countries = [1, 7, 12];

    let mut public_inputs = vec![vec![Fp::from(public_age)]];
    for pc in public_countries.iter() {
        public_inputs.push(vec![Fp::from(*pc)]);
    }

    let user_age = Value::known(Fp::from(21));
    let user_country = Value::known(Fp::from(7));

    let circuit = KYCCircuit::new(user_age, user_country);

    let verify_res = run_mock_prover(circuit, public_inputs);

    println!("{verify_res:?}");
    assert!(verify_res.is_ok());
}

#[test]
fn private_all_dont_fit_public() {
    let public_age = 18;
    let public_countries = [1, 7, 12];

    let mut public_inputs = vec![vec![Fp::from(public_age)]];
    for pc in public_countries.iter() {
        public_inputs.push(vec![Fp::from(*pc)]);
    }

    let user_age = Value::known(Fp::from(14));
    let user_country = Value::known(Fp::from(2));

    let circuit = KYCCircuit::new(user_age, user_country);

    let verify_res = run_mock_prover(circuit, public_inputs);

    assert!(verify_res.is_err());
}

#[test]
fn public_all_dont_fit_private() {
    let public_age = 25;
    let public_countries = [1, 7, 12];

    let mut public_inputs = vec![vec![Fp::from(public_age)]];
    for pc in public_countries.iter() {
        public_inputs.push(vec![Fp::from(*pc)]);
    }

    let user_age = Value::known(Fp::from(21));
    let user_country = Value::known(Fp::from(7));

    let circuit = KYCCircuit::new(user_age, user_country);

    let verify_res = run_mock_prover(circuit, public_inputs);

    assert!(verify_res.is_err());
}

#[test]
fn public_age_dont_fit_private() {
    let public_age = 25;
    let public_countries = [1, 7, 12];

    let mut public_inputs = vec![vec![Fp::from(public_age)]];
    for pc in public_countries.iter() {
        public_inputs.push(vec![Fp::from(*pc)]);
    }

    let user_age = Value::known(Fp::from(21));
    let user_country = Value::known(Fp::from(7));

    let circuit = KYCCircuit::new(user_age, user_country);

    let verify_res = run_mock_prover(circuit, public_inputs);

    assert!(verify_res.is_err());
}

#[test]
fn public_country_dont_fit_private() {
    let public_age = 18;
    let public_countries = [1, 17, 12];

    let mut public_inputs = vec![vec![Fp::from(public_age)]];
    for pc in public_countries.iter() {
        public_inputs.push(vec![Fp::from(*pc)]);
    }

    let user_age = Value::known(Fp::from(21));
    let user_country = Value::known(Fp::from(7));

    let circuit = KYCCircuit::new(user_age, user_country);

    let verify_res = run_mock_prover(circuit, public_inputs);

    assert!(verify_res.is_err());
}
