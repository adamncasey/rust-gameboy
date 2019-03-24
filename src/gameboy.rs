use crate::cpu::Cpu;
use crate::gpu::{Gpu, GpuDebugTrace};
use crate::input::Input;
use crate::interrupt;
use crate::memory::Memory;
use crate::rom::Rom;

pub struct GameBoy {
    title: String,
    cpu: Cpu,
    gpu: Gpu,
    mem: Memory,

    steps: u64,
}

impl GameBoy {
    pub fn new(rom_filename: &str) -> GameBoy {
        let cartridge: Rom = Rom::load(rom_filename).unwrap();

        if cartridge.rom_type != 0 {
            println!("ROM type unsupported {}", cartridge.rom_type);
        }

        GameBoy {
            title: cartridge.game_title,
            cpu: Cpu::new(),
            gpu: Gpu::new(),
            mem: Memory::new(cartridge.rom_contents),
            steps: 0,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn cycle(&mut self, screen_rgba: &mut Vec<u8>, debug: bool) -> bool {
        let cycles: u8 = self.cpu.cycle(&mut self.mem, debug);

        self.gpu.cycle(&mut self.mem, cycles);

        self.mem.tick_timer(cycles);

        let mut redraw_screen = false;

        let int = interrupt::fetch_interrupt(&mut self.mem);
        if let Some(active) = int {
            if self.cpu.interrupt(&mut self.mem, active) {
                screen_rgba.copy_from_slice(&self.gpu.screen_rgba);
                redraw_screen = true;
            }
        }

        self.steps += 1;
        if debug {
            println!(
                "State after {} total steps {} cycles: {}. | Gpu PWR {} mode: {:?} elapsed {} line {}",
                self.steps,
                cycles,
                self.cpu.print_state(),
                self.gpu.debug_lcd_pwr,
                self.gpu.mode,
                self.gpu.mode_elapsed,
                self.gpu.line
            );
        }

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
}
