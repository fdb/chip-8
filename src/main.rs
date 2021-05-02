mod cpu;
mod display;
mod keypad;

use cpu::Cpu;
use minifb::{Key, Window, WindowOptions};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const BUFFER_SCALE: usize = 8;
const WINDOW_WIDTH: usize = display::WIDTH * BUFFER_SCALE;
const WINDOW_HEIGHT: usize = display::HEIGHT * BUFFER_SCALE;

fn draw_dot(window_buffer: &mut Vec<u32>, x: usize, y: usize, color: u32) {
    for dy in 0..BUFFER_SCALE {
        for dx in 0..BUFFER_SCALE {
            let offset = (y * BUFFER_SCALE + dy) * WINDOW_WIDTH + (x * BUFFER_SCALE + dx);
            window_buffer[offset] = color;
        }
    }
}

fn main() {
    let mut window_buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    // 1  2  3  4
    // Q  W  E  R
    // A  S  D  F
    // Z  X  C  V
    let mut key_map: HashMap<Key, usize> = HashMap::new();
    key_map.insert(Key::Key1, 0x0);
    key_map.insert(Key::Key2, 0x1);
    key_map.insert(Key::Key3, 0x2);
    key_map.insert(Key::Key4, 0xC);
    key_map.insert(Key::Q, 0x4);
    key_map.insert(Key::W, 0x5);
    key_map.insert(Key::E, 0x6);
    key_map.insert(Key::R, 0xD);
    key_map.insert(Key::A, 0x7);
    key_map.insert(Key::S, 0x8);
    key_map.insert(Key::D, 0x9);
    key_map.insert(Key::F, 0xE);
    key_map.insert(Key::Z, 0xA);
    key_map.insert(Key::X, 0x0);
    key_map.insert(Key::C, 0xB);
    key_map.insert(Key::V, 0xF);

    let mut window = Window::new(
        "Test - ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut cpu = Cpu::new();
    let path = Path::new("programs/cavern/cavern.ch8");
    let mut file = match File::open(&path) {
        Err(err) => panic!("Could not open {:?}: {}", path, err),
        Ok(file) => file,
    };
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Err(err) => panic!("Could not read {:?}: {}", path, err),
        Ok(_) => {}
    }

    // Copy the buffer into the CHIP-8's memory.
    for i in 0..buffer.len() {
        cpu.memory[i + 0x200] = buffer[i];
    }
    cpu.reset();
    // cpu.print_state();
    // cpu.display.print();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (k, v) in &key_map {
            cpu.keypad.set_key(*v, window.is_key_down(*k));
        }
        cpu.step();
        for y in 0..display::HEIGHT {
            for x in 0..display::WIDTH {
                let color: u32 = if cpu.display.get_pixel(x, y) {
                    0xFF00FFFF
                } else {
                    0x0
                };
                draw_dot(&mut window_buffer, x, y, color);
            }
        }
        // }
        //     for i in buffer.iter_mut() {
        //     *i = 0; // write something more funny here!
        // }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&window_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}
