use crate::cpu::{Cpu, Cpu16Register, CpuRegister};
use crate::math;
use crate::memory::Memory;
use crate::opcode::{read_extended_opcode, read_opcode};

#[derive(Debug)]
pub enum Instruction {
    Noop,
    LDI16 {
        val: u16,
        reg: Cpu16Register,
    },
    LDI8 {
        val: u8,
        reg: CpuRegister,
    },
    LDR8 {
        src: CpuRegister,
        dst: CpuRegister,
    },
    LDA8 {
        src_addr: Cpu16Register,
        dst: CpuRegister,
    },
    LDHA {
        addr: u8,
    },
    LDHCA,
    LDD,
    LDI,
    LDAA {
        addr: u16,
    },
    LDHLI {
        offset: i8,
    },
    LDSPA {
        addr: u16,
    },
    LDSPHL,
    JP {
        addr: u16,
    },
    JPNZ {
        addr: u16,
    },
    JPZ {
        addr: u16,
    },
    JPNC {
        addr: u16,
    },
    JPC {
        addr: u16,
    },
    JPA,
    JR {
        offset: i8,
    },
    JRNZ {
        offset: i8,
    },
    JRZ {
        offset: i8,
    },
    JRNC {
        offset: i8,
    },
    JRC {
        offset: i8,
    },
    CALL {
        addr: u16,
    },
    CALLNZ {
        addr: u16,
    },
    CALLNC {
        addr: u16,
    },
    CALLZ {
        addr: u16,
    },
    CALLC {
        addr: u16,
    },
    RET,
    RETNZ,
    RETZ,
    RETNC,
    RETC,
    RETI,
    RST {
        addr: u16,
    },
    STA8 {
        dst_addr: Cpu16Register,
        src: CpuRegister,
    },
    STI8 {
        dst_addr: Cpu16Register,
        val: u8,
    },
    STHA {
        addr: u8,
    },
    STHCA,
    STAA {
        addr: u16,
    },
    STD,
    STI,
    SUBR {
        reg: CpuRegister,
    },
    SUBA {
        reg_addr: Cpu16Register,
    },
    SUBI {
        val: u8,
    },
    SBCR {
        reg: CpuRegister,
    },
    SBCA {
        reg_addr: Cpu16Register,
    },
    SBCI {
        val: u8,
    },
    ADDR {
        reg: CpuRegister,
    },
    ADDA,
    ADDI {
        val: u8,
    },
    ADD16 {
        src: Cpu16Register,
    },
    ADDSP {
        val: u8,
    },
    ADC {
        reg: CpuRegister,
    },
    ADCA,
    ADCI {
        val: u8,
    },
    XORR {
        reg: CpuRegister,
    },
    XORA,
    XORI {
        val: u8,
    },
    INC {
        reg: CpuRegister,
    },
    INCA,
    DEC {
        reg: CpuRegister,
    },
    DECA,
    INC16 {
        reg: Cpu16Register,
    },
    DEC16 {
        reg: Cpu16Register,
    },
    CPL,
    CCF,
    DI,
    EI,
    CMPR {
        reg: CpuRegister,
    },
    CMPI {
        val: u8,
    },
    CMPA,
    ORR {
        reg: CpuRegister,
    },
    ORA,
    ORI {
        val: u8,
    },
    ANDR {
        reg: CpuRegister,
    },
    ANDA,
    ANDI {
        val: u8,
    },
    PUSH {
        reg: Cpu16Register,
    },
    POP {
        reg: Cpu16Register,
    },
    SWAP {
        reg: CpuRegister,
    },
    SWAPA,
    BIT {
        n: u8,
        reg: CpuRegister,
    },
    BITA {
        n: u8,
    },
    SET {
        n: u8,
        reg: CpuRegister,
    },
    SETA {
        n: u8,
    },
    RESET {
        n: u8,
        reg: CpuRegister,
    },
    RESETA {
        n: u8,
    },
    SLA {
        reg: CpuRegister,
    },
    SLAA,
    SRL {
        reg: CpuRegister,
    },
    SRLA,
    SRA {
        reg: CpuRegister,
    },
    SRAA,
    RLCA,
    RLA,
    RRCA,
    RRA,
    RLC {
        reg: CpuRegister,
    },
    RLCHL,
    RL {
        reg: CpuRegister,
    },
    RLHL,
    RRC {
        reg: CpuRegister,
    },
    RRCHL,
    RR {
        reg: CpuRegister,
    },
    RRHL,
    SCF,
    DAA,
    HALT,

