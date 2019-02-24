use crate::cpu::{Cpu, CpuInterrupt};
use crate::gpu::{Gpu, GpuInterrupt};
use crate::input::Input;
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

        let igpu = self.gpu.cycle(&mut self.mem, cycles);
        let ijoypad = self.mem.input().fetch_interrupt();
        let itimer = false;

        let int = self.get_interrupt(igpu, ijoypad, itimer);
        if let CpuInterrupt::None = int {
            // nothing TODO syntax
        } else {
            self.cpu.interrupt(&mut self.mem, int);
        }

        self.steps += 1;
        if debug {
            println!(
                "State after {} total steps {} cycles: {}",
                self.steps,
                cycles,
                self.cpu.print_state()
            );
        }

        if let CpuInterrupt::VBlank = int {
            screen_rgba.copy_from_slice(&self.gpu.screen_rgba);
            true
        } else {
            false
        }
    }

    pub fn input(&mut self) -> &mut Input {
        self.mem.input()
    }

    fn get_interrupt(&mut self, igpu: GpuInterrupt, ijoypad: bool, itimer: bool) -> CpuInterrupt {
        match igpu {
            GpuInterrupt::VBlank => CpuInterrupt::VBlank,
            GpuInterrupt::LCDStatus => CpuInterrupt::LCDStatus,
            _ => {
                if itimer {
                    CpuInterrupt::Timer
                } else if ijoypad {
                    CpuInterrupt::Joypad
                } else {
                    CpuInterrupt::None
                }
            }
        }
    }
}
