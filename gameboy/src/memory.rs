use crate::input::Input;
use crate::interrupt::{set_interrupt, Interrupt};
use crate::rom::Cartridge;
use crate::timer::Timer;

pub struct Memory {
    cartridge: Cartridge,
    vram: Vec<u8>,
    ram: Vec<u8>,
    sprite: Vec<u8>,
    io: Vec<u8>,
    highram: Vec<u8>,

    // All unused memory is forwarded to the same byte
    // TODO reads shouldn't be affected by writes
    unused: u8,
    // Input & Timer are always interfaced via the MMU
    input: Input,
    timer: Timer,

    serial_buf: Vec<u8>,
}

const VRAM_SIZE: usize = 8 * 1024;
const RAM_SIZE: usize = 8 * 1024;
const SPRITE_SIZE: usize = 160;
const IO_SIZE: usize = 76;
const HIGHRAM_SIZE: usize = 128;

const MAX_SERIAL_BUF_LEN: usize = 50000;

impl Memory {
    pub fn new(cartridge: Cartridge) -> Memory {
        // Move rom contents into Memory
        let mut mem = Memory {
            cartridge,
            vram: vec![0; VRAM_SIZE],
            ram: vec![0; RAM_SIZE],
            sprite: vec![0; SPRITE_SIZE],
            io: vec![0; IO_SIZE],
            highram: vec![0; HIGHRAM_SIZE],

            unused: 0,
            input: Input::new(),
            timer: Timer::new(),

            serial_buf: Vec::new(),
        };

        mem.set(0xFF40, 0x91);
        mem.set(0xFF47, 0xFC);
        mem.set(0xFF48, 0xFF);
        mem.set(0xFF49, 0xFF);

        mem
    }

    pub fn get(&self, addr: u16) -> u8 {
        match addr {
            0xFF00 => self.input.value(),
            0xFF4D => 0xFF,
            0xFF04..=0xFF07 => self.timer.read(addr),
            _ => *self.mmu(addr),
        }
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

    fn mmu(&self, addr: u16) -> &u8 {
        match addr {
            0x0000..=0x7FFF => self.cartridge.mbc(addr),
            0x8000..=0x9FFF => &self.vram[(addr - 0x8000) as usize],
            0xA000..=0xBFFF => self.cartridge.mbc(addr),
            0xC000..=0xDFFF => &self.ram[(addr - 0xC000) as usize],
            0xE000..=0xFDFF => &self.ram[(addr - 0xE000) as usize],
            0xFE00..=0xFE9F => &self.sprite[(addr - 0xFE00) as usize],
            0xFEA0..=0xFEFF => &self.unused,
            0xFF00..=0xFF4B => &self.io[(addr - 0xFF00) as usize],
            0xFF4C..=0xFF7F => &self.unused,
            0xFF80..=0xFFFF => &self.highram[(addr - 0xFF80) as usize],
        }
    }

    pub fn set(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7FFF => self.cartridge.mbc_write(addr, val),
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = val,
            0xA000..=0xBFFF => self.cartridge.mbc_write(addr, val),
            0xC000..=0xDFFF => self.ram[(addr - 0xC000) as usize] = val,
            0xE000..=0xFDFF => self.ram[(addr - 0xE000) as usize] = val,
            0xFE00..=0xFE9F => self.sprite[(addr - 0xFE00) as usize] = val,
            0xFEA0..=0xFEFF => {}, // unused
            0xFF00 => self.input.update(val),
            0xFF01 => {
                self.serial_buf.push(val);

                if val as char == '\n' {
                    println!("{}", String::from_utf8_lossy(&self.serial_buf));
                }

                if self.serial_buf.len() > MAX_SERIAL_BUF_LEN {
                    self.serial_buf.clear();
                }
            }
            0xFF02..=0xFF03 => {}, // unimplemented
            0xFF04..=0xFF06 => self.timer.write(addr, val),
            0xFF00..=0xFF45 | 0xFF47..=0xFF4B => self.io[(addr - 0xFF00) as usize] = val,
            0xFF46 => {
                // OAM Write
                // TODO SLOW This could be a lot faster
                let source: u16 = u16::from(val) << 8;
                let target: u16 = 0xFE00;
                for i in 0..160 {
                    let val: u8 = self.get(source + i);
                    self.set(target + i, val);
                }
            },
            0xFF4C => {},
            0xFF4D => {
                println!("Speed");
            }
            0xFF4E..=0xFF7F => {},
            0xFF80..=0xFFFF => self.highram[(addr - 0xFF80) as usize] = val,
        }
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn tick_timer(&mut self, cycles: u8) {
        if self.timer.tick(cycles) {
            set_interrupt(Interrupt::Timer, self);
        }
    }

    pub fn clone_bytes(&self, start: u16, len: u16) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(usize::from(len));

        let mut addr = start;
        for _ in 0..len {
            bytes.push(self.get(addr));
            addr += 1;
        }

        bytes
    }

    pub fn serial_buffer(&self) -> &[u8] {
        &self.serial_buf
    }
}
