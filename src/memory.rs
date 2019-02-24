use crate::input::Input;

pub struct Memory {
    cartridge: Vec<u8>,
    vram: Vec<u8>,
    ram: Vec<u8>,
    eram: Vec<u8>,
    sprite: Vec<u8>,
    io: Vec<u8>,
    highram: Vec<u8>,

    // All unused memory is forwarded to the same byte
    // TODO reads shouldn't be affected by writes
    unused: u8,
    // Input is always interfaced via the MMU
    input: Input,
}

const VRAM_SIZE: usize = 8 * 1024;
const RAM_SIZE: usize = 8 * 1024;
const ERAM_SIZE: usize = 8 * 1024;
const SPRITE_SIZE: usize = 160;
const IO_SIZE: usize = 76;
const HIGHRAM_SIZE: usize = 128;

impl Memory {
    pub fn new(rom_contents: Vec<u8>) -> Memory {
        // Move rom contents into Memory
        let mut mem = Memory {
            cartridge: rom_contents,
            vram: vec![0; VRAM_SIZE],
            ram: vec![0; RAM_SIZE],
            eram: vec![0; ERAM_SIZE],
            sprite: vec![0; SPRITE_SIZE],
            io: vec![0; IO_SIZE],
            highram: vec![0; HIGHRAM_SIZE],

            unused: 0,
            input: Input::new(),
        };

        mem.set(0xFF40, 0x91);

        mem
    }

    pub fn get(&self, addr: u16) -> u8 {
        match addr {
            0xFF00 => self.input.value(),
            _ => *self.mmu(addr),
        }
    }

    pub fn set(&mut self, addr: u16, val: u8) {
        *self.mmu_mut(addr) = val;

        self.special(addr, val);
    }

    pub fn get16(&self, addr: u16) -> u16 {
        let low: u8 = self.get(addr);
        let high: u16 = u16::from(self.get(addr + 1));

        (high << 8) + u16::from(low)
    }

    pub fn set16(&mut self, addr: u16, val: u16) {
        let low = val & 0x00FF;
        let high = (val & 0xFF00) >> 8;

        self.set(addr, low as u8);
        self.set(addr + 1, high as u8);
    }

    fn mmu_mut(&mut self, addr: u16) -> &mut u8 {
        match addr {
            0x0000...0x7FFF => &mut self.unused,
            0x8000...0x9FFF => &mut self.vram[(addr - 0x8000) as usize],
            0xA000...0xBFFF => &mut self.eram[(addr - 0xA000) as usize],
            0xC000...0xDFFF => &mut self.ram[(addr - 0xC000) as usize],
            0xE000...0xFDFF => &mut self.ram[(addr - 0xE000) as usize],
            0xFE00...0xFE9F => &mut self.sprite[(addr - 0xFE00) as usize],
            0xFEA0...0xFEFF => &mut self.unused,
            0xFF00...0xFF4B => &mut self.io[(addr - 0xFF00) as usize],
            0xFF4C...0xFF7F => &mut self.unused,
            0xFF80...0xFFFF => &mut self.highram[(addr - 0xFF80) as usize],
            _ => panic!("Unknown memory region 0x{:X}", addr),
        }
    }

    fn mmu(&self, addr: u16) -> &u8 {
        match addr {
            0x0000...0x7FFF => &self.cartridge[addr as usize],
            0x8000...0x9FFF => &self.vram[(addr - 0x8000) as usize],
            0xA000...0xBFFF => &self.eram[(addr - 0xA000) as usize],
            0xC000...0xDFFF => &self.ram[(addr - 0xC000) as usize],
            0xE000...0xFDFF => &self.ram[(addr - 0xE000) as usize],
            0xFE00...0xFE9F => &self.sprite[(addr - 0xFE00) as usize],
            0xFEA0...0xFEFF => &self.unused,
            0xFF00...0xFF4B => &self.io[(addr - 0xFF00) as usize],
            0xFF4C...0xFF7F => &self.unused,
            0xFF80...0xFFFF => &self.highram[(addr - 0xFF80) as usize],
            _ => panic!("Unknown memory region 0x{:X}", addr),
        }
    }
    fn special(&mut self, addr: u16, val: u8) {
        // TODO Handle input register
        match addr {
            0xFF00 => {
                self.input.update(val);
            }
            0xFF46 => {
                // OAM Write
                // TODO SLOW This could be a lot faster
                //println!("DMA from {:x}", source);
                let source: u16 = u16::from(val) << 8;
                let target: u16 = 0xFE00;
                for i in 0..160 {
                    let val: u8 = self.get(source + i);
                    self.set(target + i, val);
                }
            }
            _ => (),
        }
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }
}
