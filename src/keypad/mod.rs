use sdl2::keyboard::Keycode as Key;
use sdl2::keyboard::Keycode::*;

mod test;

#[derive(Default, Debug)]
pub struct Keypad
{
    keys: [bool; 16],
}

impl Keypad
{
    pub fn pressed(&mut self, index: usize) -> bool
    {
        self.keys[index]
    }

    pub fn press(&mut self, key: Key, state: bool)
    {
        match key
        {
            Num1 => self.set_key(0x1, state),
            Num2 => self.set_key(0x2, state),
            Num3 => self.set_key(0x3, state),
            Num4 => self.set_key(0xc, state),
            Q => self.set_key(0x4, state),
            W => self.set_key(0x5, state),
            E => self.set_key(0x6, state),
            R => self.set_key(0xd, state),
            A => self.set_key(0x7, state),
            S => self.set_key(0x8, state),
            D => self.set_key(0x9, state),
            F => self.set_key(0xe, state),
            Z => self.set_key(0xa, state),
            X => self.set_key(0x0, state),
            C => self.set_key(0xb, state),
            V => self.set_key(0xf, state),
            _ => (),
        }
    }

    fn set_key(&mut self, index: usize, state: bool)
    {
        self.keys[index] = state;
    }
}