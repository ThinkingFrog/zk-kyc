use super::c_api::*;

#[test]
fn lib_api_simple() {
    let user_age = 21;
    let user_country = 7;

    let public_age = 18;
    let public_countries = Box::new([1, 7, 12]);

    let mut proof_len = Box::new(0);

    let proof = Box::new(create_proof(
        user_age,
        user_country,
        public_age,
        public_countries.as_ptr(),
        public_countries.len() as u64,
        &mut *proof_len,
    ));
    let verify_res = verify_proof(
        public_age,
        public_countries.as_ptr(),
        public_countries.len() as u64,
        *proof,
        *proof_len,
    );

    assert!(verify_res == 1);
}

#[test]
fn lib_api_simple_false_proof() {
    let user_age = 11;
    let user_country = 7;

    let public_age = 18;
    let public_countries = Box::new([1, 7, 12]);

    let mut proof_len = Box::new(0);

    let proof = Box::new(create_proof(
        user_age,
        user_country,
        public_age,
        public_countries.as_ptr(),
        public_countries.len() as u64,
        &mut *proof_len,
    ));
    let verify_res = verify_proof(
        public_age,
        public_countries.as_ptr(),
        public_countries.len() as u64,
        *proof,
        *proof_len,
    );

    assert!(verify_res == 0);
}
