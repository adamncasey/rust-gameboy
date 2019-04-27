mod utils;

use gameboy::gameboy::GameBoy;
use gameboy::gpu::{GB_HSIZE, GB_VSIZE};
use gameboy::instruction::Instruction;

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
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct WasmGameboy {
    gb: Option<GameBoy>,
    rom_buffer: Vec<u8>,
    debug: bool,
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
    pub lcdc: u8
}

#[wasm_bindgen]
impl WasmGameboy {
    pub fn new(rom_size: usize) -> WasmGameboy {
        utils::set_panic_hook();
        WasmGameboy {
            gb: None,
            rom_buffer: vec![0; rom_size],
            debug: false,
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
                if self.debug {
                    consolelog!(
                        "State after {} total steps {} | Gpu PWR {} mode: {:?} elapsed {} ly {} stat {:X} lcdc {:X}",
                        gb.steps,
                        gb.cpu.print_state(),
                        gb.gpu.debug_lcd_pwr,
                        gb.gpu.mode,
                        gb.gpu.mode_elapsed,
                        gb.gpu.line,
                        gb.mem.get(0xFF41),
                        gb.mem.get(0xFF40)
                    );

                    let instrs = Instruction::disassemble(&gb.mem, gb.cpu.pc, 5);

                    consolelog!("--- Disassembly at PC {:4X} {:?}", gb.cpu.pc, instrs);
                }

                if gb.cycle(false, None) {
                    return true;
                }
            }
        } else {
            consolelog!("Gameboy null");
        }
        false
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
        }
        else {
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

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life 2!");
}
