mod cpu;
mod display;

use cpu::Cpu;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
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
    for _ in 0..20 {
        cpu.step();
    }
    cpu.print_state();

    println!("Hello, world!");
}
