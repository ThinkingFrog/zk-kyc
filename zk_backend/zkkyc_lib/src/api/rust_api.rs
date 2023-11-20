pub use crate::core::KYCCircuit;

use halo2_proofs::{
    circuit::Value,
    pasta::{EqAffine, Fp},
    plonk::{keygen_pk, keygen_vk, Circuit, Error, ProvingKey, SingleVerifier, VerifyingKey},
    poly::commitment::Params,
    transcript::{Blake2bRead, Blake2bWrite},
};
use rand::rngs::OsRng;

pub fn create_circuit(user_age: u64, user_country: u64) -> KYCCircuit<Fp> {
    KYCCircuit::new(
        Value::known(Fp::from(user_age)),
        Value::known(Fp::from(user_country)),
    )
}

pub fn generate_params(k: u32) -> Params<EqAffine> {
    Params::new(k)
}

pub fn generate_vk(params: &Params<EqAffine>) -> Result<VerifyingKey<EqAffine>, Error> {
    let circuit = KYCCircuit::default();
    let empty_circuit = circuit.without_witnesses();

    keygen_vk(params, &empty_circuit)
}

pub fn generate_pk(
    vk: &VerifyingKey<EqAffine>,
    params: &Params<EqAffine>,
) -> Result<ProvingKey<EqAffine>, Error> {
    let circuit = KYCCircuit::default();
    let empty_circuit = circuit.without_witnesses();

    keygen_pk(params, vk.clone(), &empty_circuit)
}

pub fn create_proof(
    circuit: &KYCCircuit<Fp>,
    public_age: u64,
    public_countries: &[u64],
    pk: &ProvingKey<EqAffine>,
    params: &Params<EqAffine>,
) -> Result<Vec<u8>, Error> {
    let mut instances = vec![[Fp::from(public_age)]];
    for pc in public_countries.iter() {
        instances.push([Fp::from(*pc)]);
    }
    let instances: Vec<_> = instances.iter().map(|i| &i[..]).collect();

    let rng = OsRng;
    let mut transcript = Blake2bWrite::init(vec![]);

    halo2_proofs::plonk::create_proof(
        params,
        pk,
        &[circuit.clone()],
        &[&instances[..]],
        rng,
        &mut transcript,
    )?;

    Ok(transcript.finalize())
}

pub fn verify_proof(
    proof: Vec<u8>,
    public_age: u64,
    public_countries: &[u64],
    vk: &VerifyingKey<EqAffine>,
    params: &Params<EqAffine>,
) -> Result<(), Error> {
    let mut instances = vec![[Fp::from(public_age)]];
    for pc in public_countries.iter() {
        instances.push([Fp::from(*pc)]);
    }
    let instances: Vec<_> = instances.iter().map(|i| &i[..]).collect();

    let mut transcript = Blake2bRead::init(&proof[..]);

    let verify_res = halo2_proofs::plonk::verify_proof(
        params,
        vk,
        SingleVerifier::new(params),
        &[&instances[..]],
        &mut transcript,
    );

    verify_res
}
