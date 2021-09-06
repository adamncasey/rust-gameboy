use std::str;

const CARTRIDGE_DEFAULT_RAM_SIZE: usize = 8 * 1024;

const CARTRIDGE_RAM_SIZE_01: usize = 2 * 1024;
const CARTRIDGE_RAM_SIZE_03: usize = 32 * 1024;

const BANK_SIZE: usize = 16 * 1024;

const RAM_BANK_SIZE: usize = 8 * 1024;

#[derive(Debug)]
pub struct Cartridge {
    pub game_title: String,
    pub mbc_type: MbcType,
    pub rom_size: u8,
    pub ram_size: u8,
    pub rom_contents: Vec<u8>,

    pub ram: Vec<u8>,

    unused: u8,
}

#[derive(Debug)]
pub enum MbcType {
    None,
    Mbc1 {
        rom_bank: u8,
        ram_bank: Option<u8>
    },
    Mbc3 {
        rom_bank: u8,
        ram_bank: Option<u8>,
    }
}

impl MbcType {
    fn from_byte(n: u8) -> MbcType {
        match n {
            0x00 => MbcType::None,
            0x01..=0x03 => MbcType::Mbc1 { rom_bank: 1, ram_bank: None },
            0x0f..=0x13 => MbcType::Mbc3 { rom_bank: 1, ram_bank: None },
            _ => { panic!("Unsupported ROM Type: {}", n); }
        }
    }

    fn rom_banked_offset(&self, addr: u16) -> usize {
        let bank = match self {
            MbcType::None => Some(1),
            MbcType::Mbc1 { rom_bank, ..} => {
                Some(*rom_bank as usize)
            },
            MbcType::Mbc3 { rom_bank, ..} => {
                Some(*rom_bank as usize)
            }
        };
        let offset = (addr - 0x4000) as usize;
        if let Some(bank) = bank {
            bank * BANK_SIZE + offset
        }
        else {
            offset
        }
    }
    
    fn ram_banked_offset(&self, addr: u16) -> usize {
        let bank = match self {
            MbcType::None => Some(0),
            MbcType::Mbc1 { rom_bank, ram_bank } => {
                ram_bank.map(|ram_bank| ram_bank as usize)
            },
            MbcType::Mbc3 { rom_bank, ram_bank } => {
                ram_bank.map(|ram_bank| ram_bank as usize)
            }
        };

        let offset = (addr - 0xA000) as usize;
        let result = if let Some(bank) = bank {
            bank * RAM_BANK_SIZE + offset
        }
        else {
            offset
        };

        //println!("ram_banked_offset {:?} {:4x}", bank, result);

        result
    }
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
            mbc_type: MbcType::from_byte(rom_contents[ROM_TYPE_OFFSET as usize]),
            rom_size: rom_contents[ROM_SIZE_OFFSET as usize],
            ram_size: rom_contents[RAM_SIZE_OFFSET as usize],
            rom_contents,

            ram: vec![0; CARTRIDGE_DEFAULT_RAM_SIZE],
            unused: 0,
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

        if rom.ram_size == 0x01 {
            rom.ram = vec![0; CARTRIDGE_RAM_SIZE_01];
        }
        if rom.ram_size == 0x03 {
            rom.ram = vec![0; CARTRIDGE_RAM_SIZE_03];
        }

        rom
    }

    pub fn mbc_write(&mut self, addr: u16, val: u8) {
        match self.mbc_type {
            MbcType::None => {},
            MbcType::Mbc1 { ref mut rom_bank, ref mut ram_bank } => {
                match addr {
                    0x0000..=0x1fff => {
                        if (val & 0x0f) == 0x0A {
                            *ram_bank = Some(0);
                        }
                        else {
                            *ram_bank = None;
                        }
                    },
                    0x2000..=0x3fff => {
                        let bank: u8 = val & 0b00011111;
                        let bank = if bank == 0 { 1 } else { bank };
                        let bank = (*rom_bank & 0b01100000) | bank;
        
                        *rom_bank = bank;

                        //println!("Set ROM Bank to {}", bank);
                    },
                    0x4000..=0x5FFF => {
                        let bank: u8 = val & 0b11;

                        *ram_bank = Some(bank);
                        
                        //println!("Set RAM Bank to {}", bank);
                    },
                    _ => {}
                };
            },
            MbcType::Mbc3 { ref mut rom_bank, ref mut ram_bank }=> {
                match addr {
                    0x0000..=0x1fff => {
                        if (val & 0x0f) == 0x0A {
                            *ram_bank = Some(1);
                        }
                        else {
                            *ram_bank = None;
                        }
                    },
                    0x2000..=0x3fff => {
                        let bank: u8 = val & 0b01111111;
                        let bank = if bank == 0 { 1 } else { bank };

                        *rom_bank = bank;
                    }
                    0x4000..=0x5FFF => {
                        let bank: u8 = val & 0b11;

                        *ram_bank = Some(bank);
                        
                        println!("Set RAM Bank to {}", bank);
                    },
                    _ => {}
                };
            }
        };
    }

    pub fn mbc_rom_mut(&mut self, addr: u16) -> &mut u8 {
        match addr {
            0x0000..=0x3FFF => {
                //println!("Writing to ROM? {:4x}", addr);
                &mut self.unused
            }
            0x4000..=0x7FFF => {
                &mut self.rom_contents[self.mbc_type.rom_banked_offset(addr)]
            }
            0xA000..=0xBFFF => {
                let ram_offset = self.mbc_type.ram_banked_offset(addr);
                //println!("Write RAM {:4x}", ram_offset);
                &mut self.ram[ram_offset]
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
                &self.rom_contents[self.mbc_type.rom_banked_offset(addr)]
            }
            0xA000..=0xBFFF => {
                let ram_offset = self.mbc_type.ram_banked_offset(addr);
                //println!("Read RAM {:4x}", ram_offset);
                &self.ram[ram_offset]
            },
            _ => {
                panic!("ROM MBC invalid address 0x{:x}", addr);
            }
        }
    }
}
