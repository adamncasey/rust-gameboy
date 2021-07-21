use crate::memory::Memory;
use crate::instruction::Instruction;

use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct Disassembly {
    pub addr: u16,
    pub data_len: u8,
    pub data: Vec<u8>,
    pub desc: String
}

pub fn disassemble(start: u16, end: u16, mem: &Memory) -> Vec<Disassembly> {
    let mut output = Vec::new();

    let mut addr = start;

    while addr <= end {
        let instr = Instruction::read(mem, addr);
        let len = Instruction::mem_size(&instr);

        output.push(Disassembly {
            addr: addr,
            data_len: len as u8,
            data: mem.clone_bytes(addr, len),
            desc: format!("{:?}", instr)
        });

        addr += len;
    }

    output
}