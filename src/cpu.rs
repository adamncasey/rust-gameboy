use crate::memory::Memory;
use crate::instruction::Instruction;

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

    interrupts: bool,

    jumped: bool,
    halted: bool,
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
#[derive(Debug, Clone, Copy)]
pub enum CpuInterrupt {
    VBlank,
    LCDStatus,
    Timer,
    Joypad,
    None,
}

const INT_VBLANK: u8 = 1;
const INT_LCDSTAT: u8 = 2;
const INT_TIMER: u8 = 4;
const INT_JOYPAD: u8 = 16;

impl Cpu {
    pub fn new() -> Cpu {
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

    pub fn cycle(&mut self, mem: &mut Memory, debug: bool) -> u8 {
        if self.halted {
            return 8;
        }

        let instr = Instruction::read(&mem, self.pc);

        if debug {
            println!(
                "--- Executing |{:X}|{:X}|{:X}| {:?}",
                mem.get(self.pc),
                mem.get(self.pc + 1),
                mem.get(self.pc + 2),
                instr
            );
        }

        let cycles = instr.execute(self, mem);

        // If we jumped we shouldn't skip over current instr
        if !self.jumped {
            self.pc += Instruction::mem_size(&instr);
        }

        self.jumped = false;

        return cycles;
    }

    pub fn interrupt(&mut self, mem: &mut Memory, active: CpuInterrupt) {
        self.halted = false;

        if !self.interrupts {
            //println!("Interrupts disabled {:?}", active);
            return;
        }

        let targetpc;
        let int;

        let enabled: u8 = mem.get(0xFFFF);

        match active {
            CpuInterrupt::VBlank if (enabled & INT_VBLANK) != 0 => {
                targetpc = 0x0040;
                int = INT_VBLANK;
            }
            CpuInterrupt::LCDStatus if (enabled & INT_LCDSTAT) != 0 => {
                targetpc = 0x0048;
                int = INT_LCDSTAT;
            }
            CpuInterrupt::Timer if (enabled & INT_TIMER) != 0 => {
                targetpc = 0x0050;
                int = INT_TIMER;
            }
            CpuInterrupt::Joypad if (enabled & INT_JOYPAD) != 0 => {
                targetpc = 0x0060;
                int = INT_JOYPAD;
            }
            CpuInterrupt::None => {
                return;
            }
            _ => {
                //println!("Interrupt skipped because {:?} enabled {:b}", active, enabled);
                return;
            }
        };

        /*println!(
            "Interrupt! {:?} State prior to interrupt: {}",
            active,
            self.print_state()
        );*/

        self.sp -= 2;
        mem.set16(self.sp, self.pc);

        self.pc = targetpc;
        mem.set(0xFF0F, int);

        // Further interrupts are disabled until re-enabled (RETI / EI)
        self.disable_interrupts();
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
                high = self.b as u16;
                low = self.c;
            }
            Cpu16Register::DE => {
                high = self.d as u16;
                low = self.e;
            }
            Cpu16Register::HL => {
                high = self.h as u16;
                low = self.l;
            }
            Cpu16Register::AF => {
                high = self.a as u16;
                low = self.f;
            }
        };

        return (high << 8) + (low as u16);
    }

    pub fn set_flags(&mut self, z: bool, n: bool, h: bool, c: bool) {
        self.f = (z as u8) << 7 | (n as u8) << 6 | (h as u8) << 5 | (c as u8) << 4;
    }

    pub fn c_flag(&self) -> u8 {
        (self.f & (1 << 4)) >> 4
    }
    pub fn z_flag(&self) -> bool {
        (self.f & (1 << 7)) != 0
    }

    pub fn jump(&mut self, addr: u16) {
        self.pc = addr;
        self.jumped = true;
    }

    pub fn rjump(&mut self, offset: i8) {
        self.pc = ((self.pc as i32) + (offset as i32)) as u16;
        self.jumped = true;
    }

    pub fn ret(&mut self, mem: &Memory) {
        let newpc = mem.get16(self.sp);

        self.sp = self.sp.wrapping_add(2);
        self.jump(newpc);
    }

    pub fn enable_interrupts(&mut self) {
        self.interrupts = true;
        //println!("Interrupts enabled");
    }

    pub fn disable_interrupts(&mut self) {
        self.interrupts = false;
        //println!("Interrupts disabled");
    }

    pub fn halt(&mut self) {
        self.halted = true;
        println!("HALT");
    }

    pub fn print_state(&self) -> String {
        format!(
            "pc {:X} sp {:X} a {:X} b {:X} c {:X} d {:X} e {:X} f {:X} h {:X} l {:X} ",
            self.pc,
            self.sp,
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.f,
            self.h,
            self.l
        )
    }
}
