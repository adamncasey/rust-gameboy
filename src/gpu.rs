use memory::Memory;

pub const GB_HSIZE: usize = 160;
pub const GB_VSIZE: usize = 144;

const LCD_ON_BIT: u8 = 1 << 7;
const WINDOW_TILEMAP_BIT: u8 = 1 << 6;
const WINDOW_DISP_BIT: u8 = 1 << 5;
const TILEDATA_BIT: u8 = 1 << 4;
const BG_TILEMAP_BIT: u8 = 1 << 3;
const SPRITE_SIZE_BIT: u8 = 1 << 2;
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
        // TODO SLOW currently load this byte twice
        let lcdc: u8 = mem.get(0xFF40);
        if (lcdc & LCD_ON_BIT) == 0 {
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
            GpuMode::HBlank => if self.mode_elapsed >= 204 {
                self.mode_elapsed -= 204;
                self.line += 1;

                if self.line == 143 {
                    self.mode = GpuMode::VBlank;
                    vblank = true;
                } else {
                    self.mode = GpuMode::OAMRead;
                }
            },
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

        //println!("GPU State: {:?} {:?} {:?}", self.mode, self.line, self.mode_elapsed);

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
        } else if self.lcd_status_interrupt() {
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
            let tilemap = select_tilemap(bgmap);
            draw_background(line, mem, bg_win_colours, tiles, tilemap, bgmap, rgba);
        }

        if lcdc & WINDOW_DISP_BIT != 0 {
            let _tilemap = select_tilemap(lcdc & WINDOW_TILEMAP_BIT != 0);
            //draw_window(line, mem, bg_win_colours, tiles, tilemap, rgba);
        }

        if lcdc & SPRITE_DISP_BIT != 0 {
            let sprite_height: u8 = get_sprite_size(lcdc);
            draw_sprites(line, mem, sprite_height, tiles, rgba);
        }
        else {
            //println!("Sprites disabled {:X} {}", lcdc, lcdc & SPRITE_DISP_BIT);
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

fn draw_background(
    line: u8,
    mem: &Memory,
    bgp: u8,
    tiledata: u16,
    tilemap: u16,
    bgmap: bool,
    rgba: &mut [u8],
) {
    let scy: u8 = mem.get(0xFF42);
    let bgy = (line + scy) as u16;
    let vtile = bgy / 8;

    if vtile >= 32 {
        println!("reached vend of tile {} {}", vtile, line);
        return;
    }

    let ty: u16 = bgy % 8;

    let scx: u8 = mem.get(0xFF43);

    for i in 0..(GB_HSIZE + 1) {
        let bgx = (i as u16) + (scx as u16);
        let htile = bgx / 8;
        if htile >= 32 {
            println!("reached hend of tile {} {} {}", htile, line, i);
            return;
        }

        let tx: u8 = (bgx % 8) as u8;

        let mut tilenumtemp: u8 = mem.get(tilemap + vtile * 32 + htile);

        let tilenum: i32 = if !bgmap {
            (tilenumtemp as i8) as i32
        } else {
            (tilenumtemp as u16) as i32
        };

        // TODO SLOW draw all eight pixels at once.
        let tilerow = get_tile_row_data(mem, tiledata, tilenum, ty);
        let colour = get_tile_colour(tilerow, tx, bgp);

        let rgba_start = ((line as usize) * GB_HSIZE + i as usize) * 4;
        set_pixel(rgba, rgba_start, colour);
    }
}

fn draw_sprites(line: u8, mem: &Memory, sprite_height: u8, tiledata: u16, rgba: &mut [u8]) {
    let palettes = (mem.get(0xFF48), mem.get(0xFF49));
    // for each sprite
    for i in 0..40 {
        let s = load_sprite(mem, i, palettes);

        if !sprite_in_row(line as i16, s.y, sprite_height) || !sprite_on_disp(s.x) {
            //println!("Skipping sprite y line{} s{} y{} x{}", line, i, s.y, s.x);
            continue;
        }

        // look up tile pixel data
        let ty = (s.y - line as i16) as u16; // TODO yflip

        // TODO xflip:
        for tx in 0..8 {
            let x = s.x + tx;
            if x < 0 || x > GB_HSIZE as i16 {
                println!("Skipping sprite x {} {}", x, tx);
                continue;
            }
            let rgba_start = (line as usize * GB_HSIZE + x as usize) * 4;

            // Is priority bit set or is the bg value zero?
            if !s.priority && rgba[rgba_start] != 255 {
                //println!("Not drawing pixel due to priority / bg colour {}", rgba[rgba_start]);
                continue;
            }
            // draw pixel
            let tilerow = get_tile_row_data(mem, tiledata, s.tile as u32 as i32, ty);
            let colour = get_tile_colour(tilerow, tx as u8, s.palette);

            // Is this pixel transparent?
            if colour != 0 {
                println!("Drawn pixel {:X} {}", rgba_start, colour);
                set_pixel(rgba, rgba_start, colour);
            } else {
                println!("Skipped pixel {}", colour);
            }
        }
    }
}

struct Sprite {
    y: i16,
    x: i16,
    tile: u8,
    priority: bool,
    yflip: bool,
    xflip: bool,
    palette: u8,
}

const SPRITE_MEM_START: u16 = 0xFE00;
const SPRITE_MEM_SIZE: u16 = 4;

fn load_sprite(mem: &Memory, num: u16, palettes: (u8, u8)) -> Sprite {
    let addr: u16 = SPRITE_MEM_START + SPRITE_MEM_SIZE * num;

    let options = mem.get(addr + 3);

    Sprite {
        y: mem.get(addr) as i16 - 16,
        x: mem.get(addr + 1) as i16 - 8,
        tile: mem.get(addr + 3),
        priority: options & 0b1000000 != 0,
        yflip: options & 0b100000 != 0,
        xflip: options & 0b10000 != 0,
        palette: if options & 0b1000 != 0 {
            palettes.0
        } else {
            palettes.1
        },
    }
}

fn sprite_in_row(line: i16, sy: i16, height: u8) -> bool {
    sy < line && ((sy + height as i16) > line)
}

fn sprite_on_disp(sx: i16) -> bool {
    sx > -8 && sx <= (GB_HSIZE - 8) as i16
}

fn get_tile_row_data(mem: &Memory, tiledata: u16, tilenum: i32, ty: u16) -> (u8, u8) {
    const TILE_SIZE: i32 = 16;
    let signedtiledata: i32 = tiledata as u32 as i32;
    let tilestart = (signedtiledata + tilenum * TILE_SIZE) as u16;

    let tilerow = tilestart + (ty as u16 * 2);

    let rowbyte1 = mem.get(tilerow);
    let rowbyte2 = mem.get(tilerow + 1);

    (rowbyte1, rowbyte2)
}

// returns 2 bit colour
fn get_tile_colour(tilerow: (u8, u8), tx: u8, palette: u8) -> u8 {
    let (byte1, byte2) = tilerow;

    let bit = 0b1 << (7 - tx);

    let pixel = ((byte1 & bit) >> (7 - tx)) | (((byte2 & bit) >> (7 - tx)) << 1);

    apply_palette(pixel, palette)
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

fn set_pixel(rgba: &mut [u8], start: usize, colour: u8) {
    rgba[start] = colour;
    rgba[start + 1] = colour;
    rgba[start + 2] = colour;
    rgba[start + 3] = 255;
}

fn select_tilemap(bit: bool) -> u16 {
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

fn get_sprite_size(lcdc: u8) -> u8 {
    if (lcdc & SPRITE_SIZE_BIT) != 0 {
        16
    } else {
        8
    }
}
