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

extern crate sfml;
use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture};
use sfml::window::{Event, Key, Style};

fn main() {
    let cartridge: Rom = Rom::load("tetris.gb").unwrap();

    println!("Loaded rom: {:?}", cartridge.game_title);

    if cartridge.rom_type != 0 {
        println!("ROM type unsupported");
        return;
    }

    let mut window = RenderWindow::new((160, 144), "gb-rust", Style::CLOSE, &Default::default());
    window.set_vertical_sync_enabled(true);
    window.set_framerate_limit(60);

    let mut memory: Memory = Memory::new(cartridge);
    let mut cpu: Cpu = Cpu::new();
    let mut gpu: Gpu = Gpu::new();

    let mut screen: Texture = Texture::new(160, 144).unwrap();

    let mut steps: u64 = 0;

    let mut debugging;
    loop {
        debugging = steps > 50000;
        // read input
        // if should quit: break loop

        let cycles: u8 = cpu.cycle(&mut memory, debugging);

        let draw = gpu.cycle(&mut memory, cycles);

        if debugging {
            println!("State after {} total steps {} cycles", steps, cycles);
            cpu.print_state();
            //gpu.print_state();
        }

        if draw {
            window.clear(&Color::BLACK);
            screen.update_from_pixels(&gpu.screen_rgba, 160, 144, 0, 0);

            let sprite = Sprite::with_texture(&screen);
            window.draw(&sprite);
            window.display();


            if let Some(event) = window.poll_event() {
                match event {
                    Event::Closed
                    | Event::KeyPressed {
                        code: Key::Escape, ..
                    } => return,
                    _ => {}
                }
            }
        }

        steps += 1;
    }
}
