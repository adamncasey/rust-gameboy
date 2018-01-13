use memory::Memory;

pub const GB_HSIZE: usize = 160;
pub const GB_VSIZE: usize = 144;

const LCD_ON_BIT: u8 = 1 << 7;
const WINDOW_TILEMAP_BIT: u8 = 1 << 6;
const WINDOW_DISP_BIT: u8 = 1 << 5;
const TILEDATA_BIT: u8 = 1 << 4;
const BG_TILEMAP_BIT: u8 = 1 << 3;
//const SPRITE_SIZE_BIT: u8 = 1 << 2;
const SPRITE_DISP_BIT: u8 = 1 << 1;
const BG_DISP_BIT: u8 = 1 << 0;

#[derive(Debug)]
enum GpuMode {
    OAMRead,
    VRAMRead,
    HBlank,
    VBlank,
}

pub enum GpuInterrupt {
    None,
    VBlank,
    LCDStatus,
}

pub struct Gpu {
    mode: GpuMode,
    mode_elapsed: u32,
    line: u8,
    pub screen_rgba: Vec<u8>,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            mode: GpuMode::HBlank,
            mode_elapsed: 0,
            line: 0,
            screen_rgba: vec![255; GB_VSIZE * GB_HSIZE * 4],
        }
    }

    pub fn cycle(&mut self, mem: &mut Memory, elapsed: u8) -> GpuInterrupt {
        // TODO currently load this byte twice
        let lcdc: u8 = mem.get(0xFF40);
        if (lcdc & LCD_ON_BIT) == 0 {
            self.line = 0;
            self.mode_elapsed = 0;
            self.mode = GpuMode::VBlank;
            return GpuInterrupt::None;
        }

        let mut vblank = false;

        self.mode_elapsed += elapsed as u32;
        match self.mode {
            GpuMode::OAMRead => if self.mode_elapsed >= 80 {
                self.mode = GpuMode::VRAMRead;
                self.mode_elapsed -= 80;
            },
            GpuMode::VRAMRead => if self.mode_elapsed >= 172 {
                self.mode = GpuMode::HBlank;
                self.mode_elapsed -= 172;
                Gpu::draw_line(self.line, mem, &mut self.screen_rgba);
            },
            GpuMode::HBlank => {
                if self.mode_elapsed >= 204 {
                    self.mode_elapsed -= 204;
                    self.line += 1;

                    if self.line == 143 {
                        self.mode = GpuMode::VBlank;
                        vblank = true;
                    } else {
                        self.mode = GpuMode::OAMRead;
                    }
                }
            }
            GpuMode::VBlank => if self.mode_elapsed >= 456 {
                self.line += 1;
                self.mode_elapsed -= 456;

                if self.line > 153 {
                    // Blank screen for re-writing
                    self.screen_rgba.resize(0, 255);
                    self.screen_rgba.resize(GB_VSIZE * GB_HSIZE * 4, 255);
                    self.mode = GpuMode::OAMRead;
                    self.line = 0;
                }
            },
        };

        mem.set(0xFF44, self.line);

        let lcdstat: u8 = mem.get(0xFF41);

        let mode: u8 = match self.mode {
            GpuMode::HBlank => 0,
            GpuMode::VBlank => 1,
            GpuMode::OAMRead => 2,
            GpuMode::VRAMRead => 3,
        };

        let newlcdstat: u8 = (lcdstat & 0xFC) | mode;

        mem.set(0xFF41, newlcdstat);

        if vblank {
            return GpuInterrupt::VBlank;
        }
        else if self.lcd_status_interrupt() {
            return GpuInterrupt::LCDStatus;
        }

        return GpuInterrupt::None;
    }

    fn lcd_status_interrupt(&self) -> bool {
        // TODO Should we trigger an LCD Status int?
        return false;
    }

    fn draw_line(line: u8, mem: &Memory, rgba: &mut [u8]) {
        let lcdc: u8 = mem.get(0xFF40);
        let tiles = tiles_start((lcdc & TILEDATA_BIT) != 0);

        let bg_win_colours: u8 = mem.get(0xFF47);

        if lcdc & BG_DISP_BIT != 0 {
            let bgmap: bool = (lcdc & BG_TILEMAP_BIT) != 0;
            let tilemap = bg_tilemap(bgmap);
            draw_background(line, mem, bg_win_colours, tiles, tilemap, bgmap, rgba);
        }

        if lcdc & WINDOW_DISP_BIT != 0 {
            let _tilemap = win_tilemap(lcdc & WINDOW_TILEMAP_BIT != 0);
            //draw_window(line, mem, bg_win_colours, tiles, tilemap, rgba);
        }

        if lcdc & SPRITE_DISP_BIT != 0 {
            //draw_sprites(line, mem, lcdc & SPRITE_SIZE_BIT != 0, rgba);
        }
    }

    pub fn print_state(&self) {
        println!(
            "mode {:?} elapsed {} line {}",
            self.mode,
            self.mode_elapsed,
            self.line
        );
    }
}

