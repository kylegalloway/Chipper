extern crate clap;
extern crate rand;
extern crate sdl;

#[macro_use]
extern crate derive_builder;

use clap::{App, Arg};
use cpu::Cpu;
use sdl::event::Event;

pub mod cpu;
pub mod display;
pub mod keypad;

pub fn main()
{
    let mut cpu = Cpu::new();

    let args = App::new("Chipper")
        .about("Runs chip-8 programs from the programs directory")
        .version("0.1.0")
        .author("Kyle Galloway")
        .arg(Arg::with_name("file")
                 .help("the program file to use; i.e. programs/<file>")
                 .index(1)
                 .required(true)
                 .short("f")
                 .long("file"))
        .get_matches();

    let program = format!("programs/{}", args.value_of("file").unwrap());

    cpu.load_game(program);

    sdl::init(&[sdl::InitFlag::Video, sdl::InitFlag::Audio, sdl::InitFlag::Timer]);

    'main: loop
    {
        'event: loop
        {
            match sdl::event::poll_event()
            {
                Event::Quit => break 'main,
                Event::None => break 'event,
                Event::Key(key, state, _, _) => cpu.keypad.press(key, state),
                _ =>
                {}
            }
        }

        cpu.emulate_cycle();
        cpu.display.draw_screen();
    }

    sdl::quit();
}
