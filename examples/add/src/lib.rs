use wasm_bindgen::prelude::*;

// Allows us to use Javascript's alert directly from Rust.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// #[wasm_bindgen]
// pub fn get_thing() -> Thing {
//     Thing {
//         a: 1,
//         b: 2,
//     }
// }

#[wasm_bindgen(inspectable)]
pub struct Machine {
    /// Example field A
    pub a: u32,
    /// Example field B
    pub b: u32,
    // WARNING: These macros have to go before the doc comments, not after
    #[wasm_bindgen(skip)]
    /// Example field that does not impl the wasm bindgen traits. I'm hoping that having the 
    /// skip here will allow us to still use this type in javascript.
    pub inner: NotWasmFriendlyRuntime,
}

/// A struct that is not ready to cross the wasm boundary.
/// Notice the missing #[wasm_bindgen]. This is
/// analogue to FRAME's Runtime, Call, etc types.
pub struct NotWasmFriendlyRuntime {
    pub storage_item: u64,
}

#[wasm_bindgen]
impl Machine {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            a: 1,
            b: 2,
            inner: NotWasmFriendlyRuntime {
                storage_item: 4,
            }
        }
    }

    /// A method that can mutate this object and be called from javascript
    pub fn inc_a(&mut self) {
        self.a += 1;
    }

    pub fn get_inner_storage_item(&self) -> u64 {
        self.inner.storage_item
    }
}
