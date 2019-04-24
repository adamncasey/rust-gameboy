use crate::interrupt;
use crate::memory::Memory;

pub const GB_HSIZE: usize = 160;
pub const GB_VSIZE: usize = 144;

const LCD_ON_BIT: u8 = 1 << 7;
const WINDOW_TILEMAP_BIT: u8 = 1 << 6;
const WINDOW_DISP_BIT: u8 = 1 << 5;
const TILEDATA_BIT: u8 = 1 << 4;
const BG_TILEMAP_BIT: u8 = 1 << 3;
const SPRITE_SIZE_BIT: u8 = 1 << 2;
const SPRITE_DISP_BIT: u8 = 1 << 1;
const BG_DISP_BIT: u8 = 1;

#[derive(Debug)]
pub enum GpuMode {
    OAMRead,
    VRAMRead,
    HBlank,
    VBlank,
}

#[derive(Debug, Clone)]
pub struct GpuDebugTrace {
    sprites: u16,
    yflipped_sprite_lines: u16,
    xflipped_sprite_lines: u16,
}

impl GpuDebugTrace {
    fn new() -> GpuDebugTrace {
        GpuDebugTrace {
            sprites: 0,
            yflipped_sprite_lines: 0,
            xflipped_sprite_lines: 0,
        }
    }
}

pub struct Gpu {
    pub mode: GpuMode,
    pub mode_elapsed: u32,
    pub line: u8,
    pub screen_rgba: Vec<u8>,
    debug_current_frame: GpuDebugTrace,
    pub debug_last_frame: GpuDebugTrace,
    pub debug_lcd_pwr: bool,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            mode: GpuMode::HBlank,
            mode_elapsed: 0,
            line: 0,
            screen_rgba: vec![255; GB_VSIZE * GB_HSIZE * 4],
            debug_current_frame: GpuDebugTrace::new(),
            debug_last_frame: GpuDebugTrace::new(),
            debug_lcd_pwr: false,
        }
    }

    pub fn cycle(&mut self, mem: &mut Memory, elapsed: u8) {
        // TODO SLOW currently load this byte twice
        let lcdc: u8 = mem.get(0xFF40);

        if (lcdc & LCD_ON_BIT) == 0 {
            self.debug_lcd_pwr = false;
            let lcdstat: u8 = mem.get(0xFF41);

            let newlcdstat: u8 = lcdstat & 0xFC;

            mem.set(0xFF41, newlcdstat);

            return;
        }

        self.debug_lcd_pwr = true;

        let mut vblank = false;
        let mut newline = false;
        let mut newmode = false;

        self.mode_elapsed += u32::from(elapsed);
        match self.mode {
            GpuMode::OAMRead => {
                if self.mode_elapsed >= 80 {
                    newmode = true;
                    self.mode = GpuMode::VRAMRead;
                    self.mode_elapsed -= 80;
                }
            }
            GpuMode::VRAMRead => {
                if self.mode_elapsed >= 172 {
                    newmode = true;
                    self.mode = GpuMode::HBlank;
                    self.mode_elapsed -= 172;
                    self.draw_line(mem);
                }
            }
            GpuMode::HBlank => {
                if self.mode_elapsed >= 204 {
                    self.mode_elapsed -= 204;
                    self.line += 1;

                    newline = true;
                    newmode = true;
                    if self.line == 143 {
                        self.mode = GpuMode::VBlank;
                        vblank = true;
                    } else {
                        self.mode = GpuMode::OAMRead;
                    }
                }
            }
            GpuMode::VBlank => {
                if self.mode_elapsed >= 456 {
                    self.line += 1;
                    self.mode_elapsed -= 456;

                    if self.line > 153 {
                        self.debug_last_frame = self.debug_current_frame.clone();
                        self.debug_current_frame = GpuDebugTrace::new();
                        // Blank screen for re-writing
                        self.screen_rgba.resize(GB_VSIZE * GB_HSIZE * 4, 255);
                        self.mode = GpuMode::OAMRead;
                        self.line = 0;
                    }
                }
            }
        };

        ////println!("GPU State: {:?} {:?} {:?}", self.mode, self.line, self.mode_elapsed);

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
            interrupt::set_interrupt(interrupt::Interrupt::VBlank, mem);
        }

        if self.lcd_status_interrupt(mem, newlcdstat, newmode, newline) {
            //println!("lcd_status_interrupt");
            interrupt::set_interrupt(interrupt::Interrupt::LcdStat, mem);
        }
    }

    fn lcd_status_interrupt(
        &self,
        mem: &Memory,
        lcdstat: u8,
        newmode: bool,
        newline: bool,
    ) -> bool {
        if newline && lcdstat & 0b10_0000 != 0 {
            if lcdstat & 0b100 != 0 {
                if self.line == mem.get(0xFF45) {
                    return true;
                }
            } else if self.line != mem.get(0xFF45) {
                return true;
            }
        }

        if newmode {
            // TODO figure out mode related interrupt
        }

        false
    }

    fn draw_line(&mut self, mem: &Memory) {
        let lcdc: u8 = mem.get(0xFF40);
        let tiledataselect = (lcdc & TILEDATA_BIT) != 0;
        let tiles = tiles_start(tiledataselect);

        let bg_win_colours: u8 = mem.get(0xFF47);

        if lcdc & BG_DISP_BIT != 0 {
            let tilemap = select_tilemap((lcdc & BG_TILEMAP_BIT) != 0);
            draw_background(
                self.line,
                mem,
                bg_win_colours,
                tiles,
                tiledataselect,
                tilemap,
                &mut self.screen_rgba,
            );
        }

        if lcdc & WINDOW_DISP_BIT != 0 {
            let _tilemap = select_tilemap(lcdc & WINDOW_TILEMAP_BIT != 0);
            // TODO draw_window
        }

        if lcdc & SPRITE_DISP_BIT != 0 {
            let sprite_height: u8 = get_sprite_size(lcdc);
            draw_sprites(
                self.line,
                mem,
                sprite_height,
                0x8000,
                bg_win_colours,
                &mut self.screen_rgba,
                &mut self.debug_current_frame,
            );
        } else {
            ////println!("Sprites disabled {:X} {}", lcdc, lcdc & SPRITE_DISP_BIT);
        }
    }
}

