use cpu::{Cpu, Cpu16Register, CpuRegister};
use memory::Memory;
use math;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    LDI16 { val: u16, reg: Cpu16Register },
    LDI8 { val: u8, reg: CpuRegister },
    LDR8 { src: CpuRegister, dst: CpuRegister },
    LDA8 {
        src_addr: Cpu16Register,
        dst: CpuRegister,
    },
    JP { addr: u16 },
    CALL { addr: u16 },
    RST18,
    STA8 {
        dst_addr: Cpu16Register,
        src: CpuRegister,
    },
    STH { addr: u8, reg: CpuRegister },
    SUBR { reg: CpuRegister },
    SUBA { reg_addr: Cpu16Register },
    SBCR { reg: CpuRegister },
    SBCA { reg_addr: Cpu16Register },
    INC { reg: CpuRegister },
    DEC { reg: CpuRegister },
    CPL,
}

impl Instruction {
    pub fn read(mem: &Memory, addr: u16) -> Instruction {
        let opcode = mem.get(addr);

        if opcode == 0x3B {
            return read_extended_opcode(mem.get(addr + 1), addr + 2, mem);
        }

        return read_opcode(opcode, addr + 1, mem);
    }

    pub fn mem_size(inst: &Instruction) -> u16 {
        match *inst {
            Instruction::Noop => 1,
            Instruction::LDI16 { .. } => 3,
            Instruction::LDI8 { .. } => 2,
            Instruction::LDR8 { .. } => 1,
            Instruction::LDA8 { .. } => 1,
            Instruction::JP { .. } => 3,
            Instruction::CALL { .. } => 3,
            Instruction::RST18 => 1,
            Instruction::STA8 { .. } => 1,
            Instruction::STH { .. } => 2,
            Instruction::SUBR { .. } => 1,
            Instruction::SUBA { .. } => 1,
            Instruction::SBCR { .. } => 1,
            Instruction::SBCA { .. } => 1,
            Instruction::INC { .. } => 1,
            Instruction::DEC { .. } => 1,
            Instruction::CPL => 1,
        }
    }

    pub fn execute(&self, cpu: &mut Cpu, mem: &mut Memory) {
        // Execute based on opcode
        match *self {
            Instruction::Noop => (),
            Instruction::LDI16 { val, reg } => {
                cpu.set16(reg, val);
            }
            Instruction::LDI8 { val, reg } => {
                cpu.set(reg, val);
            }
            Instruction::LDR8 { src, dst } => {
                let val: u8 = cpu.get(src);
                cpu.set(dst, val);
            }
            Instruction::LDA8 { src_addr, dst } => {
                let addr: u16 = cpu.get16(src_addr);
                cpu.set(dst, mem.get(addr));
            }
            Instruction::JP { addr } => cpu.pc = addr,
            Instruction::CALL { addr } => {
                mem.set16(cpu.sp, cpu.pc + Instruction::mem_size(self));
                cpu.sp -= 2;
                cpu.pc = addr;
            }
            Instruction::RST18 => {
                // Store next pc on stack & jump to 0x0018
                mem.set16(cpu.sp, cpu.pc + Instruction::mem_size(self));
                cpu.sp -= 2;
                cpu.pc = 0x0018;
            }
            Instruction::STA8 { dst_addr, src } => {
                let dst: u16 = cpu.get16(dst_addr);
                mem.set(dst, cpu.get(src));
            }
            Instruction::STH { addr, reg } => mem.set((0xFF00 + addr as u16) as u16, cpu.get(reg)),
            Instruction::SUBR { reg } => {
                let val: u8 = cpu.get(reg);
                math::subtract(cpu, val);
            }
            Instruction::SUBA { reg_addr } => {
                let val: u8 = mem.get(cpu.get16(reg_addr));
                math::subtract(cpu, val);
            }
            Instruction::SBCR { reg } => {
                let val: u8 = cpu.get(reg) + cpu.c_flag();
                math::subtract(cpu, val);
            }
            Instruction::SBCA { reg_addr } => {
                let val: u8 = mem.get(cpu.get16(reg_addr)) + cpu.c_flag();
                math::subtract(cpu, val);
            }
            Instruction::INC { reg } => {
                math::increment(cpu, reg);
            }
            Instruction::DEC { reg } => {
                math::decrement(cpu, reg);
            }
            Instruction::CPL => {
                math::complement(cpu);
            }
        }
    }
}

