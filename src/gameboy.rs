use rom::Rom;
use cpu::{Cpu, CpuInterrupt};
use gpu::{Gpu, GpuInterrupt};
use memory::Memory;

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
            panic!("ROM type unsupported {}", cartridge.rom_type);
        }

        GameBoy {
            title: cartridge.game_title,
            cpu: Cpu::new(),
            gpu: Gpu::new(),
            mem: Memory::new(cartridge.rom_contents),
            steps: 0
        }
    }

    pub fn title(&self) -> &str {
        return &self.title;
    }

    pub fn cycle(&mut self, screen_rgba: &mut Vec<u8>, debug: bool) -> bool {
        let cycles: u8 = self.cpu.cycle(&mut self.mem, debug);

        let igpu = self.gpu.cycle(&mut self.mem, cycles);
        let ijoypad = false;
        let itimer = false;

        let int = self.get_interrupt(igpu, ijoypad, itimer);

        self.cpu.interrupt(&mut self.mem, int);

        self.steps += 1;
        if debug {
            println!("State after {} total steps {} cycles", self.steps, cycles);
            self.cpu.print_state();
        }

        if let CpuInterrupt::VBlank = int {
            screen_rgba.copy_from_slice(&self.gpu.screen_rgba);
            return true;
        }

        return false;
    }

    fn get_interrupt(&mut self, igpu: GpuInterrupt, ijoypad: bool, itimer: bool) -> CpuInterrupt {
        match igpu {
            GpuInterrupt::VBlank => CpuInterrupt::VBlank,
            GpuInterrupt::LCDStatus => CpuInterrupt::LCDStatus,
            _ => if itimer {
                CpuInterrupt::Timer
            } else if ijoypad {
                CpuInterrupt::Joypad
            } else {
                CpuInterrupt::None
            }
        }
    }
}