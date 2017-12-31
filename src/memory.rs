use rom::Rom;

pub struct Memory {
    cartridge: Vec<u8>,
    vram: Vec<u8>,
    ram: Vec<u8>,
    sprite: Vec<u8>,
    io: Vec<u8>,
    highram: Vec<u8>,
}

const VRAM_SIZE: usize = 8 * 1024;
const RAM_SIZE: usize = 8 * 1024;
const SPRITE_SIZE: usize = 154;
const IO_SIZE: usize = 76;
const HIGHRAM_SIZE: usize = 128;

impl Memory {
    pub fn new(rom: Rom) -> Memory {
        // Move rom contents into Memory
        Memory {
            cartridge: rom.rom_contents,
            vram: vec![0; VRAM_SIZE],
            ram: vec![0; RAM_SIZE],
            sprite: vec![0; SPRITE_SIZE],
            io: vec![0; IO_SIZE],
            highram: vec![0; HIGHRAM_SIZE],
        }
    }

    pub fn get(&self, addr: u16) -> u8 {
        *self.mmu(addr)
    }

    pub fn set(&mut self, addr: u16, val: u8) {
        *self.mmu_mut(addr) = val;
    }

    pub fn get16(&self, addr: u16) -> u16 {
        let high: u16 = self.get(addr) as u16;
        let low: u8 = self.get(addr + 1);

        return (high << 8) + (low as u16);
    }

    pub fn set16(&mut self, addr: u16, val: u16) {
        let low = val & 0x00FF;
        let high = (val & 0xFF00) >> 8;

        self.set(addr + 1, low as u8);
        self.set(addr, high as u8);
    }

    fn mmu_mut(&mut self, addr: u16) -> &mut u8 {
        match addr {
            0x0000...0x7FFF => &mut self.cartridge[addr as usize],
            0x8000...0x9FFF => &mut self.vram[(addr - 0x8000) as usize],
            0xFF80...0xFFFF => &mut self.highram[(addr - 0xFF80) as usize],
            _ => panic!("Unknown memory region 0x{:X}", addr),
        }
    }    

    fn mmu(&self, addr: u16) -> &u8 {
        match addr {
            0x0000...0x7FFF => &self.cartridge[addr as usize],
            0x8000...0x9FFF => &self.vram[(addr - 0x8000) as usize],
            0xFF80...0xFFFF => &self.highram[(addr - 0xFF80) as usize],
            _ => panic!("Unknown memory region 0x{:X}", addr),
        }
    }
}
