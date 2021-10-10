//!# Keyboard
//!Implements chip8 keyboard
//!```
//!use super::keyboard::Keyboard;
//!let keyboard = Keyboard::new();
//!
//!```
use super::cfg;

pub struct Keyboard {
    keypad: [bool; cfg::KEYPAD as usize],
}

impl Keyboard {
    ///Initializes Keyboard
    pub fn new() -> Self {
        Keyboard {
            keypad: [false; cfg::KEYPAD as usize],
        }
    }

    ///Executes on Press
    pub fn on_press(&mut self, keymap: &[glfw::Key], key: glfw::Key) {
        let key = self.mapped(keymap, key);
        if key != 999 {
            self.keypad[key] = true;
        }
    }

    ///Executes on Release
    pub fn on_release(&mut self, keymap: &[glfw::Key], key: glfw::Key) {
        let key = self.mapped(keymap, key);
        if key != 999 {
            self.keypad[key] = false;
        }
    }

    ///Check if Pressed
    pub fn is_pressed(&self, key: usize) -> bool {
        self.keypad[key]
    }

    ///Returns the value of pressed key
    pub fn pressed_key(&self) -> usize {
        for (i, on_press) in self.keypad.iter().enumerate() {
            if *on_press {
                return i;
            }
        }
        999
    }

    #[doc(hidden)]
    fn mapped(&self, keymap: &[glfw::Key], key: glfw::Key) -> usize {
        for (i, k) in keymap.iter().enumerate() {
            if key == *k {
                return i;
            }
        }
        999
    }
}
