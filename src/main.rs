mod rom;
mod memory;
mod cpu;
mod instruction;
mod math;
mod gpu;
mod opcode;
mod gameboy;
mod input;

use gpu::{GB_HSIZE, GB_VSIZE};
use gameboy::GameBoy;
use input::Button;

extern crate sfml;
use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture, Transformable};
use sfml::window::{Event, Key, Style};
use sfml::system::{Vector2f};

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("gb-rust")
        .version("1.0")
        .about("Gameboy emulator in Rust!")
        .author("A. Casey")
        .arg(
            Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely"),
        )
        .arg(Arg::with_name("INPUT").help("Input Gameboy file").index(1))
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap_or("tetris.gb");

    let mut gb = GameBoy::new(filename);

    println!("Loaded rom: {:?}", gb.title());

    let mut window = RenderWindow::new(
        (GB_HSIZE as u32 *3, GB_VSIZE as u32 *3),
        "gb-rust",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    window.set_framerate_limit(60);

    let mut screen: Texture = Texture::new(GB_HSIZE as u32, GB_VSIZE as u32).unwrap();
    let mut screen_rgba: Vec<u8> = vec![255; GB_HSIZE * GB_VSIZE * 4];

    let mut debugging = matches.is_present("debug");
    loop {
        let drawn = gb.cycle(&mut screen_rgba, debugging);

        if drawn {
            window.clear(&Color::BLACK);
            screen.update_from_pixels(&screen_rgba, GB_HSIZE as u32, GB_VSIZE as u32, 0, 0);

            let mut sprite: Sprite = Sprite::with_texture(&screen);
            sprite.set_scale(Vector2f::new(3.0, 3.0));
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
                    Key::A => gb.input().set_input(Button::A, true),
                    Key::Z => gb.input().set_input(Button::B, true),
                    Key::M => gb.input().set_input(Button::Start, true),
                    Key::N => gb.input().set_input(Button::Select, true),
                    Key::Up => gb.input().set_input(Button::Up, true),
                    Key::Down => gb.input().set_input(Button::Down, true),
                    Key::Left => gb.input().set_input(Button::Left, true),
                    Key::Right => gb.input().set_input(Button::Right, true),
                    _ => (),
                },
                Event::KeyReleased { code, .. } => match code {
                    Key::A => gb.input().set_input(Button::A, false),
                    Key::Z => gb.input().set_input(Button::B, false),
                    Key::M => gb.input().set_input(Button::Start, false),
                    Key::N => gb.input().set_input(Button::Select, false),
                    Key::Up => gb.input().set_input(Button::Up, false),
                    Key::Down => gb.input().set_input(Button::Down, false),
                    Key::Left => gb.input().set_input(Button::Left, false),
                    Key::Right => gb.input().set_input(Button::Right, false),
                    _ => (),
                }
                _ => {}
            }
        }
    }
}
