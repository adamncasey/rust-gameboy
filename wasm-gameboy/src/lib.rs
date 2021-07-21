mod utils;

use std::mem;

use gameboy::disassemble::{disassemble, Disassembly};
use gameboy::gameboy::GameBoy;
use gameboy::gpu::{GB_HSIZE, GB_VSIZE};

use wasm_bindgen::prelude::*;

use web_sys::console;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! consolelog {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct WasmGameboy {
    gb: Option<GameBoy>,
    rom_buffer: Vec<u8>,
    debug: bool,

    last_disassembly: Vec<Disassembly>,
}

#[wasm_bindgen]
pub struct GameboyDebugInfo {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub lcd_pwr: bool,
    pub stat: u8,
    pub ly: u8,
    pub lcdc: u8,
}

#[wasm_bindgen]
impl WasmGameboy {
    pub fn new(rom_size: usize) -> WasmGameboy {
        utils::set_panic_hook();
        WasmGameboy {
            gb: None,
            rom_buffer: vec![0; rom_size],
            debug: false,
            last_disassembly: Vec::new(),
        }
    }

    pub fn debug(&mut self, enable: bool) {
        self.debug = enable;
    }

    pub fn start(&mut self) {
        self.gb = Some(GameBoy::new(self.rom_buffer.clone()))
    }

    pub fn cycle_until_vsync(&mut self) -> bool {
        if let Some(gb) = self.gb.as_mut() {
            for _ in 0..100_000 {
                if gb.cycle(false, None) {
                    return true;
                }
            }
        } else {
            consolelog!("Gameboy null");
        }
        false
    }

    pub fn cycle(&mut self) -> bool {
        if let Some(gb) = self.gb.as_mut() {
            if gb.cycle(false, None) {
                return true;
            }
        } else {
            consolelog!("Gameboy null");
        }
        false
    }

    /// Populates an internal buffer with the decoding of the memory address
    /// range [start..end]. Returns the decoded instruction info
    pub fn disassemble(&mut self, start: u16, end: u16) -> JsValue {
        if let Some(gb) = self.gb.as_mut() {
            self.last_disassembly = disassemble(start, end, &gb.mem);

            consolelog!("{:?}", self.last_disassembly);

            JsValue::from_serde(&self.last_disassembly).unwrap()
        } else {
            panic!("Gameboy null");
        }
    }

    /// Returns a pointer to the internal buffer storing the results
    /// from the last disassembly. Pointer is valid until the next call to
    /// `disassemble`
    pub fn last_disassembly(&self) -> *const Disassembly {
        self.last_disassembly.as_ptr()
    }

    pub fn disassembly_row_size(&self) -> usize {
        mem::size_of::<Disassembly>()
    }

    pub fn debug_info(&self) -> GameboyDebugInfo {
        if let Some(gb) = self.gb.as_ref() {
            GameboyDebugInfo {
                pc: gb.cpu.pc,
                sp: gb.cpu.sp,
                a: gb.cpu.a,
                b: gb.cpu.b,
                c: gb.cpu.c,
                d: gb.cpu.d,
                e: gb.cpu.e,
                f: gb.cpu.f,
                h: gb.cpu.h,
                l: gb.cpu.l,
                lcd_pwr: gb.gpu.debug_lcd_pwr,
                ly: gb.gpu.line,
                stat: gb.mem.get(0xFF41),
                lcdc: gb.mem.get(0xFF40),
            }
        } else {
            panic!("gb is null");
        }
    }

    pub fn rom_buffer(&mut self) -> *mut u8 {
        self.rom_buffer.as_mut_ptr()
    }

    pub fn screen_buffer(&self) -> *const u8 {
        self.gb.as_ref().unwrap().buffer()
    }

    pub fn screen_size(&self) -> usize {
        self.gb.as_ref().map(|gb| gb.buffer_vec().len()).unwrap()
    }

    pub fn screen_width(&self) -> usize {
        GB_HSIZE
    }
    pub fn screen_height(&self) -> usize {
        GB_VSIZE
    }
}