fn draw_background(
    line: u8,
    mem: &Memory,
    bgp: u8,
    tiledata: u16,
    tiledataselect: bool,
    tilemap: u16,
    rgba: &mut [u8],
) {
    let scy: u8 = mem.get(0xFF42);
    let bgy: u16 = u16::from(line.wrapping_add(scy));
    let vtile = bgy / 8;

    if vtile >= 32 {
        //println!("reached vend of tile {} {}", vtile, line);
        return;
    }

    let ty: u16 = bgy % 8;

    let scx: u8 = mem.get(0xFF43);

    for i in 0..=GB_HSIZE {
        let bgx = ((i as u16) + u16::from(scx)) % 256;
        let htile = bgx / 8;
        if htile >= 32 {
            //println!("reached hend of tile {} {} {}", htile, line, i);
            return;
        }

        let tx: u8 = (bgx % 8) as u8;

        let tilenumtemp: u8 = mem.get(tilemap + vtile * 32 + htile);

        let tilenum: i32 = if !tiledataselect {
            i32::from(tilenumtemp as i8)
        } else {
            i32::from(u16::from(tilenumtemp))
        };

        // TODO draw all eight pixels at once.
        let tilerow = get_tile_row_data(mem, tiledata, tilenum, ty);
        let colour = get_tile_colour(tilerow, tx);
        let pixel = apply_palette(colour, bgp);

        let rgba_start = ((line as usize) * GB_HSIZE + i as usize) * 4;
        set_pixel(rgba, rgba_start, pixel);
    }
}

