use memory::Memory;
use instruction::Instruction;

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
}

#[derive(Debug, Clone, Copy)]
pub enum Cpu16Register {
    PC,
    SP,
    BC,
    DE,
    HL,
}

#[derive(Debug, Clone, Copy)]
pub enum CpuRegister {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0x0100,
            sp: 0xFFFE,
            a: 0x00,
            b: 0x00,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
        }
    }

    pub fn cycle(&mut self, mem: &mut Memory) {
        let instr = Instruction::read(&mem, self.pc);

        println!("Executing {:?}", instr);

        instr.execute(self, mem);

        self.pc += Instruction::mem_size(&instr);
    }

    pub fn set(&mut self, reg: CpuRegister, val: u8) {
        match reg {
            CpuRegister::A => self.a = val,
            CpuRegister::B => self.b = val,
            CpuRegister::C => self.c = val,
            CpuRegister::D => self.d = val,
            CpuRegister::E => self.e = val,
            CpuRegister::F => self.f = val,
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
            CpuRegister::F => self.f,
            CpuRegister::H => self.h,
            CpuRegister::L => self.l,
        }
    }

    pub fn set16(&mut self, reg: Cpu16Register, val: u16) {
        let low: u8 = (val & 0x00FF) as u8;
        let high: u8 = ((val & 0xFF00) >> 8) as u8;

        match reg {
            Cpu16Register::PC => {
                self.pc = val;
            }
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
                self.b = high;
                self.c = low;
            }
        }
    }

    pub fn get16(&mut self, reg: Cpu16Register) -> u16 {
        let low: u8; // = (val & 0x00FF) as u8;
        let high: u16; // = ((val & 0xFF00) >> 8) as u8;

        match reg {
            Cpu16Register::PC => {
                return self.pc;
            }
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
                high = self.b as u16;
                low = self.c;
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

    pub fn print_state(&self) {
        println!(
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
        );
    }
}
