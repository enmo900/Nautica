use wasm_bindgen::prelude::*;

// Fungsi ini akan dipanggil dari JavaScript
#[wasm_bindgen]
pub fn handle_link() -> String {
    "Halo".into()
}
