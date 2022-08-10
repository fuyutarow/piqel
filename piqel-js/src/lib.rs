mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Pool {
    body: String,
}

#[wasm_bindgen]
impl Pool {
    pub fn new(body: &str) -> Self {
        Self {
            body: body.to_string(),
        }
    }

    pub fn get_body(&self) -> String {
        self.body.to_owned()
    }

    pub fn query(&self, query: &str) -> Option<String> {
        piqel::engine::evaluate(query, &self.body, "json", "json").ok()
    }
}

#[wasm_bindgen]
pub fn evaluate(sql: &str, input: &str, from: &str, to: &str) -> Option<String> {
    piqel::engine::evaluate(sql, input, from, to).ok()
}

#[wasm_bindgen]
pub fn addi(a: i32, b: i32) -> i32 {
    a + b
}
