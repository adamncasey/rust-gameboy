mod rom;
mod memory;
mod cpu;
mod instruction;
mod math;
mod gpu;

use rom::Rom;
use memory::Memory;
use cpu::Cpu;
use gpu::Gpu;

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
    let mut gpu: Gpu = Gpu::new();

    let mut steps: u64 = 0;

    let mut debugging;
    loop {
        debugging = steps > 50000000;
        // read input
        // if should quit: break loop

        let cycles: u8 = cpu.cycle(&mut memory, debugging);

        let draw = gpu.cycle(&mut memory, cycles);

        if debugging {
            println!("State after {} total steps {} cycles", steps, cycles);
            cpu.print_state();
            gpu.print_state();
        }

        //let _ = stdin().read(&mut [0u8]).unwrap();

        steps += 1;
    }
}
