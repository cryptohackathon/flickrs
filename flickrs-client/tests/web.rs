//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use cife_rs::abe::dippe::*;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn end_to_end_conjunction() {
    let mut rng = rand::thread_rng();
    let dippe = Dippe::new(&mut rng, 2);

    let (_alice_pub, _alice_priv) = dippe.generate_key_pair(&mut rng);
    let (_bob_pub, _bob_priv) = dippe.generate_key_pair(&mut rng);
}
