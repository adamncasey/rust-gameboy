use cpu::*;
use instruction::Instruction;
use memory::Memory;


pub fn read_opcode(opcode: u8, argstart: u16, mem: &Memory) -> Instruction {
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
        0x80 => Instruction::ADDR { reg: CpuRegister::B },
        0x81 => Instruction::ADDR { reg: CpuRegister::C },
        0x82 => Instruction::ADDR { reg: CpuRegister::D },
        0x83 => Instruction::ADDR { reg: CpuRegister::E },
        0x84 => Instruction::ADDR { reg: CpuRegister::H },
        0x85 => Instruction::ADDR { reg: CpuRegister::L },
        0x86 => Instruction::ADDA,
        0x87 => Instruction::ADDR { reg: CpuRegister::A },
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
        0xCA => Instruction::JPZ { addr: mem.get16(argstart) },
        0xCD => Instruction::CALL {
            addr: mem.get16(argstart),
        },
        0xCF => Instruction::RST { addr: 0x0008 },
        0xD0 => Instruction::RETNC,
        0xD1 => Instruction::POP {
            reg: Cpu16Register::DE,
        },
        0xD3 => Instruction::ILLEGAL,
        0xD5 => Instruction::PUSH {
            reg: Cpu16Register::DE,
        },
        0xD6 => Instruction::SUBI {
            val: mem.get(argstart),
        },
        0xD7 => Instruction::RST { addr: 0x0010 },
        0xD8 => Instruction::RETC,
        0xD9 => Instruction::RETI,
        0xDB => Instruction::ILLEGAL,
        0xDD => Instruction::ILLEGAL,
        0xDF => Instruction::RST { addr: 0x0018 },
        0xE0 => Instruction::STHA {
            addr: mem.get(argstart),
        },
        0xE1 => Instruction::POP {
            reg: Cpu16Register::HL,
        },
        0xE2 => Instruction::STHCA,
        0xE3 => Instruction::ILLEGAL,
        0xE4 => Instruction::ILLEGAL,
        0xE5 => Instruction::PUSH {
            reg: Cpu16Register::HL,
        },
        0xE6 => Instruction::ANDI {
            val: mem.get(argstart),
        },
        0xE7 => Instruction::RST { addr: 0x0020 },
        0xE9 => Instruction::JPA,
        0xEF => Instruction::RST { addr: 0x0028 },
        0xEA => Instruction::STAA {
            addr: mem.get16(argstart),
        },
        0xEB => Instruction::ILLEGAL,
        0xEC => Instruction::ILLEGAL,
        0xED => Instruction::ILLEGAL,
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
        0xF4 => Instruction::ILLEGAL,
        0xF5 => Instruction::PUSH {
            reg: Cpu16Register::AF,
        },
        0xF6 => Instruction::ORI {
            val: mem.get(argstart),
        },
        0xF7 => Instruction::RST { addr: 0x0030 },
        0xFA => Instruction::LDAA { addr: mem.get16(argstart) },
        0xFB => Instruction::EI,
        0xFC => Instruction::ILLEGAL,
        0xFD => Instruction::ILLEGAL,
        0xFE => Instruction::CMPI {
            val: mem.get(argstart),
        },
        0xFF => Instruction::RST { addr: 0x0038 },
        _ => panic!("Unknown opcode {:2X}", opcode),
    }
}

pub fn read_extended_opcode(opcode: u8, _argstart: u16, _mem: &Memory) -> Instruction {
    match opcode {
        0x30 => Instruction::SWAP { reg: CpuRegister::B },
        0x31 => Instruction::SWAP { reg: CpuRegister::C },
        0x32 => Instruction::SWAP { reg: CpuRegister::D },
        0x33 => Instruction::SWAP { reg: CpuRegister::E },
        0x34 => Instruction::SWAP { reg: CpuRegister::H },
        0x35 => Instruction::SWAP { reg: CpuRegister::L },
        0x36 => Instruction::SWAPA,
        0x37 => Instruction::SWAP { reg: CpuRegister::A },
        0x87 => Instruction::RESET { n: 0, reg: CpuRegister::A},
        _ => panic!("Unknown extended opcode 0xCB{:2X}", opcode),
    }
}
