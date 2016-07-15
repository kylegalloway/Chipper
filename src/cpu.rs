use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::process;
use rand;
use std::env;

use display::Display;
use keypad::Keypad;

pub struct Cpu {
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    i: usize,
    pc: usize,
    stack: [u16; 16],
    sp: usize,
    delay_timer: u8,
    sound_timer: u8,
    pub keypad: Keypad,
    pub display: Display,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],
            i: 0x200,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: Keypad::new(),
            display: Display::new(),
        };

        for i in 0..80 {
            cpu.memory[i] = FONTSET[i];
        }
        cpu
    }

    pub fn emulate_cycle(&mut self) {
        self.fetch_opcode();
        self.opcode_execute();

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!\n");
            }
            self.sound_timer -= 1;
        }

        for _ in 0..10000 {
        }
    }

    pub fn load_game(&mut self, program: String) {
        let mut path = env::current_dir().unwrap();
        path.push(program.trim());
        let display = path.display();
        let mut reader = match File::open(&path) {
            Err(why) => panic!("Couldn't open {}: {}", display, why),
            Ok(reader) => reader,
        };
        self.load_to_memory(&mut reader);
    }

    fn load_to_memory(&mut self, reader: &mut File) {
        match reader.bytes().next() {
            Some(value) => {
                match value {
                    Ok(value) => {
                        self.memory[self.pc] = value;
                        self.pc += 1;
                        self.load_to_memory(reader)
                    }
                    Err(e) => panic!("{:?}", e),
                }
            }
            None => self.pc = 0x200,
        };
    }

    fn fetch_opcode(&mut self) {
        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
    }

    fn opcode_execute(&mut self) {
        match (self.opcode & 0xf000) {
            0x0000 => self.op_0xxx(),
            0x1000 => self.op_1xxx(),
            0x2000 => self.op_2xxx(),
            0x3000 => self.op_3xxx(),
            0x4000 => self.op_4xxx(),
            // 0x5000 => self.op_5xxx(),
            0x6000 => self.op_6xxx(),
            0x7000 => self.op_7xxx(),
            0x8000 => self.op_8xxx(),
            // 0x9000 => self.op_9xxx(),
            0xA000 => self.op_Axxx(),
            // 0xB000 => self.op_Bxxx(),
            0xC000 => self.op_Cxxx(),
            0xD000 => self.op_Dxxx(),
            0xE000 => self.op_Exxx(),
            0xF000 => self.op_Fxxx(),
            _ => not_implemented(self.opcode as usize, self.pc),
        }
    }

    fn op_0xxx(&mut self) {
        match (self.opcode & 0x000F) {
            0x0000 => {
                self.display.clear();
            }
            0x000E => {
                self.sp -= 1;
                self.pc = self.stack[self.sp] as usize;
            }
            _ => not_implemented(self.opcode as usize, self.pc),
        }
        self.pc += 2;
    }

    fn op_1xxx(&mut self) {
        self.pc = self.op_nnn() as usize;
    }

    fn op_2xxx(&mut self) {
        self.stack[self.sp] = self.pc as u16;
        self.sp += 1;
        self.pc = self.op_nnn() as usize;
    }

    fn op_3xxx(&mut self) {
        if self.v[self.op_x()] == self.op_nn() {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn op_4xxx(&mut self) {
        if self.v[self.op_x()] != self.op_nn() {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn op_5xxx(&mut self) {}

    fn op_6xxx(&mut self) {
        self.v[self.op_x()] = self.op_nn();
        self.pc += 2;
    }

    fn op_7xxx(&mut self) {
        self.v[self.op_x()] += self.op_nn();
        self.pc += 2;
    }

    fn op_8xxx(&mut self) {
        match (self.opcode & 0x000F) {
            0x0 => {
                self.v[self.op_x()] = self.v[self.op_y()];
            }
            0x1 => {
                self.v[self.op_x()] |= self.v[self.op_y()];
            }
            0x2 => {
                self.v[self.op_x()] &= self.v[self.op_y()];
            }
            0x3 => {
                self.v[self.op_x()] ^= self.v[self.op_y()];
            }
            0x4 => {
                let (result, overflow) = self.v[self.op_x()].overflowing_add(self.v[self.op_y()]);
                self.v[self.op_x()] = result;
                self.v[15] = overflow as u8;
            }
            0x5 => {
                let (result, overflow) = self.v[self.op_x()].overflowing_sub(self.v[self.op_y()]);
                self.v[self.op_x()] = result;
                self.v[15] = overflow as u8;
            }
            // 0x6 => {}
            // 0x7 => {}
            // 0xE => {}
            _ => not_implemented(self.opcode as usize, self.pc),
        }
        self.pc += 2;
    }

    fn op_9xxx(&mut self) {}

    fn op_Axxx(&mut self) {
        self.i = self.op_nnn() as usize;
        self.pc += 2;
    }

    fn op_Bxxx(&mut self) {}

    fn op_Cxxx(&mut self) {
        self.v[self.op_x()] = self.op_nn() & rand::random::<u8>();
        self.pc += 2;
    }

    fn op_Dxxx(&mut self) {
        let x = self.v[self.op_x()];
        let y = self.v[self.op_y()];
        let n = self.op_n();
        let from = self.i;
        let to = from + n as usize;
        self.v[15] = self.display.draw(x as usize, y as usize, &self.memory[from..to]);
        self.pc += 2;
    }

    fn op_Exxx(&mut self) {
        let v = self.v[self.op_x()] as usize;
        let skip = match (self.opcode & 0x00FF) {
            0x9E => {
                if self.keypad.pressed(v) {
                    4
                } else {
                    2
                }
            }
            0xA1 => {
                if !self.keypad.pressed(v) {
                    4
                } else {
                    2
                }
            }
            _ => 2,
        };
        self.pc += skip;
    }

    fn op_Fxxx(&mut self) {
        match (self.opcode & 0x00FF) {
            0x07 => {
                self.v[self.op_x()] = self.delay_timer;
            }
            0x15 => {
                self.delay_timer = self.v[self.op_x()];
            }
            0x18 => {
                self.sound_timer = self.v[self.op_x()];
            }
            0x29 => {
                self.i = self.v[self.op_x()] as usize * 5;
            }
            0x33 => {
                self.memory[self.i] = self.v[self.op_x()] / 100;
                self.memory[self.i + 1] = (self.v[self.op_x()] / 10) % 10;
                self.memory[self.i + 2] = (self.v[self.op_x()] % 100) % 10;
            }
            0x65 => {
                let x = self.op_x() + 1;
                for index in 0..x {
                    self.v[index] = self.memory[self.i + index];
                }
                self.i += x;
            }
            _ => not_implemented(self.opcode as usize, self.pc),
        }
        self.pc += 2;
    }

    // returns the x part of an opcode
    fn op_x(&self) -> usize {
        ((self.opcode & 0x0F00) >> 8) as usize
    }
    // returns the y part of an opcode
    fn op_y(&self) -> usize {
        ((self.opcode & 0x00F0) >> 4) as usize
    }
    // returns the n part of an opcode
    fn op_n(&self) -> u8 {
        (self.opcode & 0x000F) as u8
    }
    // returns the nn part of an opcode
    fn op_nn(&self) -> u8 {
        (self.opcode & 0x00FF) as u8
    }
    // returns the nnn part of an opcode
    fn op_nnn(&self) -> u16 {
        self.opcode & 0x0FFF
    }
}

fn not_implemented(op: usize, pc: usize) {
    println!("**************************************************************");
    println!("Not implemented:: op: {:x}, pc: {:x}", op, pc);
    println!("**************************************************************");
    process::exit(1);
}

static FONTSET: [u8; 80] =
    [0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0,
     0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0,
     0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0,
     0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0,
     0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0,
     0xF0, 0x80, 0xF0, 0x80, 0x80];
