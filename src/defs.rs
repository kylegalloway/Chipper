pub struct Chip8 {
    // The systems memory map:
    // 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
    // 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    // 0x200-0xFFF - Program ROM and work RAM
    //
    // The Chip 8 has 35 opcodes which are all two bytes long.
    opcode: i16,

    // The Chip 8 has 4K memory in total.
    memory: [i8; 4096],

    // CPU registers:
    // The Chip 8 has 15 8-bit general purpose registers named V0,V1 up to VE.
    // The 16th register is used  for the ‘carry flag’.
    v: [i8; 16],

    // There is an Index register I and program counter (pc),
    // which can have a value from 0x000 to 0xFFF.
    i: i16,
    pc: i16,

    // The graphics system:
    // The chip 8 has one instruction that draws sprite to the screen.
    // Drawing is done in XOR mode and if a pixel is turned off as a result of drawing,
    // the VF register is set. This is used for collision detection.
    // The graphics of the Chip 8 are black and white,
    // and the screen has a total of 2048 pixels (64 x 32).
    // This can easily be implemented using an array that hold the pixel state (1 or 0).
    gfx: [i8; 64 * 32],

    // The delay timer is active whenever the delay timer register (DT) is non-zero.
    // This timer does nothing more than subtract 1 from the value of DT at a rate of 60Hz.
    // When DT reaches 0, it deactivates.
    delay_timer: i8,

    // The sound timer is active whenever the sound timer register (ST) is non-zero.
    // This timer also decrements at a rate of 60Hz, however,
    // as long as ST's value is greater than zero, the Chip-8 buzzer will sound.
    // When ST reaches zero, the sound timer deactivates.
    sound_timer: i8,

    // It is important to know that the Chip 8 instruction set has opcodes
    // that allow the program to jump to a certain address or call a subroutine.
    // The system has 16 levels of stack and in order to remember which level of the stack is used,
    // you need to implement a stack pointer (sp).
    stack: [i16; 16],
    sp: i16,

    // Finally, the Chip 8 has a HEX based keypad (0x0-0xF),
    // you can use an array to store the current state of the key.
    key: [i8; 16],
}
