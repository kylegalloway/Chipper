// use std::io;
// use std::io::prelude::*;
use std::fs::File;
use bit_reader::BitReader;

mod defs;
mod bit_reader;
// import OpenGL graphics and input


fn main() {
    use std::env::args;

    let cmdline_args : Vec<String> = args().skip(1).collect();

    if cmdline_args.len() == 0 {
        println!("Usage: chipper <filename>");
        return;
    }

    println!("Loading: {}", &cmdline_args[0]);
    let fname = &cmdline_args[0];

    use std::io::{Error, ErrorKind};

    let mut f = try!(File::open(fname));
    let mut buffer: [u32; 4096] = [0; 4096];
    try!(f.read(&mut buffer));

    let mut my_chip8 = defs::Chip8::new();

    // Initialize the Chip8 system
    my_chip8.initialize(buffer);

    // Set up render system and register input callbacks
    // setupGraphics();
    // setupInput();

    // Load the game into the memory.
    // my_chip8.loadGame("pong"); // TODO change to command line argument

    // Emulation loop
    'GameLoop: loop {
        // Emulate one cycle
        my_chip8.emulate_cycle();

        // If the draw flag is set, update the screen
        // if(my_chip8.drawFlag)
        //   drawGraphics();

        // Store key press state (Press and Release)
        // my_chip8.setKeys();
    }
}
