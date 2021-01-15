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

    let (_alice_pub, _alice_priv) = dippe.generate_key_pair(&mut rng);
    let (_bob_pub, _bob_priv) = dippe.generate_key_pair(&mut rng);

    alert("Alice public key calculated");
}

#[wasm_bindgen]
pub fn get_image_title() -> String {
    "Hello from Rust".into()
}

#[wasm_bindgen]
pub fn seal(
    server_key: &JsValue,
    bytes: &JsValue,
    selected_attrs: &JsValue,
    attributes: &JsValue,
) -> JsValue {
    let dippe = Dippe::new(b"flickrs", 2);

    let authority: PublicKey = server_key.into_serde().unwrap();
    let bytes: String = bytes.into_serde().unwrap();
    let selected_attrs: Vec<usize> = selected_attrs.into_serde().unwrap();
    let attributes: usize = attributes.into_serde().unwrap();

    let mut rng = rand::thread_rng();

    let sealed = hybrid::seal(
        &mut rng,
        &dippe,
        &authority,
        &bytes.as_bytes(),
        &selected_attrs,
        attributes,
    );

    JsValue::from_serde(&sealed).unwrap()
}

#[wasm_bindgen]
pub fn open(
    upk: &JsValue,
    av: &[usize],
    gid: &JsValue,
    bytes: &[u8],
    attributes: usize,
) -> JsValue {
    let dippe = Dippe::new(b"flickrs", 2);

    let policy = dippe.create_attribute_vector(attributes, av);

    let upk: UserPrivateKey = upk.into_serde().unwrap();
    let gid: String = gid.into_serde().unwrap();

    let open = hybrid::open(&dippe, attributes, &policy, &upk, &gid.as_bytes(), &bytes);

    JsValue::from_serde(&open).unwrap()
}
