pub use crate::cpu::Cpu;
use crate::gpu::{Gpu, GpuDebugTrace};
use crate::input::Input;
use crate::interrupt;
use crate::memory::Memory;
use crate::rom::Cartridge;

pub struct GameBoy {
    title: String,
    pub cpu: Cpu,
    pub gpu: Gpu,
    pub mem: Memory,

    pub steps: u64,
}

impl GameBoy {
    pub fn new(rom_contents: Vec<u8>) -> GameBoy {
        let cartridge: Cartridge= Cartridge::load_rom(rom_contents);

        if cartridge.rom_type != 0 {
            println!("ROM type unsupported {}", cartridge.rom_type);
        }
        if cartridge.rom_size != 0 {
            println!("ROM Size unsupported {}", cartridge.rom_size);
        }
        if cartridge.ram_size != 0 {
            println!("RAM Size unsupported {}", cartridge.ram_size);
        }

        GameBoy {
            title: cartridge.game_title.clone(),
            cpu: Cpu::new(),
            gpu: Gpu::new(),
            mem: Memory::new(cartridge),
            steps: 0,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn cycle(&mut self, debug: bool, debug_pc_panic: Option<u16>) -> bool {
        if debug {
            println!(
                "State after {} total steps {} | Gpu PWR {} mode: {:?} elapsed {} ly {} stat {:X} lcdc {:X}",
                self.steps,
                self.cpu.print_state(),
                self.gpu.debug_lcd_pwr,
                self.gpu.mode,
                self.gpu.mode_elapsed,
                self.gpu.line,
                self.mem.get(0xFF41),
                self.mem.get(0xFF40),
            );
            println!(
                "Cpu Flags: Z{} N{} H{} C{}",
                self.cpu.z_flag(),
                self.cpu.n_flag(),
                self.cpu.h_flag(),
                self.cpu.c_flag()
            );
        }

        let cycles: u8 = self.cpu.cycle(&mut self.mem, debug);

        if let Some(pc) = debug_pc_panic {
            if self.cpu.pc == pc {
                panic!("Reached panic-at-pc");
            }
        }

        self.gpu.cycle(&mut self.mem, cycles);

        self.mem.tick_timer(cycles);

        let mut redraw_screen = false;

        let int = interrupt::fetch_interrupt(&mut self.mem);
        if let Some(active) = int {
            if self.cpu.interrupt(&mut self.mem, active) {
                redraw_screen = true;
            }
        }

        self.steps += 1;

        redraw_screen
    }

    pub fn input(&mut self) -> &mut Input {
        self.mem.input()
    }

    pub fn read_region(&self, start: u16, end: u16) -> Vec<u8> {
        assert!(end >= start);
        let mut result = Vec::with_capacity(usize::from(end - start + 1));

        for i in start..=end {
            result.push(self.mem.get(i));
        }

        result
    }

    pub fn gpu_trace(&self) -> GpuDebugTrace {
        self.gpu.debug_last_frame.clone()
    }

    pub fn buffer(&self) -> *const u8 {
        self.gpu.screen_rgba.as_ptr()
    }

    pub fn buffer_vec(&self) -> &Vec<u8> {
        &self.gpu.screen_rgba
    }
}