fn read_opcode(opcode: u8, argstart: u16, mem: &Memory) -> Instruction {
    match opcode {
        0x00 => Instruction::Noop,
        0x01 => Instruction::LDI16 {
            val: mem.get16(argstart),
            reg: Cpu16Register::BC,
        },
        0x0C => Instruction::INC {
            reg: CpuRegister::C,
        },
        0x0D => Instruction::DEC {
            reg: CpuRegister::C,
        },
        0x0E => Instruction::LDI8 {
            val: mem.get(argstart),
            reg: CpuRegister::C,
        },
        0x11 => Instruction::LDI16 {
            val: mem.get16(argstart),
            reg: Cpu16Register::DE,
        },
        0x1C => Instruction::INC {
            reg: CpuRegister::E,
        },
        0x1D => Instruction::DEC {
            reg: CpuRegister::E,
        },
        0x1E => Instruction::LDI8 {
            val: mem.get(argstart),
            reg: CpuRegister::E,
        },
        0x21 => Instruction::LDI16 {
            val: mem.get16(argstart),
            reg: Cpu16Register::HL,
        },
        0x2C => Instruction::INC {
            reg: CpuRegister::L,
        },
        0x2D => Instruction::DEC {
            reg: CpuRegister::L,
        },
        0x2E => Instruction::LDI8 {
            val: mem.get(argstart),
            reg: CpuRegister::L,
        },
        0x2F => Instruction::CPL,
        0x31 => Instruction::LDI16 {
            val: mem.get16(argstart),
            reg: Cpu16Register::SP,
        },
        0x3C => Instruction::INC {
            reg: CpuRegister::A,
        },
        0x3D => Instruction::DEC {
            reg: CpuRegister::A,
        },
        0x3E => Instruction::LDI8 {
            val: mem.get(argstart),
            reg: CpuRegister::A,
        },
        0x40 => Instruction::LDR8 {
            src: CpuRegister::B,
            dst: CpuRegister::B,
        },
        0x41 => Instruction::LDR8 {
            src: CpuRegister::C,
            dst: CpuRegister::B,
        },
        0x42 => Instruction::LDR8 {
            src: CpuRegister::D,
            dst: CpuRegister::B,
        },
        0x43 => Instruction::LDR8 {
            src: CpuRegister::E,
            dst: CpuRegister::B,
        },
        0x44 => Instruction::LDR8 {
            src: CpuRegister::H,
            dst: CpuRegister::B,
        },
        0x45 => Instruction::LDR8 {
            src: CpuRegister::L,
            dst: CpuRegister::B,
        },
        0x46 => Instruction::LDA8 {
            src_addr: Cpu16Register::HL,
            dst: CpuRegister::B,
        },
        0x47 => Instruction::LDR8 {
            src: CpuRegister::A,
            dst: CpuRegister::B,
        },
        0x48 => Instruction::LDR8 {
            src: CpuRegister::B,
            dst: CpuRegister::C,
        },
        0x49 => Instruction::LDR8 {
            src: CpuRegister::C,
            dst: CpuRegister::C,
        },
        0x4A => Instruction::LDR8 {
            src: CpuRegister::D,
            dst: CpuRegister::C,
        },
        0x4B => Instruction::LDR8 {
            src: CpuRegister::E,
            dst: CpuRegister::C,
        },
        0x4C => Instruction::LDR8 {
            src: CpuRegister::H,
            dst: CpuRegister::C,
        },
        0x4D => Instruction::LDR8 {
            src: CpuRegister::L,
            dst: CpuRegister::C,
        },
        0x4E => Instruction::LDA8 {
            src_addr: Cpu16Register::HL,
            dst: CpuRegister::C,
        },
        0x4F => Instruction::LDR8 {
            src: CpuRegister::A,
            dst: CpuRegister::C,
        },
        0x50 => Instruction::LDR8 {
            src: CpuRegister::B,
            dst: CpuRegister::D,
        },
        0x51 => Instruction::LDR8 {
            src: CpuRegister::C,
            dst: CpuRegister::D,
        },
        0x52 => Instruction::LDR8 {
            src: CpuRegister::D,
            dst: CpuRegister::D,
        },
        0x53 => Instruction::LDR8 {
            src: CpuRegister::E,
            dst: CpuRegister::D,
        },
        0x54 => Instruction::LDR8 {
            src: CpuRegister::H,
            dst: CpuRegister::D,
        },
        0x55 => Instruction::LDR8 {
            src: CpuRegister::L,
            dst: CpuRegister::D,
        },
        0x56 => Instruction::LDA8 {
            src_addr: Cpu16Register::HL,
            dst: CpuRegister::D,
        },
        0x57 => Instruction::LDR8 {
            src: CpuRegister::A,
            dst: CpuRegister::D,
        },
        0x58 => Instruction::LDR8 {
            src: CpuRegister::B,
            dst: CpuRegister::E,
        },
        0x59 => Instruction::LDR8 {
            src: CpuRegister::C,
            dst: CpuRegister::E,
        },
        0x5A => Instruction::LDR8 {
            src: CpuRegister::D,
            dst: CpuRegister::E,
        },
        0x5B => Instruction::LDR8 {
            src: CpuRegister::E,
            dst: CpuRegister::E,
        },
        0x5C => Instruction::LDR8 {
            src: CpuRegister::H,
            dst: CpuRegister::E,
        },
        0x5D => Instruction::LDR8 {
            src: CpuRegister::L,
            dst: CpuRegister::E,
        },
        0x5E => Instruction::LDA8 {
            src_addr: Cpu16Register::HL,
            dst: CpuRegister::E,
        },
        0x5F => Instruction::LDR8 {
            src: CpuRegister::A,
            dst: CpuRegister::E,
        },
        0x60 => Instruction::LDR8 {
            src: CpuRegister::B,
            dst: CpuRegister::H,
        },
        0x61 => Instruction::LDR8 {
            src: CpuRegister::C,
            dst: CpuRegister::H,
        },
        0x62 => Instruction::LDR8 {
            src: CpuRegister::D,
            dst: CpuRegister::H,
        },
        0x63 => Instruction::LDR8 {
            src: CpuRegister::E,
            dst: CpuRegister::H,
        },
        0x64 => Instruction::LDR8 {
            src: CpuRegister::H,
            dst: CpuRegister::H,
        },
        0x65 => Instruction::LDR8 {
            src: CpuRegister::L,
            dst: CpuRegister::H,
        },
        0x66 => Instruction::LDA8 {
            src_addr: Cpu16Register::HL,
            dst: CpuRegister::E,
        },
        0x67 => Instruction::LDR8 {
            src: CpuRegister::A,
            dst: CpuRegister::E,
        },
        0x68 => Instruction::LDR8 {
            src: CpuRegister::B,
            dst: CpuRegister::L,
        },
        0x69 => Instruction::LDR8 {
            src: CpuRegister::C,
            dst: CpuRegister::L,
        },
        0x6A => Instruction::LDR8 {
            src: CpuRegister::D,
            dst: CpuRegister::L,
        },
        0x6B => Instruction::LDR8 {
            src: CpuRegister::E,
            dst: CpuRegister::L,
        },
        0x6C => Instruction::LDR8 {
            src: CpuRegister::H,
            dst: CpuRegister::L,
        },
        0x6D => Instruction::LDR8 {
            src: CpuRegister::L,
            dst: CpuRegister::L,
        },
        0x6E => Instruction::LDA8 {
            src_addr: Cpu16Register::HL,
            dst: CpuRegister::L,
        },
        0x6F => Instruction::LDR8 {
            src: CpuRegister::A,
            dst: CpuRegister::L,
        },
        0x70 => Instruction::STA8 {
            dst_addr: Cpu16Register::HL,
            src: CpuRegister::B,
        },
        0x71 => Instruction::STA8 {
            dst_addr: Cpu16Register::HL,
            src: CpuRegister::C,
        },
        0x72 => Instruction::STA8 {
            dst_addr: Cpu16Register::HL,
            src: CpuRegister::D,
        },
        0x73 => Instruction::STA8 {
            dst_addr: Cpu16Register::HL,
            src: CpuRegister::E,
        },
        0x74 => Instruction::STA8 {
            dst_addr: Cpu16Register::HL,
            src: CpuRegister::H,
        },
        0x75 => Instruction::STA8 {
            dst_addr: Cpu16Register::HL,
            src: CpuRegister::L,
        },
        // 0x76 HALT
        0x77 => Instruction::STA8 {
            dst_addr: Cpu16Register::HL,
            src: CpuRegister::A,
        },
        0x78 => Instruction::LDR8 {
            src: CpuRegister::B,
            dst: CpuRegister::A,
        },
        0x79 => Instruction::LDR8 {
            src: CpuRegister::C,
            dst: CpuRegister::A,
        },
        0x7A => Instruction::LDR8 {
            src: CpuRegister::D,
            dst: CpuRegister::A,
        },
        0x7B => Instruction::LDR8 {
            src: CpuRegister::E,
            dst: CpuRegister::A,
        },
        0x7C => Instruction::LDR8 {
            src: CpuRegister::H,
            dst: CpuRegister::A,
        },
        0x7D => Instruction::LDR8 {
            src: CpuRegister::L,
            dst: CpuRegister::A,
        },
        0x7E => Instruction::LDA8 {
            src_addr: Cpu16Register::HL,
            dst: CpuRegister::A,
        },
        0x7F => Instruction::LDR8 {
            src: CpuRegister::A,
            dst: CpuRegister::A,
        },
        0x90 => Instruction::SUBR {
            reg: CpuRegister::B,
        },
        0x91 => Instruction::SUBR {
            reg: CpuRegister::C,
        },
        0x92 => Instruction::SUBR {
            reg: CpuRegister::D,
        },
        0x93 => Instruction::SUBR {
            reg: CpuRegister::E,
        },
        0x94 => Instruction::SUBR {
            reg: CpuRegister::H,
        },
        0x95 => Instruction::SUBR {
            reg: CpuRegister::L,
        },
        0x96 => Instruction::SUBA {
            reg_addr: Cpu16Register::HL,
        },
        0x97 => Instruction::SUBR {
            reg: CpuRegister::A,
        },
        0x98 => Instruction::SBCR {
            reg: CpuRegister::B,
        },
        0x99 => Instruction::SBCR {
            reg: CpuRegister::C,
        },
        0x9A => Instruction::SBCR {
            reg: CpuRegister::D,
        },
        0x9B => Instruction::SBCR {
            reg: CpuRegister::E,
        },
        0x9C => Instruction::SBCR {
            reg: CpuRegister::H,
        },
        0x9D => Instruction::SBCR {
            reg: CpuRegister::L,
        },
        0x9E => Instruction::SBCA {
            reg_addr: Cpu16Register::HL,
        },
        0x9F => Instruction::SBCR {
            reg: CpuRegister::A,
        },
        0xC3 => Instruction::JP {
            addr: mem.get16(argstart),
        },
        0xCD => Instruction::CALL {
            addr: mem.get16(argstart),
        },
        0xDF => Instruction::RST18,
        0xE0 => Instruction::STH {
            addr: mem.get(argstart),
            reg: CpuRegister::A,
        },
        _ => panic!("Unknown opcode {:2X}", opcode),
    }
}

fn read_extended_opcode(opcode: u8, _argstart: u16, _mem: &Memory) -> Instruction {
    match opcode {
        _ => panic!("Unknown extended opcode 0xC3{:2X}", opcode),
    }
}
