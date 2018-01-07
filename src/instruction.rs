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
    LDHA { addr: u8 },
    LDHCA,
    LDD,
    LDI,
    LDAA { addr: u16 },
    JP { addr: u16 },
    JPNZ { addr: u16 },
    JR { offset: i8 },
    JRNZ { offset: i8 },
    JRZ { offset: i8 },
    CALL { addr: u16 },
    RET,
    RETNZ,
    RETZ,
    RETNC,
    RETC,
    RETI,
    RST { addr: u16 },
    STA8 {
        dst_addr: Cpu16Register,
        src: CpuRegister,
    },
    STI8 { dst_addr: Cpu16Register, val: u8 },
    STHA { addr: u8 },
    STHCA,
    STAA { addr: u16 },
    STD,
    STI,
    SUBR { reg: CpuRegister },
    SUBA { reg_addr: Cpu16Register },
    SUBI { val: u8 },
    SBCR { reg: CpuRegister },
    SBCA { reg_addr: Cpu16Register },
    ADDI { val: u8 },
    ADD16 { src: Cpu16Register },
    XORR { reg: CpuRegister },
    XORA,
    XORI { val: u8 },
    INC { reg: CpuRegister },
    DEC { reg: CpuRegister },
    INC16 { reg: Cpu16Register },
    DEC16 { reg: Cpu16Register },
    CPL,
    DI,
    EI,
    CMPR { reg: CpuRegister },
    CMPI { val: u8 },
    CMPA,
    ORR { reg: CpuRegister },
    ORA,
    ORI { val: u8 },
    ANDR { reg: CpuRegister },
    ANDA,
    ANDI { val: u8 },
    PUSH { reg: Cpu16Register },
    POP { reg: Cpu16Register },
}

impl Instruction {
    pub fn read(mem: &Memory, addr: u16) -> Instruction {
        let opcode: u8 = mem.get(addr);

        if opcode == 0xCB {
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
            Instruction::LDHA { .. } => 2,
            Instruction::LDHCA => 1,
            Instruction::LDD { .. } => 1,
            Instruction::LDI { .. } => 1,
            Instruction::LDAA { .. } => 3,
            Instruction::JP { .. } => 3,
            Instruction::JPNZ { .. } => 3,
            Instruction::JR { .. } => 2,
            Instruction::JRNZ { .. } => 2,
            Instruction::JRZ { .. } => 2,
            Instruction::CALL { .. } => 3,
            Instruction::RET => 1,
            Instruction::RETNZ => 1,
            Instruction::RETZ => 1,
            Instruction::RETNC => 1,
            Instruction::RETC => 1,
            Instruction::RETI => 1,
            Instruction::RST { .. } => 1,
            Instruction::STA8 { .. } => 1,
            Instruction::STI8 { .. } => 2,
            Instruction::STHA { .. } => 2,
            Instruction::STHCA => 1,
            Instruction::STAA { .. } => 3,
            Instruction::STD => 1,
            Instruction::STI => 1,
            Instruction::SUBR { .. } => 1,
            Instruction::SUBA { .. } => 1,
            Instruction::SUBI { .. } => 2,
            Instruction::SBCR { .. } => 1,
            Instruction::SBCA { .. } => 1,
            Instruction::ADDI { .. } => 2,
            Instruction::ADD16 { .. } => 1,
            Instruction::XORR { .. } => 1,
            Instruction::XORA => 1,
            Instruction::XORI { .. } => 2,
            Instruction::INC { .. } => 1,
            Instruction::DEC { .. } => 1,
            Instruction::INC16 { .. } => 1,
            Instruction::DEC16 { .. } => 1,
            Instruction::CPL => 1,
            Instruction::DI => 1,
            Instruction::EI => 1,
            Instruction::CMPR { .. } => 1,
            Instruction::CMPA => 1,
            Instruction::CMPI { .. } => 2,
            Instruction::ORR { .. } => 1,
            Instruction::ORA => 1,
            Instruction::ORI { .. } => 2,
            Instruction::ANDR { .. } => 1,
            Instruction::ANDA => 1,
            Instruction::ANDI { .. } => 2,
            Instruction::PUSH { .. } => 1,
            Instruction::POP { .. } => 1,
        }
    }

