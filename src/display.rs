const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    pub memory: [u8; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Self {
            memory: [0; WIDTH * HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.memory.fill(0);
    }
}
