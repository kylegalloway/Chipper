extern crate rand;
extern crate sdl;

#[macro_use]
extern crate derive_builder;

use cpu::Cpu;
use sdl::event::Event;

pub mod cpu;
pub mod display;
pub mod keypad;

pub fn main(program: String)
{
    let mut cpu = Cpu::new();
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
