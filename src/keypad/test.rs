#[cfg(test)]
mod test {
    use super::super::*;
    use sdl2::keyboard::Keycode;

    #[test]
    fn test_default()
    {
        let mut keypad = Keypad::default();
        // Ordered based on order in file
        let keys: [Keycode; 16] = [X, Num1, Num2, Num3, Q, W, E, A, S, D, Z, C, Num4, R, F, V];

        for i in 0..keys.len()
        {
            assert_eq!(keypad.pressed(i), false);
        }
    }

    #[test]
    fn test_pressing_keys()
    {
        let mut keypad = Keypad::default();
        // Ordered based on order in file
        let keys: [Keycode; 16] = [X, Num1, Num2, Num3, Q, W, E, A, S, D, Z, C, Num4, R, F, V];

        for i in 0..keys.len()
        {
            keypad.press(keys[i], true);
            assert!(keypad.pressed(i));
            
            keypad.press(keys[i], false);
            assert_eq!(keypad.pressed(i), false);
        }
    }
}