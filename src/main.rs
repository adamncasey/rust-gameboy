mod rom;
mod memory;
mod cpu;
mod instruction;
mod math;

use rom::Rom;
use memory::Memory;
use cpu::Cpu;

use std::io::{stdin, Read};

fn main() {
    let cartridge: Rom = Rom::load("tetris.gb").unwrap();

    println!("Loaded rom: {:?}", cartridge.game_title);

    if cartridge.rom_type != 0 {
        println!("ROM type unsupported");
        return;
    }

    let mut memory: Memory = Memory::new(cartridge);
    let mut cpu: Cpu = Cpu::new();
    //let mut Gpu: Gpu = Gpu::new();

    let mut steps: u64 = 0;
    let mut cycles: u64 = 0;
    loop {
        //gpu.cycle(&mut memory, cycles);

        // read input
        // if should quit: break loop

        cycles += cpu.cycle(&mut memory) as u64;
        println!("Cpu State after {} steps {} cycles", steps, cycles);
        cpu.print_state();

        //let _ = stdin().read(&mut [0u8]).unwrap();

        steps += 1;
    }
}
