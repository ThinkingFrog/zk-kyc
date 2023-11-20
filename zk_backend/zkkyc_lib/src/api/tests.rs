use super::rust_api::*;
use crate::core::K;

#[test]
fn lib_api_simple() {
    let user_age = 21;
    let user_country = 7;
    let circuit = create_circuit(user_age, user_country);

    let public_age = 18;
    let public_countries = [1, 7, 12];

    let params = generate_params(K);

    let vk = generate_vk(&params).unwrap();
    let pk = generate_pk(&vk, &params).unwrap();

    let proof = create_proof(&circuit, public_age, &public_countries, &pk, &params).unwrap();
    let verify_res = verify_proof(proof, public_age, &public_countries, &vk, &params);

    assert!(verify_res.is_ok());
}

#[test]
fn lib_api_simple_false_proof() {
    let user_age = 11;
    let user_country = 7;
    let circuit = create_circuit(user_age, user_country);

    let public_age = 18;
    let public_countries = [1, 7, 12];

    let params = generate_params(K);

    let vk = generate_vk(&params).unwrap();
    let pk = generate_pk(&vk, &params).unwrap();

    let proof = create_proof(&circuit, public_age, &public_countries, &pk, &params).unwrap();
    let verify_res = verify_proof(proof, public_age, &public_countries, &vk, &params);

    assert!(verify_res.is_err());
}

#[test]
fn lib_api_with_separate_keygen() {
    // Must be common knowledge to prover and verifier
    let public_age = 18;
    let public_countries = [1, 7, 12];

    // To share prove easier
    let proof;

    // Proving phase
    {
        let user_age = 21;
        let user_country = 7;
        let circuit = create_circuit(user_age, user_country);

        let params = generate_params(K);
        let vk = generate_vk(&params).unwrap();
        let pk = generate_pk(&vk, &params).unwrap();

        proof = create_proof(&circuit, public_age, &public_countries, &pk, &params).unwrap();
    }

    // Verification phase
    {
        let params = generate_params(K);
        let vk = generate_vk(&params).unwrap();

        let verify_res = verify_proof(proof, public_age, &public_countries, &vk, &params);

        assert!(verify_res.is_ok());
    }
}

#[test]
fn lib_api_with_separate_keygen_false_proof() {
    // Must be common knowledge to prover and verifier
    let public_age = 18;
    let public_countries = [1, 7, 12];

    // To share prove easier
    let proof;

    // Proving phase
    {
        let user_age = 11;
        let user_country = 7;
        let circuit = create_circuit(user_age, user_country);

        let params = generate_params(K);
        let vk = generate_vk(&params).unwrap();
        let pk = generate_pk(&vk, &params).unwrap();

        proof = create_proof(&circuit, public_age, &public_countries, &pk, &params).unwrap();
    }

    // Verification phase
    {
        let params = generate_params(K);
        let vk = generate_vk(&params).unwrap();

        let verify_res = verify_proof(proof, public_age, &public_countries, &vk, &params);

        assert!(verify_res.is_err());
    }
}
