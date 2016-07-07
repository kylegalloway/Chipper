extern crate sdl;

use cpu::Cpu;
use std::env;
use sdl::event::Event;
use std::io::{self, BufRead};

mod cpu;
mod display;
mod keypad;

fn main() {
    let mut cpu = Cpu::new();


    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let input = &args[1];
        let program = format!("programs/{}", input);

        cpu.load_game(program);


        sdl::init(&[sdl::InitFlag::Video, sdl::InitFlag::Audio, sdl::InitFlag::Timer]);

        'main: loop {
            'event : loop {
                match sdl::event::poll_event() {
                    Event::Quit                  => break 'main,
                    Event::None                  => break 'event,
                    Event::Key(key, state, _, _) => cpu.keypad.press(key, state),
                    _                            => {}
                }
            }

            cpu.emulate_cycle();
            cpu.display.draw_screen();
        }

        sdl::quit();
    }
    else {
        println!("Please give the program name as the only command line argument.");
    }
}
