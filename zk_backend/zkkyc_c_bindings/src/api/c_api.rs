use std::ffi::{c_uchar, c_ulong};
use zkkyc_lib::{create_circuit, generate_params, generate_pk, generate_vk, K};

fn c_array_to_rust_vec<T: Copy>(arr: *const T, arr_len: c_ulong) -> Vec<T> {
    let mut v = vec![];

    for idx in 0..arr_len {
        v.push(unsafe { *arr.offset(idx as isize) });
    }

    v
}

#[no_mangle]
pub extern "C" fn create_proof(
    user_age: c_ulong,
    user_country: c_ulong,
    public_age: c_ulong,
    public_countries: *const c_ulong,
    public_countries_len: c_ulong,
    proof_len: *mut c_ulong,
) -> *const c_uchar {
    let handle_error = || {
        unsafe { *proof_len = 0 };
        std::ptr::null()
    };

    let public_countries = c_array_to_rust_vec(public_countries, public_countries_len);

    let circuit = create_circuit(user_age, user_country);

    let params = generate_params(K);

    let vk = match generate_vk(&params) {
        Ok(val) => val,
        Err(_) => return handle_error(),
    };

    let pk = match generate_pk(&vk, &params) {
        Ok(val) => val,
        Err(_) => return handle_error(),
    };

    let mut proof_vec =
        match zkkyc_lib::create_proof(&circuit, public_age, &public_countries, &pk, &params) {
            Ok(val) => val,
            Err(_) => return handle_error(),
        };
    proof_vec.shrink_to_fit();

    unsafe { *proof_len = proof_vec.len() as u64 };
    let proof_ptr = proof_vec.as_mut_ptr();
    std::mem::forget(proof_vec);

    proof_ptr
}

#[no_mangle]
pub extern "C" fn verify_proof(
    public_age: c_ulong,
    public_countries: *const c_ulong,
    public_countries_len: c_ulong,
    proof: *const c_uchar,
    proof_len: c_ulong,
) -> c_uchar {
    let public_countries = c_array_to_rust_vec(public_countries, public_countries_len);

    let proof = c_array_to_rust_vec(proof, proof_len);

    let params = generate_params(K);
    let vk = match generate_vk(&params) {
        Ok(val) => val,
        Err(_) => return 0,
    };

    let verify_res = zkkyc_lib::verify_proof(proof, public_age, &public_countries, &vk, &params);

    verify_res.is_ok() as u8
}
