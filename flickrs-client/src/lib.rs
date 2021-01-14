mod utils;

use wasm_bindgen::prelude::*;

use cife_rs::abe::dippe::*;

pub mod hybrid;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn end_to_end_conjunction() {
    let mut rng = rand::thread_rng();
    let dippe = Dippe::randomized(&mut rng, 2);

    let (alice_pub, _alice_priv) = dippe.generate_key_pair(&mut rng);
    let (_bob_pub, _bob_priv) = dippe.generate_key_pair(&mut rng);

    alert("Alice public key calculated");
}

#[wasm_bindgen]
pub fn get_image_title() -> String {
    "Hello from Rust".into()
}
