use cpu::Cpu;
use memory::Memory;

/*
pub struct Instruction {
    op: u16,
    arg1: u8, // Might make sense to indicate arg type
    arg2: u8,
    size: u16,
}*/

#[derive(Debug)]
pub enum Instruction {
    Noop,
    LDBC { val: u16 },
    LDDE { val: u16 },
    LDHL { val: u16 },
    LDSP { val: u16 },
    JP { addr: u16 },
    CALL { addr: u16 },
    RST18,
    STHA { addr: u8 },
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
            Instruction::LDBC { .. } => 3,
            Instruction::LDDE { .. } => 3,
            Instruction::LDHL { .. } => 3,
            Instruction::LDSP { .. } => 3,
            Instruction::JP { .. } => 3,
            Instruction::CALL { .. } => 3,
            Instruction::RST18 => 1,
            Instruction::STHA { .. } => 2,
        }
    }

    pub fn execute(&self, cpu: &mut Cpu, mem: &mut Memory) {
        // Execute based on opcode
        match *self {
            Instruction::Noop => (),
            Instruction::LDBC { val } => {
                cpu.c = (0x00FF & val) as u8;
                cpu.b = ((0xFF00 & val) >> 8) as u8;
            }
            Instruction::LDDE { val } => {
                cpu.e = (0x00FF & val) as u8;
                cpu.d = ((0xFF00 & val) >> 8) as u8;
            }
            Instruction::LDHL { val } => {
                cpu.l = (0x00FF & val) as u8;
                cpu.h = ((0xFF00 & val) >> 8) as u8;
            }
            Instruction::LDSP { val } => {
                cpu.sp = val;
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
            Instruction::STHA { addr } => mem.set((0xFF00 + addr as u16) as u16, cpu.a),
        }
    }
}

fn read_opcode(opcode: u8, argstart: u16, mem: &Memory) -> Instruction {
    match opcode {
        0x00 => Instruction::Noop,
        0x01 => Instruction::LDBC {
            val: mem.get16(argstart),
        },
        0x11 => Instruction::LDDE {
            val: mem.get16(argstart),
        },
        0x21 => Instruction::LDHL {
            val: mem.get16(argstart),
        },
        0x31 => Instruction::LDSP {
            val: mem.get16(argstart),
        },
        0xC3 => Instruction::JP {
            addr: mem.get16(argstart),
        },
        0xCD => Instruction::CALL {
            addr: mem.get16(argstart),
        },
        0xDF => Instruction::RST18,
        0xE0 => Instruction::STHA {
            addr: mem.get(argstart),
        },
        _ => panic!("Unknown opcode {:2X}", opcode),
    }
}

fn read_extended_opcode(opcode: u8, _argstart: u16, _mem: &Memory) -> Instruction {
    match opcode {
        _ => panic!("Unknown extended opcode 0xC3{:2X}", opcode),
    }
}
