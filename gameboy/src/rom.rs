use std::str;

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
    pub fn load(rom_contents: Vec<u8>) -> Rom {
        let mut rom = Rom {
            game_title: String::from(""),
            rom_type: 255,
            rom_contents: rom_contents,
        };

        // Copy out game_title
        let bytes = &rom.rom_contents[ROM_TITLE_START..ROM_TITLE_END];
        rom.game_title = str::from_utf8(bytes).unwrap_or("Empty title").to_string();

        // Copy out rom_type
        rom.rom_type = rom.rom_contents[ROM_TYPE_OFFSET as usize];

        rom
    }
}
