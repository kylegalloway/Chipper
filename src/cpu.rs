use std::fs::File;
use std::path::Path;
use std::io::Read;
// use std::rand;
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
    pub display: Display
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
            display: Display::new()
        };

        for i in 0..80 {
            cpu.memory[i] = fontset[i];
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
        let mut reader = File::open(&path).unwrap();
        // KG fixme: problem is here.
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
                    },
                    Err(e) => {panic!("{:?}", e)}
                }
            },
            None => self.pc = 0x200,
        };
    }

    fn fetch_opcode(&mut self) {
        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
    }

    fn opcode_execute(&mut self) {
        match (self.opcode & 0xf000) {
            // 0x0000 => self.op_0xxx(),
            // 0x1000 => self.op_1xxx(),
            // 0x2000 => self.op_2xxx(),
            // 0x3000 => self.op_3xxx(),
            // 0x4000 => self.op_4xxx(),
            // 0x5000 => self.op_5xxx(),
            // 0x6000 => self.op_6xxx(),
            // 0x7000 => self.op_7xxx(),
            // 0x8000 => self.op_8xxx(),
            // 0x9000 => self.op_9xxx(),
            // 0xA000 => self.op_Axxx(),
            // 0xB000 => self.op_Bxxx(),
            // 0xC000 => self.op_Cxxx(),
            // 0xD000 => self.op_Dxxx(),
            // 0xE000 => self.op_Exxx(),
            // 0xF000 => self.op_Fxxx(),
            _ => not_implemented(self.opcode as usize, self.pc),
        }
    }
}

fn not_implemented(op: usize, pc: usize) {
    println!("Not implemented:: op: {:x}, pc: {:x}", op, pc)
}

static fontset: [u8; 80] =
    [0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0,
     0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0,
     0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0,
     0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0,
     0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0,
     0xF0, 0x80, 0xF0, 0x80, 0x80];
