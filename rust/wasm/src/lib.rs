use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn transform(input: &str) -> String {
    iloader_core::transform(input)
}
