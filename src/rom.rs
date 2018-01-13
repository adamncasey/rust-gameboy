use std::io;
use std::str;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Rom {
    pub game_title: String,
    pub rom_type: u8,
    pub rom_contents: Vec<u8>,
}

const ROM_TITLE_START: usize = 0x134;
const ROM_TITLE_END: usize = ROM_TITLE_START + 16;
const ROM_TYPE_OFFSET: usize = 0x147;

impl Rom {
    pub fn load(path: &str) -> io::Result<Rom> {
        // Load file contents at path into rom_contents
        let mut file = File::open(path)?;

        let mut rom = Rom {
            game_title: String::from(""),
            rom_type: 255,
            rom_contents: vec![],
        };
        file.read_to_end(&mut rom.rom_contents)?;

        // Copy out game_title
        rom.game_title = String::from(
            str::from_utf8(&rom.rom_contents[ROM_TITLE_START..ROM_TITLE_END]).unwrap(),
        );

        // Copy out rom_type
        rom.rom_type = rom.rom_contents[ROM_TYPE_OFFSET as usize];

        Ok(rom)
    }
}
