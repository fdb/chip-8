// The computers which originally used the Chip-8 Language
// had a 16-key hexadecimal keypad with the following layout:
// 1  2  3  C
// 4  5  6  D
// 7  8  9  E
// A  0  B  F

pub struct Keypad {
    pub keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn reset(&mut self) {
        self.keys.fill(false);
    }

    pub fn set_key(&mut self, key: usize, state: bool) {
        self.keys[key] = state
    }

    pub fn is_pressed(&self, key: usize) -> bool {
        self.keys[key]
    }
}
