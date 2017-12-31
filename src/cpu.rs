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
    // TODO other registers
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
            l: 0
        }
    }

    pub fn cycle(&mut self, mem: &mut Memory) {
        let instr = Instruction::read(&mem, self.pc);

        println!("Executing {:?}", instr);

        instr.execute(self, mem);

        self.pc += Instruction::mem_size(&instr);
    }

    pub fn print_state(&self) {
        println!("pc {:x} a {:x} b {:x} c {:x} ", self.pc, self.a, self.b, self.c);
    }
}
