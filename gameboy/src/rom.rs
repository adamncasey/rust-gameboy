use std::str;

const CARTRIDGE_DEFAULT_RAM_SIZE: usize = 8 * 1024;

const CARTRIDGE_RAM_SIZE_01: usize = 2 * 1024;
const CARTRIDGE_RAM_SIZE_03: usize = 32 * 1024;

const BANK_SIZE: usize = 16 * 1024;

const RAM_BANK_SIZE: usize = 8 * 1024;

#[derive(Debug)]
pub struct Cartridge {
    pub game_title: String,
    pub rom_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub rom_contents: Vec<u8>,
    pub rom_bank: usize,
    pub ram_bank: usize,

    pub ram: Vec<u8>,
}

const ROM_TITLE_START: usize = 0x134;
const ROM_TITLE_LEN: usize = 16;
const ROM_TITLE_END: usize = ROM_TITLE_START + ROM_TITLE_LEN;
const ROM_TYPE_OFFSET: usize = 0x147;
const ROM_SIZE_OFFSET: usize = 0x148;
const RAM_SIZE_OFFSET: usize = 0x149;

impl Cartridge {
    pub fn load_rom(rom_contents: Vec<u8>) -> Cartridge {
        let mut rom = Cartridge {
            game_title: String::from(""),
            rom_type: rom_contents[ROM_TYPE_OFFSET as usize],
            rom_size: rom_contents[ROM_SIZE_OFFSET as usize],
            ram_size: rom_contents[RAM_SIZE_OFFSET as usize],
            rom_contents,
            rom_bank: 0,
            ram_bank: 0,

            ram: vec![0; CARTRIDGE_DEFAULT_RAM_SIZE],
        };

        // Copy out game_title
        let bytes = &rom.rom_contents[ROM_TITLE_START..ROM_TITLE_END];

        let nul_range_end = bytes
            .iter()
            .position(|&c| c == b'\0')
            .unwrap_or(ROM_TITLE_LEN);

        rom.game_title = str::from_utf8(&bytes[0..nul_range_end])
            .unwrap_or("Empty title")
            .to_string();

        if rom.rom_type == 0x13 {
            // confirm size of rom
            if rom.rom_contents.len() < 1024 * 1024 {
                panic!(
                    "ROM {:} is of type {:?} but only has {:?}kb of contents",
                    rom.game_title,
                    rom.rom_type,
                    rom.rom_contents.len() / 1024
                );
            }
        }

        if rom.ram_size == 0x01 {
            rom.ram = vec![0; CARTRIDGE_RAM_SIZE_01];
        }
        if rom.ram_size == 0x03 {
            rom.ram = vec![0; CARTRIDGE_RAM_SIZE_03];
        }

        rom
    }

    pub fn mbc_write(&mut self, addr: u16, val: u8) {
        //println!("mbc write 0x{:x} {}", addr, val);
        if self.rom_type == 0 {
            return;
        }

        match addr {
            0x2000..=0x3FFF => {
                let bank: u8 = val & 0b00111111;
                println!("bank switch to {}", bank);

                self.rom_bank = bank as usize;
            }
            0x4000..=0x5FFF => {
                self.ram_bank = val as usize;
                /*let upper_bits = val & 0b01100000;
                self.rom_bank = (self.rom_bank & 0b00011111) | upper_bits as usize;*/
                println!("RAM Bank switch to {}", self.ram_bank);
            }
            _ => { /*println!("Unknown mbc write 0x{:x} = {}", addr, val);*/ }
        }
    }

    pub fn mbc_rom_mut(&mut self, addr: u16) -> &mut u8 {
        match addr {
            0x0000..=0x3FFF => &mut self.rom_contents[addr as usize],
            0x4000..=0x7FFF => {
                &mut self.rom_contents
                    [BANK_SIZE + self.rom_bank * BANK_SIZE + (addr - 0x4000) as usize]
            }
            0xA000..=0xBFFF => {
                &mut self.ram[RAM_BANK_SIZE * self.ram_bank + (addr - 0xA000) as usize]
            }
            _ => {
                panic!("ROM MBC invalid address 0x{:x}", addr);
            }
        }
    }

    pub fn mbc(&self, addr: u16) -> &u8 {
        match addr {
            0x0000..=0x3FFF => &self.rom_contents[addr as usize],
            0x4000..=0x7FFF => {
                &self.rom_contents[BANK_SIZE + self.rom_bank * BANK_SIZE + (addr - 0x4000) as usize]
            }
            0xA000..=0xBFFF => &self.ram[RAM_BANK_SIZE * self.ram_bank + (addr - 0xA000) as usize],
            _ => {
                panic!("ROM MBC invalid address 0x{:x}", addr);
            }
        }
    }
}