fn draw_background(line: u8, mem: &Memory, bgp: u8, tiledata: u16, tilemap: u16, bgmap: bool, rgba: &mut [u8]) {
    let scy: u8 = mem.get(0xFF42);
    let bgy = (line + scy) as u16;
    let vtile = bgy / 8;

    if vtile >= 32 {
        println!("reached vend of tile {} {}", vtile, line);
        return;
    }

    let ty: u16 = (bgy % 8) as u16;
    
    let scx: u8 = mem.get(0xFF43);

    for i in 0..GB_HSIZE {
        let bgx = (i as u16) + (scx as u16);
        let htile = bgx / 8;
        if htile >= 32 {
            println!("reached hend of tile {} {} {}", htile, line, i);
            return;
        }

        let tx: u16 = (bgx % 8) as u16;

        let mut tilenumtemp: u8 = mem.get(tilemap + vtile * 32 + htile);

        let tilenum: i32 = if !bgmap {
            (tilenumtemp as i8) as i32
        }
        else {
            (tilenumtemp as u16) as i32
        };
        
        const TILE_SIZE: i32 = 16;
        let signedtiledata: i32 = tiledata as u32 as i32;
        let tilestart = (signedtiledata + tilenum * TILE_SIZE) as u16;
        
        let tilerow = tilestart + (ty * 2);
        let bit = 0b1 << (7 - tx);

        // TODO draw all eight pixels at once.
        let rowbyte1 = mem.get(tilerow);
        let rowbyte2 = mem.get(tilerow + 1);

        let pixel = ((rowbyte1 & bit) >> (7 - tx)) | (((rowbyte2 & bit) >> (7 - tx)) << 1);
        let colour = apply_palette(pixel, bgp);

        rgba[((line as usize) * GB_HSIZE + i as usize) * 4] = colour;
        rgba[((line as usize) * GB_HSIZE + i as usize) * 4 + 1] = colour;
        rgba[((line as usize) * GB_HSIZE + i as usize) * 4 + 2] = colour;
        rgba[((line as usize) * GB_HSIZE + i as usize) * 4 + 3] = 255;
    }
                
}

fn apply_palette(colour: u8, pal: u8) -> u8 {
    match colour {
        3 => get_colour((pal & 0b11000000) >> 6),
        2 => get_colour((pal & 0b00110000) >> 4),
        1 => get_colour((pal & 0b00001100) >> 2),
        0 => get_colour(pal & 0b00000011),
        _ => panic!("Invalid colour {}", colour),
    }
}

fn get_colour(colour: u8) -> u8 {
    match colour {
        0 => 0xFF,
        1 => 0xC0,
        2 => 0x60,
        3 => 0x00,
        _ => panic!("Invalid colour {}", colour),
    }
}

fn bg_tilemap(bit: bool) -> u16 {
    if bit {
        0x9C00
    } else {
        0x9800
    }
}

fn win_tilemap(bit: bool) -> u16 {
    if bit {
        0x9C00
    } else {
        0x9800
    }
}

fn tiles_start(bit: bool) -> u16 {
    if bit {
        0x8000
    } else {
        0x9000
    }
}
