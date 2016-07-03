pub struct Chip8 {
    // The systems memory map:
    // 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
    // 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    // 0x200-0xFFF - Program ROM and work RAM
    //
    // The Chip 8 has 35 opcodes which are all two bytes long.
    pub opcode: u16,

    // The Chip 8 has 4K memory in total.
    pub memory: [u8; 4096],

    // CPU registers:
    // The Chip 8 has 15 8-bit general purpose registers named V0,V1 up to VE.
    // The 16th register is used  for the ‘carry flag’.
    pub v: [u8; 16],

    // There is an Index register I and program counter (pc),
    // which can have a value from 0x000 to 0xFFF.
    pub i: u16,
    pub pc: u16,

    // The graphics system:
    // The chip 8 has one instruction that draws sprite to the screen.
    // Drawing is done in XOR mode and if a pixel is turned off as a result of drawing,
    // the VF register is set. This is used for collision detection.
    // The graphics of the Chip 8 are black and white,
    // and the screen has a total of 2048 pixels (64 x 32).
    // This can easily be implemented using an array that hold the pixel state (1 or 0).
    pub gfx: [u8; 64 * 32],

    // The delay timer is active whenever the delay timer register (DT) is non-zero.
    // This timer does nothing more than subtract 1 from the value of DT at a rate of 60Hz.
    // When DT reaches 0, it deactivates.
    pub delay_timer: u8,

    // The sound timer is active whenever the sound timer register (ST) is non-zero.
    // This timer also decrements at a rate of 60Hz, however,
    // as long as ST's value is greater than zero, the Chip-8 buzzer will sound.
    // When ST reaches zero, the sound timer deactivates.
    pub sound_timer: u8,

    // It is important to know that the Chip 8 instruction set has opcodes
    // that allow the program to jump to a certain address or call a subroutine.
    // The system has 16 levels of stack and in order to remember which level of the stack is used,
    // you need to implement a stack pointer (sp).
    pub stack: [u16; 16],
    pub sp: u16,

    // Finally, the Chip 8 has a HEX based keypad (0x0-0xF),
    // you can use an array to store the current state of the key.
    pub key: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Chip8 {
        // Initialize registers and memory once
        Chip8 {
            memory: [0; 4096],
            v: [0; 16],
            gfx: [0; 64 * 32],
            stack: [0; 16],
            key: [0; 16],
            opcode: 0,
            i: 0,
            pc: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            sp: 0,
        }
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch Opcode
        let current_pc = self.pc as usize;
        let high_bits = ((self.memory[current_pc]) as u16) << 8;
        let low_bits = (self.memory[current_pc + 1]) as u16;
        self.opcode = high_bits | low_bits;

        // Decode Opcode
        match self.opcode & 0xF000 {
            // Execute Opcode
            0xA000 => {
                self.i = self.opcode & 0x0FFF;
                self.pc += 2;
            }

            0x0000 => {
                match self.opcode & 0x000F {
                    0x0000 => {
                        // 0x00E0: Clears the screen
                        println!("Clears Screen");
                        // Execute opcode
                    }

                    0x000E => {
                        // 0x00EE: Returns from subroutine
                        println!("Returns from subroutine");
                        // Execute opcode
                    }
                    x => println!("ERROR: opcode 0x{:X} undefined.", x),
                }
            }
            x => println!("ERROR: opcode 0x{:X} undefined.", x),
        }

        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            self.sound_timer -= 1;
        }
    }
}