    pub fn execute(&self, cpu: &mut Cpu, mem: &mut Memory) -> u8 {
        let cycles: u8;
        // Execute based on opcode
        match *self {
            Instruction::Noop => cycles = 4,
            Instruction::LDI16 { val, reg } => {
                cpu.set16(reg, val);
                cycles = 12;
            }
            Instruction::LDI8 { val, reg } => {
                cpu.set(reg, val);
                cycles = 8;
            }
            Instruction::LDR8 { src, dst } => {
                let val: u8 = cpu.get(src);
                cpu.set(dst, val);
                cycles = 4;
            }
            Instruction::LDA8 { src_addr, dst } => {
                let addr: u16 = cpu.get16(src_addr);
                cpu.set(dst, mem.get(addr));
                cycles = 8;
            }
            Instruction::LDHA { addr } => {
                cpu.set(CpuRegister::A, mem.get(0xFF00 + (addr as u16)));
                cycles = 12;
            }
            Instruction::LDHCA => {
                let addr: u16 = cpu.get(CpuRegister::C) as u16;
                cpu.set(CpuRegister::A, mem.get(0xFF00 + addr));
                cycles = 8;
            }
            Instruction::LDD => {
                let addr: u16 = cpu.get16(Cpu16Register::HL);
                cpu.set(CpuRegister::A, mem.get(addr));
                cpu.set16(Cpu16Register::HL, addr - 1);
                cycles = 8;
            }
            Instruction::LDI => {
                let addr: u16 = cpu.get16(Cpu16Register::HL);
                cpu.set(CpuRegister::A, mem.get(addr));
                cpu.set16(Cpu16Register::HL, addr + 1);
                cycles = 8;
            }
            Instruction::LDAA { addr } => {
                cpu.set(CpuRegister::A, mem.get(addr));
                cycles = 16;
            }
            Instruction::JP { addr } => {
                cpu.jump(addr);
                cycles = 16;
            },
            Instruction::JPNZ { addr } => if !cpu.z_flag() {
                cpu.jump(addr);
                cycles = 16;
            } else {
                cycles = 12;
            },
            Instruction::JR { offset } => {
                cpu.rjump(offset + 2);
                cycles = 12;
            }
            Instruction::JRNZ { offset } => if !cpu.z_flag() {
                cpu.rjump(offset + 2);
                cycles = 12;
            } else {
                cycles = 8;
            },
            Instruction::JRZ { offset } => if cpu.z_flag() {
                cpu.rjump(offset + 2);
                cycles = 12;
            } else {
                cycles = 8;
            },
            Instruction::CALL { addr } => {
                mem.set16(cpu.sp, cpu.pc + Instruction::mem_size(self));
                cpu.sp -= 2;
                cpu.jump(addr);
                cycles = 24;
            }
            Instruction::RET => {
                cpu.ret(mem);
                cycles = 16;
            }
            Instruction::RETNZ => if !cpu.z_flag() {
                cpu.ret(mem);
                cycles = 20;
            } else {
                cycles = 8;
            },
            Instruction::RETZ => if cpu.z_flag() {
                cpu.ret(mem);
                cycles = 20;
            } else {
                cycles = 8;
            },
            Instruction::RETNC => if cpu.c_flag() == 1 {
                cpu.ret(mem);
                cycles = 20;
            } else {
                cycles = 8;
            },
            Instruction::RETC => if cpu.c_flag() == 0 {
                cpu.ret(mem);
                cycles = 20;
            } else {
                cycles = 8;
            },
            Instruction::RETI => {
                cpu.ret(mem);
                cpu.enable_interrupts();
                cycles = 16;
            }
            Instruction::RST { addr } => {
                // Store next pc on stack & jump to addr
                mem.set16(cpu.sp, cpu.pc + Instruction::mem_size(self));
                cpu.sp -= 2;
                cpu.jump(addr);
                cycles = 16;
            }
            Instruction::STA8 { dst_addr, src } => {
                let dst: u16 = cpu.get16(dst_addr);
                mem.set(dst, cpu.get(src));
                cycles = 8;
            }
            Instruction::STHA { addr } => {
                mem.set((0xFF00 + addr as u16) as u16, cpu.get(CpuRegister::A));
                cycles = 12;
            }
            Instruction::STHCA => {
                let addr: u16 = cpu.get(CpuRegister::C) as u16;
                mem.set(0xFF00 + addr, cpu.get(CpuRegister::A));
                cycles = 8;
            }
            Instruction::STI8 { dst_addr, val } => {
                mem.set(cpu.get16(dst_addr), val);
                cycles = 8;
            }
            Instruction::STAA { addr } => {
                mem.set(addr, cpu.get(CpuRegister::A));
                cycles = 16;
            }
            Instruction::STD => {
                let addr: u16 = cpu.get16(Cpu16Register::HL);
                mem.set(addr, cpu.get(CpuRegister::A));
                cpu.set16(Cpu16Register::HL, addr - 1);
                cycles = 8;
            }
            Instruction::STI => {
                let addr: u16 = cpu.get16(Cpu16Register::HL);
                mem.set(addr, cpu.get(CpuRegister::A));
                cpu.set16(Cpu16Register::HL, addr + 1);
                cycles = 8;
            }
            Instruction::SUBR { reg } => {
                let val: u8 = cpu.get(reg);
                math::subtract(cpu, val);
                cycles = 4;
            }
            Instruction::SUBA { reg_addr } => {
                let val: u8 = mem.get(cpu.get16(reg_addr));
                math::subtract(cpu, val);
                cycles = 8;
            }
            Instruction::SUBI { val } => {
                math::subtract(cpu, val);
                cycles = 8;
            }
            Instruction::SBCR { reg } => {
                let val: u8 = cpu.get(reg) + cpu.c_flag();
                math::subtract(cpu, val);
                cycles = 4;
            }
            Instruction::SBCA { reg_addr } => {
                let val: u8 = mem.get(cpu.get16(reg_addr)) + cpu.c_flag();
                math::subtract(cpu, val);
                cycles = 8;
            }
            Instruction::ADDI { val } => {
                math::add(cpu, val);
                cycles = 8;
            }
            Instruction::ADD16 { src } => {
                let val: u16 = cpu.get16(src);
                math::add16(cpu, val);
                cycles = 8;
            }
            Instruction::XORR { reg } => {
                let val: u8 = cpu.get(reg);
                math::xor(cpu, val);
                cycles = 4;
            }
            Instruction::XORA => {
                let addr: u16 = cpu.get16(Cpu16Register::HL);
                math::xor(cpu, mem.get(addr));
                cycles = 8;
            }
            Instruction::XORI { val } => {
                math::xor(cpu, val);
                cycles = 8;
            }
            Instruction::INC { reg } => {
                math::increment(cpu, reg);
                cycles = 4;
            }
            Instruction::DEC { reg } => {
                math::decrement(cpu, reg);
                cycles = 4;
            }
            Instruction::INC16 { reg } => {
                math::increment16(cpu, reg);
                cycles = 8;
            }
            Instruction::DEC16 { reg } => {
                math::decrement16(cpu, reg);
                cycles = 8;
            }
            Instruction::CPL => {
                math::complement(cpu);
                cycles = 4;
            }
            Instruction::DI => {
                cpu.disable_interrupts();
                cycles = 4;
            }
            Instruction::EI => {
                cpu.enable_interrupts();
                cycles = 4;
            }
            Instruction::CMPI { val } => {
                math::compare(cpu, val);
                cycles = 8;
            }
            Instruction::CMPR { reg } => {
                let val: u8 = cpu.get(reg);
                math::compare(cpu, val);
                cycles = 4;
            }
            Instruction::CMPA => {
                let addr: u16 = cpu.get16(Cpu16Register::HL);
                math::compare(cpu, mem.get(addr));
                cycles = 8;
            }
            Instruction::ORR { reg } => {
                let val: u8 = cpu.get(reg);
                math::or(cpu, val);
                cycles = 4;
            }
            Instruction::ORA => {
                let addr: u16 = cpu.get16(Cpu16Register::HL);
                math::or(cpu, mem.get(addr));
                cycles = 8;
            }
            Instruction::ORI { val } => {
                math::or(cpu, val);
                cycles = 8;
            }
            Instruction::ANDR { reg } => {
                let val: u8 = cpu.get(reg);
                math::and(cpu, val);
                cycles = 4;
            }
            Instruction::ANDA => {
                let val: u8 = mem.get(cpu.get16(Cpu16Register::HL));
                math::and(cpu, val);
                cycles = 8;
            }
            Instruction::ANDI { val } => {
                math::and(cpu, val);
                cycles = 8;
            }
            Instruction::PUSH { reg } => {
                let sp = cpu.sp;
                mem.set16(sp, cpu.get16(reg));
                cpu.sp = sp - 2;
                cycles = 16;
            }
            Instruction::POP { reg } => {
                let sp = cpu.sp + 2;
                cpu.set16(reg, mem.get16(sp));
                cpu.sp = sp;
                cycles = 16;
            }
        };

        return cycles;
    }
}

