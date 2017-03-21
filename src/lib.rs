extern crate rand;
extern crate sdl2;

#[macro_use]
extern crate derive_builder;

use cpu::Cpu;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Renderer;

pub mod cpu;
pub mod display;
pub mod keypad;

fn init<'a>() -> (Renderer<'a>, EventPump)
{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Chipper", 400, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let event_pump = sdl_context.event_pump().unwrap();

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();

    (renderer, event_pump)
}

pub fn main(program: String)
{
    let (mut renderer, mut events) = init();
    let mut cpu = Cpu::new(&mut renderer);
    cpu.load_game(program);


    'main: loop
    {
        'event: loop
        {
            for event in events.poll_iter()
            {
                match event
                {
                    Event::Quit { .. } => break 'main,
                    Event::KeyDown { keycode, .. } => cpu.keypad.press(keycode.unwrap(), true),
                    Event::KeyUp { keycode, .. } => cpu.keypad.press(keycode.unwrap(), false),
                    _ => break 'event,
                }
            }
        }

        cpu.emulate_cycle();
        cpu.display.draw_screen();
    }
}
