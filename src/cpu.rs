use memory::Memory;
use instruction::Instruction;

pub struct Cpu {
    pc: u16,

    a: u8,
    b: u8,
    c: u8,
    // TODO other registers
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            a: 0,
            b: 0,
            c: 0
        }
    }

    pub fn cycle(&mut self, mem: &mut Memory) {
        let instr = Instruction::read(&mem, self.pc);

        instr.execute(self, mem);

        self.pc += instr.mem_size();
    }

    pub fn print_state(&self) {
        println!("pc {:x} a {:x} b {:x} c {:x} ", self.pc, self.a, self.b, self.c);
    }
}
