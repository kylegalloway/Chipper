mod defs;
// import OpenGL graphics and input


fn main() {
    // Initialize the Chip8 system
    let mut my_chip8 = defs::Chip8::new();

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
