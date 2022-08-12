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
pub fn fact(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * fact(n - 1)
    }
}
