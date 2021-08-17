pub use crate::cpu::Cpu;
use crate::gpu::{Gpu, GpuDebugTrace, GpuMode};
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
    pub cycles: u64,
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
            cycles: 0,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn cycle(&mut self, debug: bool, debug_pc_panic: Option<u16>) -> bool {
        if debug {
            print_cpu_trace(&self.cpu, self.cycles);
            /*println!(
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
            );*/
        }

        let cycles: u8 = self.cpu.cycle(&mut self.mem, debug);
        self.cycles += cycles as u64;

        if let Some(pc) = debug_pc_panic {
            if self.cpu.pc == pc {
                panic!("Reached panic-at-pc");
            }
        }

        let old_mode = self.gpu.mode;

        self.gpu.cycle(&mut self.mem, cycles);

        let redraw_screen = old_mode != GpuMode::VBlank && self.gpu.mode == GpuMode::VBlank;

        self.mem.tick_timer(cycles);

        let int = interrupt::fetch_interrupt(&mut self.mem);
        if let Some(active) = int {
            if self.cpu.interrupt(&mut self.mem, active) {
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

fn print_cpu_trace(cpu: &Cpu, cycles: u64) {
    let z = if cpu.z_flag() { 'Z' } else {'-'};
    let n = if cpu.n_flag() { 'N' } else {'-'};
    let c = if cpu.c_flag() { 'C' } else {'-'};
    let h = if cpu.h_flag() { 'H' } else {'-'};

    println!("A:{:02X} F:{}{}{}{} BC:{:02X}{:02X} DE:{:02x}{:02x} HL:{:02x}{:02x} SP:{:04x} PC:{:04x} (cy: {})", cpu.a, z, n, h, c, cpu.b, cpu.c, cpu.d, cpu.e, cpu.h, cpu.l, cpu.sp, cpu.pc, cycles);
}