extern crate gameboy;

use std::fs::File;
use std::io::Read;

use gameboy::gameboy::GameBoy;
use gameboy::gpu::{GB_HSIZE, GB_VSIZE};
use gameboy::input::Button;

use clap::{App, Arg};
use minifb::{Key, Window, WindowOptions};

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("gb-rust")
        .version("1.0")
        .about("Gameboy emulator in Rust!")
        .author("A. Casey")
        .arg(
            Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely"),
        )
        .arg(
            Arg::with_name("watch start")
                .long("watch-start")
                .help("Watch Address Start"),
        )
        .arg(
            Arg::with_name("watch end")
                .long("watch-end")
                .help("Watch Address End"),
        )
        .arg(
            Arg::with_name("panic-at-pc")
                .long("panic-at-pc")
                .help("Panic at given Program Counter value")
                .default_value("FFFF"),
        )
        .arg(Arg::with_name("INPUT").help("Input Gameboy file").index(1))
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap_or("tetris.gb");

    let watch_start =
        u16::from_str_radix(matches.value_of("watch-start").unwrap_or("FF0F"), 16).unwrap();
    let watch_end =
        u16::from_str_radix(matches.value_of("watch-end").unwrap_or("FFFF"), 16).unwrap();

    let mut file = File::open(filename)?;
    let mut rom_contents = Vec::new();
    file.read_to_end(&mut rom_contents)?;

    let mut gb = GameBoy::new(rom_contents);

    println!("Loaded rom: {:?}", gb.title());

    let mut window = Window::new(
        "Rust Gameboy",
        GB_HSIZE,
        GB_VSIZE,
        WindowOptions {
            scale: minifb::Scale::X2,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut screen_rgba: Vec<u8> = vec![255; GB_HSIZE * GB_VSIZE * 4];

    let mut debugging = matches.is_present("debug");

    let pc_panic = u16::from_str_radix(matches.value_of("panic-at-pc").unwrap(), 16).unwrap();

    println!("Will panic when pc == {:X?}", pc_panic);

    loop {
        let drawn = gb.cycle(debugging, Some(pc_panic));

        if drawn {
            screen_rgba.copy_from_slice(gb.buffer_vec());

            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window
                .update_with_buffer(
                    unsafe {
                        std::slice::from_raw_parts(
                            screen_rgba.as_ptr() as *const u32,
                            screen_rgba.len() / 4,
                        )
                    },
                    GB_HSIZE,
                    GB_VSIZE,
                )
                .unwrap();

            for keyset in window.get_keys_pressed(minifb::KeyRepeat::No) {
                for k in keyset {
                    match k {
                        Key::Escape => return Ok(()),
                        Key::D => debugging = true,
                        Key::E => debugging = false,
                        Key::W => println!("{:?}", gb.read_region(watch_start, watch_end)),
                        Key::G => println!("{:?}", gb.gpu_trace()),
                        Key::A => gb.input().set_input(Button::A, true),
                        Key::Z => gb.input().set_input(Button::B, true),
                        Key::M => gb.input().set_input(Button::Start, true),
                        Key::N => gb.input().set_input(Button::Select, true),
                        Key::Up => gb.input().set_input(Button::Up, true),
                        Key::Down => gb.input().set_input(Button::Down, true),
                        Key::Left => gb.input().set_input(Button::Left, true),
                        Key::Right => gb.input().set_input(Button::Right, true),
                        _ => (),
                    }
                }
            }

            for keyset in window.get_keys_released() {
                for k in keyset {
                    match k {
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
                }
            }

            if ! window.is_open() {
                return Ok(());
            }
        }
    }
    /*
        if let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return Ok(()),
                Event::KeyPressed { code, .. } => match code {
                    Key::Escape => return Ok(()),
                    Key::D => debugging = true,
                    Key::E => debugging = false,
                    Key::W => println!("{:?}", gb.read_region(watch_start, watch_end)),
                    Key::G => println!("{:?}", gb.gpu_trace()),
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
                },
                _ => {}
            }
        }
    } */
}
