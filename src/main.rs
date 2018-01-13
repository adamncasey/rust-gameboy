mod rom;
mod memory;
mod cpu;
mod instruction;
mod math;
mod gpu;
mod opcode;
mod gameboy;

use gpu::{GB_HSIZE, GB_VSIZE};
use gameboy::GameBoy;

extern crate sfml;
use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture};
use sfml::window::{Event, Key, Style};

fn main() {
    let mut gb = GameBoy::new("tetris.gb");

    println!("Loaded rom: {:?}", gb.title());

    let mut window = RenderWindow::new(
        (GB_HSIZE as u32, GB_VSIZE as u32),
        "gb-rust",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    window.set_framerate_limit(60);

    let mut screen: Texture = Texture::new(GB_HSIZE as u32, GB_VSIZE as u32).unwrap();
    let mut screen_rgba: Vec<u8> = vec![255; GB_HSIZE * GB_VSIZE * 4];

    let mut debugging = true;
    loop {
        let drawn = gb.cycle(&mut screen_rgba, debugging);

        if drawn {
            window.clear(&Color::BLACK);
            screen.update_from_pixels(&screen_rgba, GB_HSIZE as u32, GB_VSIZE as u32, 0, 0);

            let sprite = Sprite::with_texture(&screen);
            window.draw(&sprite);
            window.display();
        }

        if let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                Event::KeyPressed { code, .. } => match code {
                    Key::Escape => return,
                    Key::D => debugging = true,
                    Key::E => debugging = false,
                    _ => (),
                },
                _ => {}
            }
        }
    }
}