fn draw_sprites(
    line: u8,
    mem: &Memory,
    sprite_height: u8,
    tiledata: u16,
    bgp: u8,
    rgba: &mut [u8],
    debug: &mut GpuDebugTrace,
) {
    let palettes = (mem.get(0xFF48), mem.get(0xFF49));
    let bgcolouroverdraw = apply_palette(0, bgp);

    let mut sprites_drawn = 0;
    // for each sprite
    for i in 0..40 {
        if sprites_drawn >= 10 {
            return;
        }

        let s = load_sprite(mem, i, palettes);

        if !sprite_in_row(line, s.y, sprite_height) || !sprite_on_disp(s.x) {
            ////println!("Skipping sprite y line{} s{} y{} x{}", line, i, s.y, s.x);
            continue;
        }

        // look up tile pixel data
        let mut ty: u8 = ((s.y - u16::from(line) as i16).abs() % 8) as u8;

        if s.yflip {
            debug.yflipped_sprite_lines += 1;
            ty = sprite_height - ty;
        }

        let mut drawn = false;

        // TODO xflip:
        for px in 0..8 {
            let x = s.x + px;
            if x < 0 || x > GB_HSIZE as i16 {
                ////println!("Skipping sprite x {} {}", x, tx);
                continue;
            }
            drawn = true;
            let rgba_start = (line as usize * GB_HSIZE + x as usize) * 4;

            // Is priority bit set or is the bg value zero?
            if !s.priority && rgba[rgba_start] != bgcolouroverdraw {
                ////println!("Not drawing pixel due to priority / bg colour {}", rgba[rgba_start]);
                continue;
            }
            // draw pixel
            let tilerow = get_tile_row_data(mem, tiledata, u32::from(s.tile) as i32, u16::from(ty));

            let tx = if s.xflip {
                debug.xflipped_sprite_lines += 1;
                8 - px
            } else {
                px
            };

            let colour = get_tile_colour(tilerow, tx as u8);
            let pixel = apply_palette(colour, s.palette);

            // Is this pixel transparent?
            if colour != 0 {
                ////println!("Drawn pixel {:X} {}", rgba_start, pixel);
                set_pixel(rgba, rgba_start, pixel);
            } else {
                ////println!("Skipped pixel {}", colour);
            }
        }
        if drawn {
            sprites_drawn += 1;
        }
    }

    debug.sprites += sprites_drawn;
}

#[derive(Debug, Copy, Clone)]
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
        y: u16::from(mem.get(addr)) as i16 - 16,
        x: u16::from(mem.get(addr + 1)) as i16 - 8,
        tile: mem.get(addr + 2),
        priority: options & 0b100_0000 == 0,
        yflip: options & 0b10_0000 != 0,
        xflip: options & 0b1_0000 != 0,
        palette: if options & 0b1000 != 0 {
            palettes.1
        } else {
            palettes.0
        },
    }
}

fn sprite_in_row(line: u8, sy: i16, height: u8) -> bool {
    let line = i32::from(line);
    let sy = i32::from(sy);

    sy < line && (sy + i32::from(height)) > line
}

fn sprite_on_disp(sx: i16) -> bool {
    sx > -8 && sx <= GB_HSIZE as i16
}

fn get_tile_row_data(mem: &Memory, tiledata: u16, tilenum: i32, ty: u16) -> (u8, u8) {
    const TILE_SIZE: i32 = 16;
    let signedtiledata: i32 = u32::from(tiledata) as i32;
    let tilestart = (signedtiledata + tilenum * TILE_SIZE) as u16;

    let tilerow = tilestart + (ty as u16 * 2);

    let rowbyte1 = mem.get(tilerow);
    let rowbyte2 = mem.get(tilerow + 1);

    (rowbyte1, rowbyte2)
}

// returns 2 bit colour
fn get_tile_colour(tilerow: (u8, u8), tx: u8) -> u8 {
    let (byte1, byte2) = tilerow;

    let bit = 0b1 << (7 - tx);

    ((byte1 & bit) >> (7 - tx)) | (((byte2 & bit) >> (7 - tx)) << 1)
}

fn apply_palette(colour: u8, pal: u8) -> u8 {
    match colour {
        3 => get_colour((pal & 0b1100_0000) >> 6),
        2 => get_colour((pal & 0b0011_0000) >> 4),
        1 => get_colour((pal & 0b0000_1100) >> 2),
        0 => get_colour(pal & 0b0000_0011),
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sprite_in_row() {
        assert_eq!(true, sprite_in_row(0, -8, 16));
        assert_eq!(false, sprite_in_row(0, -8, 8));

        assert_eq!(false, sprite_in_row(0, (65535 as i32) as i16, 8));
    }
}
