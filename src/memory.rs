use rom::Rom;

pub struct Memory {
    cartridge: Vec<u8>,
    // Cartridge ROM
    // RAM
    // ???
}

impl Memory {
    pub fn new(rom: Rom) -> Memory {
        // Move rom contents into Memory
        Memory{ cartridge: rom.rom_contents }
    }

    pub fn get(&self, addr: u16) -> u8 {
        self.cartridge[addr as usize]
    }
}