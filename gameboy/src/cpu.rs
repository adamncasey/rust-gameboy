use crate::instruction::Instruction;
use crate::interrupt;
use crate::memory::Memory;

#[derive(PartialEq, Eq, Debug)]
pub struct Cpu {
    pub pc: u16,
    pub sp: u16,

    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,

    pub interrupts: bool,

    pub jumped: bool,
    pub halted: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Cpu16Register {
    SP,
    BC,
    DE,
    HL,
    AF,
}

#[derive(Debug, Clone, Copy)]
pub enum CpuRegister {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            pc: 0x0100,
            sp: 0xFFFE,
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: 0xB0,
            h: 0x01,
            l: 0x4D,
            interrupts: true, // TODO start value?
            jumped: false,
            halted: false,
        }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu::default()
    }

    pub fn cycle(&mut self, mem: &mut Memory, debug: bool) -> u8 {
        if self.pc == 0xC303 {
            println!("Got here");
        }

        if self.halted {
            return 8;
        }

        let instr = Instruction::read(mem, self.pc);

        if debug {
            println!("Instruction: {:?}", &instr);
        }

        let cycles = instr.execute(self, mem);

        // If we jumped we shouldn't skip over current instr
        if !self.jumped {
            self.pc += Instruction::mem_size(&instr);
        }

        self.jumped = false;

        cycles
    }

    pub fn interrupt(&mut self, mem: &mut Memory, int: interrupt::Interrupt) -> bool {
        self.halted = false;

        if !self.interrupts {
            //println!("Interrupt disabled {:?}", int);
            return false;
        }

        //println!("Interrupt enabled {:?}", int);

        let targetpc = match int {
            interrupt::Interrupt::VBlank => {
                0x0040
            }
            interrupt::Interrupt::LcdStat => {
                0x0048
            }
            interrupt::Interrupt::Timer => {
                0x0050
            }
            interrupt::Interrupt::Joypad => {
                0x0060
            }
        };

        // Push current pc onto stack, and reset pc to targetpc
        self.sp -= 2;
        mem.set16(self.sp, self.pc);

        self.pc = targetpc;

        interrupt::reset_interrupt(int, mem);

        // Further interrupts are disabled until re-enabled (RETI / EI)
        self.disable_interrupts();

        int == interrupt::Interrupt::VBlank
    }

    pub fn set(&mut self, reg: CpuRegister, val: u8) {
        match reg {
            CpuRegister::A => self.a = val,
            CpuRegister::B => self.b = val,
            CpuRegister::C => self.c = val,
            CpuRegister::D => self.d = val,
            CpuRegister::E => self.e = val,
            CpuRegister::H => self.h = val,
            CpuRegister::L => self.l = val,
        }
    }

    pub fn get(&self, reg: CpuRegister) -> u8 {
        match reg {
            CpuRegister::A => self.a,
            CpuRegister::B => self.b,
            CpuRegister::C => self.c,
            CpuRegister::D => self.d,
            CpuRegister::E => self.e,
            CpuRegister::H => self.h,
            CpuRegister::L => self.l,
        }
    }

    pub fn set16(&mut self, reg: Cpu16Register, val: u16) {
        let low: u8 = (val & 0x00FF) as u8;
        let high: u8 = ((val & 0xFF00) >> 8) as u8;

        match reg {
            Cpu16Register::SP => {
                self.sp = val;
            }
            Cpu16Register::BC => {
                self.b = high;
                self.c = low;
            }
            Cpu16Register::DE => {
                self.d = high;
                self.e = low;
            }
            Cpu16Register::HL => {
                self.h = high;
                self.l = low;
            }
            Cpu16Register::AF => {
                self.a = high;
                self.f = low & 0xF0;
            }
        }
    }

    pub fn get16(&mut self, reg: Cpu16Register) -> u16 {
        let low: u8; // = (val & 0x00FF) as u8;
        let high: u16; // = ((val & 0xFF00) >> 8) as u8;

        match reg {
            Cpu16Register::SP => {
                return self.sp;
            }
            Cpu16Register::BC => {
                high = u16::from(self.b);
                low = self.c;
            }
            Cpu16Register::DE => {
                high = u16::from(self.d);
                low = self.e;
            }
            Cpu16Register::HL => {
                high = u16::from(self.h);
                low = self.l;
            }
            Cpu16Register::AF => {
                high = u16::from(self.a);
                low = self.f;
            }
        };

        (high << 8) + u16::from(low)
    }

    pub fn set_flags(&mut self, z: bool, n: bool, h: bool, c: bool) {
        self.f = (z as u8) << 7 | (n as u8) << 6 | (h as u8) << 5 | (c as u8) << 4;
    }

    pub fn c_flag(&self) -> bool {
        (self.f & (1 << 4)) != 0
    }
    pub fn h_flag(&self) -> bool {
        (self.f & (1 << 5)) != 0
    }
    pub fn n_flag(&self) -> bool {
        (self.f & (1 << 6)) != 0
    }
    pub fn z_flag(&self) -> bool {
        (self.f & (1 << 7)) != 0
    }

    pub fn jump(&mut self, addr: u16) {
        self.pc = addr;
        self.jumped = true;
    }

    pub fn rjump(&mut self, offset: i8) {
        self.pc = (i32::from(self.pc) + i32::from(offset)) as u16;
        self.jumped = true;
    }

    pub fn ret(&mut self, mem: &Memory) {
        let newpc = mem.get16(self.sp);

        self.sp = self.sp.wrapping_add(2);
        self.jump(newpc);
    }

    pub fn enable_interrupts(&mut self) {
        self.interrupts = true;
    }

    pub fn disable_interrupts(&mut self) {
        self.interrupts = false;
    }

    pub fn halt(&mut self) {
        if !self.interrupts {
            // TODO is this valid?
            //panic!("Halted with interrupts disabled");
            println!("Halt w/ interrupts disabled");
            return;
        }

        self.halted = true;
    }

    pub fn print_state(&self) -> String {
        format!(
            "pc {:4X} sp {:2X} a {:2X} b {:2X} c {:2X} d {:2X} e {:2X} f {:2X} h {:2X} l {:2X}",
            self.pc, self.sp, self.a, self.b, self.c, self.d, self.e, self.f, self.h, self.l
        )
    }
}
