mod rom;
mod memory;
mod cpu;
mod instruction;
mod math;

use rom::Rom;
use memory::Memory;
use cpu::Cpu;

fn main() {
    println!("Hello, world!");

    let cartridge: Rom = Rom::load("tetris.gb").unwrap();

    println!("Loaded rom: {:?}", cartridge.game_title);

    if cartridge.rom_type != 0 {
        println!("ROM type unsupported");
        return;
    }

    // Setup
    let mut memory: Memory = Memory::new(cartridge);

    let mut cpu: Cpu = Cpu::new();

    // Start

    let mut cycle: u64 = 0;
    loop {
        cpu.cycle(&mut memory);

        // read input

        // if should quit: break loop

        println!("Cpu State after {} cycles", cycle);
        cpu.print_state();

        cycle += 1;
    }
}
