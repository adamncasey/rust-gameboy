use cpu::Cpu;
use memory::Memory;

pub struct Instruction {
    op: u16,
    arg1: u8, // Might make sense to indicate arg type
    arg2: u8,
    size: u16,
}

impl Instruction {
    pub fn read(mem: &Memory, addr: u16) -> Instruction {
        let opstart = mem.get(addr);

        let opcode = 0x0;
        let arg1 = 0;
        let arg2 = 0;
        let size = 1;

        if opstart != 0 {
            panic!("Unknown opcode {:x}", opstart);
        }

        // if start == 0xC3
        // Read another byte.
        // opsize = 2;

        Instruction {
            op: opcode,
            arg1: arg1,
            arg2: arg2,
            size: size,
        }
    }

    pub fn mem_size(&self) -> u16 {
        self.size
    }

    pub fn execute(&self, cpu: &mut Cpu, mem: &mut Memory) {
        // Execute based on opcode
    }
}
