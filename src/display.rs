pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

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

    pub fn set_pixel(&mut self, x: usize, y: usize, on: u8) {
        self.memory[y * WIDTH + x] = on;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.memory[y * WIDTH + x] == 1
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: u8) -> bool {
        for dx in 0..8 {
            let bit = (sprite >> (7 - dx)) & 0x1;
            self.set_pixel(x + dx, y, bit);
        }
        true
    }

    pub fn print(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.get_pixel(x, y) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