fn read_opcode(opcode: u8, argstart: u16, mem: &Memory) -> Instruction {
    match opcode {
        0x00 => Instruction::Noop,
        0x01 => Instruction::LDI16 {
            val: mem.get16(argstart),
            reg: Cpu16Register::BC,
        },
        0x02 => Instruction::STA8 {
            dst_addr: Cpu16Register::BC,
            src: CpuRegister::A,
        },
        0x03 => Instruction::INC16 {
            reg: Cpu16Register::BC,
        },
        0x04 => Instruction::INC {
            reg: CpuRegister::B,
        },
        0x05 => Instruction::DEC {
            reg: CpuRegister::B,
        },
        0x06 => Instruction::LDI8 {
            val: mem.get(argstart),
            reg: CpuRegister::B,
        },
        0x09 => Instruction::ADD16 {
            src: Cpu16Register::BC,
        },
        0x0A => Instruction::LDA8 {
            src_addr: Cpu16Register::BC,
            dst: CpuRegister::A,
        },
        0x0B => Instruction::DEC16 {
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
        0x12 => Instruction::STA8 {
            dst_addr: Cpu16Register::DE,
            src: CpuRegister::A,
        },
        0x13 => Instruction::INC16 {
            reg: Cpu16Register::DE,
        },
        0x14 => Instruction::INC {
            reg: CpuRegister::D,
        },
        0x15 => Instruction::DEC {
            reg: CpuRegister::D,
        },
        0x16 => Instruction::LDI8 {
            val: mem.get(argstart),
            reg: CpuRegister::D,
        },
        0x18 => Instruction::JR {
            offset: mem.get(argstart) as i8,
        },
        0x19 => Instruction::ADD16 {
            src: Cpu16Register::DE,
        },
        0x1A => Instruction::LDA8 {
            src_addr: Cpu16Register::DE,
            dst: CpuRegister::A,
        },
        0x1B => Instruction::DEC16 {
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
        0x20 => Instruction::JRNZ {
            offset: mem.get(argstart) as i8,
        },
        0x21 => Instruction::LDI16 {
            val: mem.get16(argstart),
            reg: Cpu16Register::HL,
        },
        0x22 => Instruction::STI,
        0x23 => Instruction::INC16 {
            reg: Cpu16Register::HL,
        },
        0x24 => Instruction::INC {
            reg: CpuRegister::H,
        },
        0x25 => Instruction::DEC {
            reg: CpuRegister::H,
        },
        0x26 => Instruction::LDI8 {
            val: mem.get(argstart),
            reg: CpuRegister::H,
        },
        0x28 => Instruction::JRZ {
            offset: mem.get(argstart) as i8,
        },
        0x29 => Instruction::ADD16 {
            src: Cpu16Register::HL,
        },
        0x2A => Instruction::LDI,
        0x2B => Instruction::DEC16 {
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
        0x32 => Instruction::STD,
        0x33 => Instruction::INC16 {
            reg: Cpu16Register::SP,
        },
        0x36 => Instruction::STI8 {
            dst_addr: Cpu16Register::HL,
            val: mem.get(argstart),
        },
        0x39 => Instruction::ADD16 {
            src: Cpu16Register::SP,
        },
        0x3A => Instruction::LDD,
        0x3B => Instruction::DEC16 {
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
        0xA0 => Instruction::ANDR { reg: CpuRegister::B },
        0xA1 => Instruction::ANDR { reg: CpuRegister::C },
        0xA2 => Instruction::ANDR { reg: CpuRegister::D },
        0xA3 => Instruction::ANDR { reg: CpuRegister::E },
        0xA4 => Instruction::ANDR { reg: CpuRegister::H },
        0xA5 => Instruction::ANDR { reg: CpuRegister::L },
        0xA6 => Instruction::ANDA,
        0xA7 => Instruction::ANDR { reg: CpuRegister::A },
        0xA8 => Instruction::XORR {
            reg: CpuRegister::B,
        },
        0xA9 => Instruction::XORR {
            reg: CpuRegister::C,
        },
        0xAA => Instruction::XORR {
            reg: CpuRegister::D,
        },
        0xAB => Instruction::XORR {
            reg: CpuRegister::E,
        },
        0xAC => Instruction::XORR {
            reg: CpuRegister::H,
        },
        0xAD => Instruction::XORR {
            reg: CpuRegister::L,
        },
        0xAE => Instruction::XORA,
        0xAF => Instruction::XORR {
            reg: CpuRegister::A,
        },
        0xB0 => Instruction::ORR {
            reg: CpuRegister::B,
        },
        0xB1 => Instruction::ORR {
            reg: CpuRegister::C,
        },
        0xB2 => Instruction::ORR {
            reg: CpuRegister::D,
        },
        0xB3 => Instruction::ORR {
            reg: CpuRegister::E,
        },
        0xB4 => Instruction::ORR {
            reg: CpuRegister::H,
        },
        0xB5 => Instruction::ORR {
            reg: CpuRegister::L,
        },
        0xB6 => Instruction::ORA,
        0xB7 => Instruction::ORR {
            reg: CpuRegister::A,
        },
        0xB8 => Instruction::CMPR {
            reg: CpuRegister::B,
        },
        0xB9 => Instruction::CMPR {
            reg: CpuRegister::C,
        },
        0xBA => Instruction::CMPR {
            reg: CpuRegister::D,
        },
        0xBB => Instruction::CMPR {
            reg: CpuRegister::E,
        },
        0xBC => Instruction::CMPR {
            reg: CpuRegister::H,
        },
        0xBD => Instruction::CMPR {
            reg: CpuRegister::L,
        },
        0xBE => Instruction::CMPA,
        0xBF => Instruction::CMPR {
            reg: CpuRegister::A,
        },
        0xC0 => Instruction::RETNZ,
        0xC1 => Instruction::POP {
            reg: Cpu16Register::BC,
        },
        0xC2 => Instruction::JPNZ {
            addr: mem.get16(argstart),
        },
        0xC3 => Instruction::JP {
            addr: mem.get16(argstart),
        },
        0xC5 => Instruction::PUSH {
            reg: Cpu16Register::BC,
        },
        0xC6 => Instruction::ADDI {
            val: mem.get(argstart),
        },
        0xC7 => Instruction::RST { addr: 0x0000 },
        0xC8 => Instruction::RETZ,
        0xC9 => Instruction::RET,
        0xCD => Instruction::CALL {
            addr: mem.get16(argstart),
        },
        0xCF => Instruction::RST { addr: 0x0008 },
        0xD0 => Instruction::RETNC,
        0xD1 => Instruction::POP {
            reg: Cpu16Register::DE,
        },
        0xD5 => Instruction::PUSH {
            reg: Cpu16Register::DE,
        },
        0xD6 => Instruction::SUBI {
            val: mem.get(argstart),
        },
        0xD7 => Instruction::RST { addr: 0x0010 },
        0xD8 => Instruction::RETC,
        0xD9 => Instruction::RETI,
        0xDF => Instruction::RST { addr: 0x0018 },
        0xE0 => Instruction::STHA {
            addr: mem.get(argstart),
        },
        0xE1 => Instruction::POP {
            reg: Cpu16Register::HL,
        },
        0xE2 => Instruction::STHCA,
        0xE5 => Instruction::PUSH {
            reg: Cpu16Register::HL,
        },
        0xE6 => Instruction::ANDI {
            val: mem.get(argstart),
        },
        0xE7 => Instruction::RST { addr: 0x0020 },
        0xEF => Instruction::RST { addr: 0x0028 },
        0xEA => Instruction::STAA {
            addr: mem.get16(argstart),
        },
        0xEE => Instruction::XORI {
            val: mem.get(argstart),
        },
        0xF0 => Instruction::LDHA {
            addr: mem.get(argstart),
        },
        0xF1 => Instruction::POP {
            reg: Cpu16Register::AF,
        },
        0xF2 => Instruction::LDHCA,
        0xF3 => Instruction::DI,
        0xF5 => Instruction::PUSH {
            reg: Cpu16Register::AF,
        },
        0xF6 => Instruction::ORI {
            val: mem.get(argstart),
        },
        0xF7 => Instruction::RST { addr: 0x0030 },
        0xFA => Instruction::LDAA { addr: mem.get16(argstart) },
        0xFB => Instruction::EI,
        0xFE => Instruction::CMPI {
            val: mem.get(argstart),
        },
        0xFF => Instruction::RST { addr: 0x0038 },
        _ => panic!("Unknown opcode {:2X}", opcode),
    }
}

fn read_extended_opcode(opcode: u8, _argstart: u16, _mem: &Memory) -> Instruction {
    match opcode {
        _ => panic!("Unknown extended opcode 0xC3{:2X}", opcode),
    }
}
