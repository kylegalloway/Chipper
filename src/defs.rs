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
    // Keypad                   Keyboard
    // +-+-+-+-+                +-+-+-+-+
    // |1|2|3|C|                |1|2|3|4|
    // +-+-+-+-+                +-+-+-+-+
    // |4|5|6|D|    suggested   |Q|W|E|R|
    // +-+-+-+-+       =>       +-+-+-+-+
    // |7|8|9|E|                |A|S|D|F|
    // +-+-+-+-+                +-+-+-+-+
    // |A|0|B|F|                |Z|X|C|V|
    // +-+-+-+-+                +-+-+-+-+
    pub key: [u8; 16],

    pub draw_flag: bool,

    pub chip8_fontset: [u8; 80],
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
            pc: 0,
            delay_timer: 0,
            sound_timer: 0,
            sp: 0,
            draw_flag: false,
            chip8_fontset: [
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80  // F
            ],
        }
    }

    pub fn initialize(&mut self, buffer: [u32]) {
        self.pc     = 0x200;  // Program counter starts at 0x200
        self.opcode = 0;      // Reset current opcode
        self.i      = 0;      // Reset index register
        self.sp     = 0;      // Reset stack pointer

        // Clear display
        self.gfx = [0; 64 * 32];
        // Clear stack
        self.stack = [0; 16];
        // Clear registers V0-VF
        self.v = [0; 16];
        // Clear memory
        self.memory = [0; 4096];

        // Load fontset
        for i in 0..80{
            self.memory[i as usize] = self.chip8_fontset[i as usize];
        }

        // Load the program into memory
        for i in 0..buffer.len() {
            self.memory[(i + 512) as usize] = buffer[i as usize];
        }

        // Reset timers
        self.delay_timer = 0;
        self.sound_timer = 0;

        // Other resets
        self.key = [0; 16];
        self.draw_flag = false;
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

            0x0004 => {
                // 0x8XY4: adds the value of VY to VX.
                // Register VF is set to 1 when there is a carry and set to 0 when there isn’t.
                // Because the register can only store values from 0 to 255 (8 bit value),
                // it means that if the sum of VX and VY is larger than 255,
                // it can’t be stored in the register (or actually it starts counting from 0 again).
                // If the sum of VX and VY is larger than 255, we use the carry flag to let
                // the system know that the total sum of both values was indeed larger than 255.
                // Don’t forget to increment the program counter by two after executing the opcode.
                if self.v[((self.opcode & 0x00F0) >> 4) as usize] >
                   (0xFF - self.v[((self.opcode & 0x0F00) >> 8) as usize]) {
                    self.v[0xF as usize] = 1; //carry
                } else {
                    self.v[0xF as usize] = 0;
                }
                self.v[((self.opcode & 0x0F00) >> 8) as usize] +=
                    self.v[((self.opcode & 0x00F0) >> 4) as usize];
                self.pc += 2;
            }

            0x0033 => {
                // 0xFX33:
                // Stores the Binary-coded decimal representation of VX
                // at the addresses I, I plus 1, and I plus 2
                let shifted_opcode = self.opcode & 0x0F00 >> 8;
                let value = self.v[shifted_opcode as usize];
                self.memory[self.i as usize] = value / 100;
                self.memory[(self.i + 1) as usize] = (value / 10) % 10;
                self.memory[(self.i + 2) as usize] = (value % 100) % 10;
                self.pc += 2;
            }

            0x2000 => {
                // 0x2NNN: calls the subroutine at address NNN.
                // Because we will need to temporarily jump to address NNN,
                // it means that we should store the current address of the pc in the stack.
                // After storing the value of the program counter in the stack,
                // increase the stack pointer to prevent overwriting the current stack.
                // Now that we have stored the program counter, we can set it to the address NNN.
                // Remember, because we’re calling a subroutine at a specific address,
                // you should not increase the program counter by two.
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = self.opcode & 0x0FFF;
            }

            0xA000 => {
                // ANNN: Sets the index register (i) to the address NNN
                self.i = self.opcode & 0x0FFF;
                self.pc += 2;
            }

            0xD000 => {
                // 0xDXYN:
                // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
                // Each row of 8 pixels is read as bit-coded starting from memory location I;
                // I value doesn’t change after the execution of this instruction.
                // VF is set to 1 if any screen pixels are flipped from set to unset
                // when the sprite is drawn, and to 0 if that doesn’t happen.
                // The Chip 8 actually draws on the screen by drawing sprites.
                // It will give us the location of where the sprite needs to be drawn
                // (the opcode tells us which V register we need to check
                // to fetch the X and Y coordinates) and the number of rows (N).
                // The width of each sprite is fixed (8 bits / 1 byte).
                // The state of each pixel is set by using a bitwise XOR operation.
                // This means that it will compare the current pixel state with
                // the current value in the memory.
                // If the current value is different from the value in the memory,
                // the bit value will be 1. If both values match, the bit value will be 0.


                // Fetch the position and height of the sprite
                let x: u16 = self.v[((self.opcode & 0x0F00) >> 8) as usize] as u16;
                let y: u16 = self.v[((self.opcode & 0x00F0) >> 4) as usize] as u16;
                let height: u16 = self.opcode & 0x000F as u16;
                // Pixel value
                let mut pixel: u16;

                // Reset register VF
                self.v[0xF as usize] = 0;
                // Loop over each row
                for yline in 0..height {
                    // Fetch the pixel value from the memory starting at location I
                    pixel = self.memory[(self.i + yline) as usize] as u16;
                    // Loop over 8 bits of one row
                    for xline in 0..8 {
                        // Check if the current evaluated pixel is set to 1
                        // (note that 0x80 >> xline scan through the byte, one bit at the time)
                        if (pixel & (0x80 >> xline)) != 0 {
                            // Check if the pixel on the display is set to 1.
                            if self.gfx[(x + xline + ((y + yline) * 64)) as usize] == 1 {
                                // If it is set, we need to register the collision by
                                // setting the VF register
                                self.v[0xF as usize] = 1;
                            }
                            // Set the pixel value by using XOR
                            self.gfx[(x + xline + ((y + yline) * 64)) as usize] ^= 1;
                        }
                    }
                }

                // We changed our gfx[] array and thus need to update the screen.
                self.draw_flag = true;
                // Update the program counter to move to the next opcode
                self.pc += 2;
            }


            0xE000=> {
                // Every cycle you should check the key input state and store it in key[].
                // It actually doesn’t matter what value you store,
                // because opcode 0xEX9E and 0xEXA1 only check if a certain key is or isn’t pressed.
                // Opcode 0xFX0A only waits for a key press, and when it receives one,
                // it stores the key name in the register and not the key state.
                match self.opcode & 0x00FF {
                    // EX9E: Skips the next instruction
                    // if the key stored in VX is pressed
                    0x009E => {
                        if self.key[self.v[((self.opcode & 0x0F00) >> 8) as usize] as usize] != 0 {
                            self.pc += 4;
                        }
                        else {
                            self.pc += 2;
                        }
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