    ILLEGAL,
    UNIMPLEMENTED {
        opcode: u8,
    },
}

impl Instruction {
    pub fn read(mem: &Memory, addr: u16) -> Instruction {
        let opcode: u8 = mem.get(addr);

        if opcode == 0xCB {
            return read_extended_opcode(mem.get(addr + 1), addr + 2, mem);
        }

        read_opcode(opcode, addr + 1, mem)
    }

    pub fn disassemble(mem: &Memory, start_addr: u16, num_instrs: usize) -> Vec<Instruction> {
        let mut instrs = Vec::with_capacity(num_instrs);
        let mut addr = start_addr;

        for _ in 0..num_instrs {
            instrs.push(Instruction::read(mem, addr));
            addr = addr.wrapping_add(Instruction::mem_size(instrs.last().unwrap()));
        }

        instrs
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
            Instruction::LDHLI { .. } => 2,
            Instruction::LDSPA { .. } => 3,
            Instruction::LDSPHL => 1,
            Instruction::JP { .. } => 3,
            Instruction::JPNZ { .. } => 3,
            Instruction::JPZ { .. } => 3,
            Instruction::JPNC { .. } => 3,
            Instruction::JPC { .. } => 3,
            Instruction::JPA => 1,
            Instruction::JR { .. } => 2,
            Instruction::JRNZ { .. } => 2,
            Instruction::JRZ { .. } => 2,
            Instruction::JRNC { .. } => 2,
            Instruction::JRC { .. } => 2,
            Instruction::CALL { .. } => 3,
            Instruction::CALLNC { .. } => 3,
            Instruction::CALLNZ { .. } => 3,
            Instruction::CALLC { .. } => 3,
            Instruction::CALLZ { .. } => 3,
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
            Instruction::SBCI { .. } => 2,
            Instruction::ADDR { .. } => 1,
            Instruction::ADDA => 1,
            Instruction::ADDI { .. } => 2,
            Instruction::ADD16 { .. } => 1,
            Instruction::ADDSP { .. } => 2,
            Instruction::ADC { .. } => 1,
            Instruction::ADCA => 1,
            Instruction::ADCI { .. } => 2,
            Instruction::XORR { .. } => 1,
            Instruction::XORA => 1,
            Instruction::XORI { .. } => 2,
            Instruction::INC { .. } => 1,
            Instruction::INCA => 1,
            Instruction::DEC { .. } => 1,
            Instruction::DECA => 1,
            Instruction::INC16 { .. } => 1,
            Instruction::DEC16 { .. } => 1,
            Instruction::CPL => 1,
            Instruction::CCF => 1,
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
            Instruction::SWAP { .. } => 2,
            Instruction::SWAPA => 2,
            Instruction::BIT { .. } => 2,
            Instruction::BITA { .. } => 2,
            Instruction::SET { .. } => 2,
            Instruction::SETA { .. } => 2,
            Instruction::RESET { .. } => 2,
            Instruction::RESETA { .. } => 2,
            Instruction::SLA { .. } => 2,
            Instruction::SLAA => 2,
            Instruction::SRL { .. } => 2,
            Instruction::SRLA => 2,
            Instruction::SRA { .. } => 2,
            Instruction::SRAA => 2,
            Instruction::RLCA => 1,
            Instruction::RRCA => 1,
            Instruction::RLA => 1,
            Instruction::RRA => 1,
            Instruction::RLC { .. } => 2,
            Instruction::RLCHL => 2,
            Instruction::RRC { .. } => 2,
            Instruction::RRCHL => 2,
            Instruction::RL { .. } => 2,
            Instruction::RLHL => 2,
            Instruction::RR { .. } => 2,
            Instruction::RRHL => 2,
            Instruction::SCF => 1,
            Instruction::DAA => 1,
            Instruction::HALT => 1,

            Instruction::ILLEGAL => 1,
            Instruction::UNIMPLEMENTED { .. } => 1,
        }
    }

    pub fn execute(&self, cpu: &mut Cpu, mem: &mut Memory) -> u8 {
        let cycles: u8;
        // Execute based on opcode
        match *self {
            Instruction::Noop => cycles = 4,
            Instruction::ILLEGAL => panic!("Illegal instruction"),
            Instruction::UNIMPLEMENTED { opcode } => panic!("Unimplemented opcode 0x{:x}", opcode),
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
                cpu.set(CpuRegister::A, mem.get(0xFF00 + u16::from(addr)));
                cycles = 12;
            }
            Instruction::LDHCA => {
                let addr: u16 = u16::from(cpu.get(CpuRegister::C));
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
            Instruction::LDHLI { offset } => {
                let addr = i32::from(cpu.get16(Cpu16Register::SP)) + i32::from(offset);
                cpu.set16(Cpu16Register::HL, addr as u16);
                cycles = 12;
                // TODO set H/C flags. Z + N = false.
            }
            Instruction::LDSPA { addr } => {
                cpu.set16(Cpu16Register::SP, mem.get16(addr));
                cycles = 20;
            }
            Instruction::LDSPHL => {
                let val: u16 = cpu.get16(Cpu16Register::HL);
                cpu.set16(Cpu16Register::SP, val);
                cycles = 8;
            }
            Instruction::JP { addr } => {
                cpu.jump(addr);
                cycles = 16;
            }
            Instruction::JPNZ { addr } => {
                if !cpu.z_flag() {
                    cpu.jump(addr);
                    cycles = 16;
                } else {
                    cycles = 12;
                }
            }
            Instruction::JPZ { addr } => {
                if cpu.z_flag() {
                    cpu.jump(addr);
                    cycles = 16;
                } else {
                    cycles = 12;
                }
            }
            Instruction::JPNC { addr } => {
                if !cpu.c_flag() {
                    cpu.jump(addr);
                    cycles = 16;
                } else {
                    cycles = 12;
                }
            }
            Instruction::JPC { addr } => {
                if cpu.c_flag() {
                    cpu.jump(addr);
                    cycles = 16;
                } else {
                    cycles = 12;
                }
            }
            Instruction::JPA => {
                let addr = cpu.get16(Cpu16Register::HL);
                cpu.jump(addr);
                cycles = 4;
            }
            Instruction::JR { offset } => {
                cpu.rjump(offset + 2);
                cycles = 12;
            }
            Instruction::JRNZ { offset } => {
                if !cpu.z_flag() {
                    cpu.rjump(offset + 2);
                    cycles = 12;
                } else {
                    cycles = 8;
                }
            }
            Instruction::JRZ { offset } => {
                if cpu.z_flag() {
                    cpu.rjump(offset + 2);
                    cycles = 12;
                } else {
                    cycles = 8;
                }
            }
            Instruction::JRNC { offset } => {
                if !cpu.c_flag() {
                    cpu.rjump(offset + 2);
                    cycles = 12;
                } else {
                    cycles = 8;
                }
            }
            Instruction::JRC { offset } => {
                if cpu.c_flag() {
                    cpu.rjump(offset + 2);
                    cycles = 12;
                } else {
                    cycles = 8;
                }
            }
            Instruction::CALL { addr } => {
                cpu.sp -= 2;
                mem.set16(cpu.sp, cpu.pc + Instruction::mem_size(self));
                cpu.jump(addr);
                cycles = 24;
            }
            Instruction::CALLNZ { addr } => {
                if !cpu.z_flag() {
                    cpu.sp -= 2;
                    mem.set16(cpu.sp, cpu.pc + Instruction::mem_size(self));
                    cpu.jump(addr);
                    cycles = 24;
                } else {
                    cycles = 12;
                }
            }
            Instruction::CALLNC { addr } => {
                if !cpu.c_flag() {
                    cpu.sp -= 2;
                    mem.set16(cpu.sp, cpu.pc + Instruction::mem_size(self));
                    cpu.jump(addr);
                    cycles = 24;
                } else {
                    cycles = 12;
                }
            }
            Instruction::CALLZ { addr } => {
                if cpu.z_flag() {
                    cpu.sp -= 2;
                    mem.set16(cpu.sp, cpu.pc + Instruction::mem_size(self));
                    cpu.jump(addr);
                    cycles = 24;
                } else {
                    cycles = 12;
                }
            }
            Instruction::CALLC { addr } => {
                if cpu.c_flag() {
                    cpu.sp -= 2;
                    mem.set16(cpu.sp, cpu.pc + Instruction::mem_size(self));
                    cpu.jump(addr);
                    cycles = 24;
                } else {
                    cycles = 12;
                }
            }
            Instruction::RET => {
                cpu.ret(mem);
                cycles = 16;
            }
            Instruction::RETNZ => {
                if !cpu.z_flag() {
                    cpu.ret(mem);
                    cycles = 20;
                } else {
                    cycles = 8;
                }
            }
            Instruction::RETZ => {
                if cpu.z_flag() {
                    cpu.ret(mem);
                    cycles = 20;
                } else {
                    cycles = 8;
                }
            }
            Instruction::RETNC => {
                if !cpu.c_flag() {
                    cpu.ret(mem);
                    cycles = 20;
                } else {
                    cycles = 8;
                }
            }
            Instruction::RETC => {
                if cpu.c_flag() {
                    cpu.ret(mem);
                    cycles = 20;
                } else {
                    cycles = 8;
                }
            }
            Instruction::RETI => {
                cpu.ret(mem);
                cpu.enable_interrupts();
                cycles = 16;
            }
            Instruction::RST { addr } => {
                // Store next pc on stack & jump to addr
                cpu.sp -= 2;
                mem.set16(cpu.sp, cpu.pc + Instruction::mem_size(self));
                cpu.jump(addr);
                cycles = 16;
            }
            Instruction::STA8 { dst_addr, src } => {
                let dst: u16 = cpu.get16(dst_addr);
                mem.set(dst, cpu.get(src));
                cycles = 8;
            }
            Instruction::STHA { addr } => {
                mem.set((0xFF00 + u16::from(addr)) as u16, cpu.get(CpuRegister::A));
                cycles = 12;
            }
            Instruction::STHCA => {
                let addr: u16 = u16::from(cpu.get(CpuRegister::C));
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
                let val: u8 = cpu.get(reg).wrapping_add(cpu.c_flag() as u8);
                math::subtract(cpu, val);
                cycles = 4;
            }
            Instruction::SBCA { reg_addr } => {
                let val: u8 = mem
                    .get(cpu.get16(reg_addr))
                    .wrapping_add(cpu.c_flag() as u8);
                math::subtract(cpu, val);
                cycles = 8;
            }
            Instruction::SBCI { val } => {
                let newval = val.wrapping_add(cpu.c_flag() as u8);
                math::subtract(cpu, newval);
                cycles = 8;
            }
            Instruction::ADDR { reg } => {
                let val = cpu.get(reg);
                math::add(cpu, val);
                cycles = 4;
            }
            Instruction::ADDA => {
                let val = mem.get(cpu.get16(Cpu16Register::HL));
                math::add(cpu, val);
                cycles = 8;
            }
            Instruction::ADDI { val } => {
                math::add(cpu, val);
                cycles = 8;
            }
            Instruction::ADD16 { src } => {
                let val: u16 = cpu.get16(src);
                math::add16(cpu, Cpu16Register::HL, val);
                cycles = 8;
            }
            Instruction::ADDSP { val } => {
                math::add16(cpu, Cpu16Register::SP, u16::from(val));
                cycles = 16;
            }
            Instruction::ADC { reg } => {
                // TODO Carry flag interaction?
                let val: u8 = cpu.get(reg).wrapping_add(cpu.c_flag() as u8);
                math::add(cpu, val);
                cycles = 4;
            }
            Instruction::ADCA => {
                let val: u8 = mem
                    .get(cpu.get16(Cpu16Register::HL))
                    .wrapping_add(cpu.c_flag() as u8);
                math::add(cpu, val);
                cycles = 8;
            }
            Instruction::ADCI { val } => {
                // TODO Carry flag interaction?
                math::add(cpu, val.wrapping_add(cpu.c_flag() as u8));
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
                let val = cpu.get(reg);
                let newval = math::increment(cpu, val);

                cpu.set(reg, newval);
                cycles = 4;
            }
            Instruction::INCA => {
                let addr = cpu.get16(Cpu16Register::HL);
                let val = math::increment(cpu, mem.get(addr));
                mem.set(addr, val);
                cycles = 12;
            }
            Instruction::DEC { reg } => {
                let val = cpu.get(reg);
                let newval = math::decrement(cpu, val);
                cpu.set(reg, newval);
                cycles = 4;
            }
            Instruction::DECA => {
                let addr = cpu.get16(Cpu16Register::HL);
                let val = math::decrement(cpu, mem.get(addr));
                mem.set(addr, val);
                cycles = 12;
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
            Instruction::CCF => {
                math::complement_carry(cpu);
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
                cpu.sp = cpu.sp.wrapping_sub(2);
                mem.set16(cpu.sp, cpu.get16(reg));
                cycles = 16;
            }
            Instruction::POP { reg } => {
                let sp = cpu.sp;
                cpu.set16(reg, mem.get16(sp));
                cpu.sp = cpu.sp.wrapping_add(2);
                cycles = 16;
            }
            Instruction::SWAP { reg } => {
                let val = cpu.get(reg);
                let newval = math::swap_nibble(cpu, val);
                cpu.set(reg, newval);
                cycles = 8;
            }
            Instruction::SWAPA => {
                let addr: u16 = cpu.get16(Cpu16Register::HL);
                let val = mem.get(addr);
                mem.set(addr, math::swap_nibble(cpu, val));
                cycles = 16;
            }
            Instruction::BIT { n, reg } => {
                let val = cpu.get(reg);
                math::bit(cpu, val, n);
                cycles = 8;
            }
            Instruction::BITA { n } => {
                let addr = cpu.get16(Cpu16Register::HL);
                math::bit(cpu, mem.get(addr), n);
                cycles = 16;
            }
            Instruction::SET { n, reg } => {
                let newval = math::set(cpu.get(reg), n);
                cpu.set(reg, newval);
                cycles = 8;
            }
            Instruction::SETA { n } => {
                let addr = cpu.get16(Cpu16Register::HL);
                let newval = math::set(mem.get(addr), n);
                mem.set(addr, newval);
                cycles = 16;
            }
            Instruction::RESET { n, reg } => {
                let newval = math::reset(cpu.get(reg), n);
                cpu.set(reg, newval);
                cycles = 8;
            }
            Instruction::RESETA { n } => {
                let addr = cpu.get16(Cpu16Register::HL);
                let newval = math::reset(mem.get(addr), n);
                mem.set(addr, newval);
                cycles = 16;
            }
            Instruction::SLA { reg } => {
                let val = cpu.get(reg);
                let newval = math::sla(cpu, val);
                cpu.set(reg, newval);
                cycles = 8;
            }
            Instruction::SLAA => {
                let addr = cpu.get16(Cpu16Register::HL);
                let newval = math::sla(cpu, mem.get(addr));
                mem.set(addr, newval);
                cycles = 16;
            }
            Instruction::SRL { reg } => {
                let val = cpu.get(reg);
                let newval = math::srl(cpu, val);
                cpu.set(reg, newval);
                cycles = 8;
            }
            Instruction::SRLA => {
                let addr = cpu.get16(Cpu16Register::HL);
                let newval = math::srl(cpu, mem.get(addr));
                mem.set(addr, newval);
                cycles = 16;
            }
            Instruction::SRA { reg } => {
                let val = cpu.get(reg);
                let newval = math::sra(cpu, val);
                cpu.set(reg, newval);
                cycles = 8;
            }
            Instruction::SRAA => {
                let addr = cpu.get16(Cpu16Register::HL);
                let newval = math::sra(cpu, mem.get(addr));
                mem.set(addr, newval);
                cycles = 16;
            }
            Instruction::RLCA => {
                let newval = math::rlc(cpu, cpu.get(CpuRegister::A));
                cpu.set(CpuRegister::A, newval);
                cpu.set_flags(false, false, false, cpu.c_flag());
                cycles = 4;
            }
            Instruction::RRCA => {
                let newval = math::rrc(cpu, cpu.get(CpuRegister::A));
                cpu.set(CpuRegister::A, newval);
                cpu.set_flags(false, false, false, cpu.c_flag());
                cycles = 4;
            }
            Instruction::RLA => {
                let newval = math::rl(cpu, cpu.get(CpuRegister::A));
                cpu.set(CpuRegister::A, newval);
                cpu.set_flags(false, false, false, cpu.c_flag());
                cycles = 4;
            }
            Instruction::RRA => {
                let newval = math::rr(cpu, cpu.get(CpuRegister::A));
                cpu.set(CpuRegister::A, newval);
                cpu.set_flags(false, false, false, cpu.c_flag());
                cycles = 4;
            }
            Instruction::RLC { reg } => {
                let newval = math::rlc(cpu, cpu.get(reg));
                cpu.set(reg, newval);
                cycles = 8;
            }
            Instruction::RLCHL => {
                let addr = cpu.get16(Cpu16Register::HL);
                let newval = math::rlc(cpu, mem.get(addr));
                mem.set(addr, newval);
                cycles = 16;
            }
            Instruction::RRC { reg } => {
                let newval = math::rrc(cpu, cpu.get(reg));
                cpu.set(reg, newval);
                cycles = 8;
            }
            Instruction::RRCHL => {
                let addr = cpu.get16(Cpu16Register::HL);
                let newval = math::rrc(cpu, mem.get(addr));
                mem.set(addr, newval);
                cycles = 16;
            }
            Instruction::RL { reg } => {
                let newval = math::rl(cpu, cpu.get(reg));
                cpu.set(reg, newval);
                cycles = 8;
            }
            Instruction::RLHL => {
                let addr = cpu.get16(Cpu16Register::HL);
                let newval = math::rl(cpu, mem.get(addr));
                mem.set(addr, newval);
                cycles = 16;
            }
            Instruction::RR { reg } => {
                let newval = math::rr(cpu, cpu.get(reg));
                cpu.set(reg, newval);
                cycles = 8;
            }
            Instruction::RRHL => {
                let addr = cpu.get16(Cpu16Register::HL);
                let newval = math::rr(cpu, mem.get(addr));
                mem.set(addr, newval);
                cycles = 16;
            }
            Instruction::SCF => {
                cpu.set_flags(cpu.z_flag(), false, false, true);
                cycles = 4;
            }
            Instruction::DAA => {
                math::daa(cpu);
                cycles = 4;
            }
            Instruction::HALT => {
                cpu.halt();
                cycles = 4;
            }
        };

        cycles
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::{Cpu, CpuRegister};
    use crate::instruction::Instruction;
    use crate::memory::Memory;

    #[test]
    fn daa_instruction() {
        let mut cpu = Cpu::new();
        let mut mem = Memory::new(vec![0; 32 * 1024]);

        let test_cases = vec![
            (0b0001_0001, 0b0000_0000, 0b0001_0001),
            (0b0000_1001, 0b0000_0001, 0b0001_0000),
            (0b0001_1000, 0b0001_1000, 0b0011_0110),
        ];

        for test in test_cases.iter() {
            dbg!(test);
            cpu.a = test.0;
            cpu.b = test.1;

            let instr = Instruction::ADDR {
                reg: CpuRegister::B,
            };

            instr.execute(&mut cpu, &mut mem);

            let instr = Instruction::DAA;

            assert_eq!(instr.execute(&mut cpu, &mut mem), 4);
            assert_eq!(cpu.a, test.2);
        }
    }

    #[test]
    fn subi_instruction() {
        let mut cpu = Cpu::new();
        let mut mem = Memory::new(vec![0; 32 * 1024]);

        cpu.a = 0xD8;
        cpu.f = 0xC0;

        let instr = Instruction::SUBI { val: 5 };

        instr.execute(&mut cpu, &mut mem);

        assert_eq!(cpu.a, 0xD3);
        assert_eq!(cpu.f, 0x40);
    }
}
