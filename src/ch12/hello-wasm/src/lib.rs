use wasm_bindgen::prelude::*; // Rust與JavaScript溝通的橋樑

// JavaScript函數，Rust可呼叫
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

// Rust函數，JavaScript可呼叫
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
