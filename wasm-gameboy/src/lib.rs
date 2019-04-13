mod utils;

use gameboy::gameboy::GameBoy;
use gameboy::gpu::{GB_HSIZE, GB_VSIZE};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct WasmGameboy {
    gb: GameBoy
}

#[wasm_bindgen]
impl WasmGameboy {
    pub fn new(rom_js: &JsValue) -> WasmGameboy {
        let rom_contents: Vec<u8> = rom_js.into_serde().unwrap();

        WasmGameboy {
            gb: GameBoy::new(rom_contents)
        }
    }

    pub fn cycle(&mut self) -> bool {
        self.gb.cycle(false, None)
    }

    pub fn buffer(&self) -> *const u8
    {
        self.gb.buffer()
    }

    pub fn buffer_size(&self) -> usize {
        self.gb.buffer_vec().len()
    }

    pub fn buffer_width() -> usize {
        GB_HSIZE
    }
    pub fn buffer_height() -> usize {
        GB_VSIZE
    }
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life 2!");
}
