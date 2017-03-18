//! Tests for keypad

extern crate chipper;
extern crate sdl;

use sdl::event::Key;
use chipper::keypad::Keypad;

#[test]
fn test_pressing_keys()
{
	let mut keypad = Keypad::new();
	let keys: [Key; 16] = [
		Key::X, Key::Num1, Key::Num2, Key::Num3,
		Key::Q, Key::W, Key::E, Key::A,
		Key::S, Key::D, Key::Z, Key::C,
		Key::Num4, Key::R, Key::F, Key::V,
		];
	for i in 0..keys.len()
	{
		keypad.press(keys[i], true);
		println!("i: {}", i);
		println!("pressed: {}", keypad.pressed(i));
		assert!(keypad.pressed(i));
		keypad.press(keys[i], false);
		assert_eq!(keypad.pressed(i), false);
	}
}