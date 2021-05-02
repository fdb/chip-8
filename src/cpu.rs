use crate::Display;

pub struct Cpu {
    // Program counter
    pub pc: u16,
    // Registers
    pub v: [u8; 16],
    // Index register:
    pub i: u16,
    // Stack
    pub stack: [u16; 16],
    // Stack pointer
    pub sp: u8,
    // Memory
    pub memory: [u8; 4096],
    // Display
    pub display: Display,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            memory: [0; 4096],
            pc: 0x0,
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            sp: 0,
            display: Display::new(),
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0x0200;
        self.v.fill(0);
        self.i = 0;
        self.stack.fill(0);
        self.sp = 0;
    }

    pub fn step(&mut self) {
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[(self.pc + 1) as usize] as u16);

        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let x = op_2 as usize;
        let y = op_3 as usize;
        let vx = self.v[x];
        let vy = self.v[y];
        let n = op_4 as u8;

        println!("[{:04X}] op: {:04X}", self.pc, opcode);

        // Match on the op and execute the instruction.
        match (op_1, op_2, op_3, op_4) {
            // 00E0 - CLS
            // Clear the display.
            (0x0, 0x0, 0xE, 0x0) => {
                self.display.clear();
                self.pc += 2;
            }
            // 00EE - RET
            // Return from a subroutine.
            (0x0, 0x0, 0xE, 0xE) => {
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            }
            // 1nnn - JP addr
            // Jump to location nnn.
            (0x1, _, _, _) => {
                self.pc = nnn;
            }
            // 2nnn - CALL addr
            // Call subroutine at nnn.
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                if self.sp >= 16 {
                    println!("Stack pointer is too high.");
                }
                self.pc = nnn;
            }

            // 4xkk - SNE Vx, byte
            // Skip next instruction if Vx != kk.
            (0x4, _, _, _) => {
                if vx != kk {
                    self.pc += 2;
                }
                self.pc += 2;
            }

            // 6xkk - LD Vx, byte
            // Set Vx = kk.
            (0x6, _, _, _) => {
                self.v[x] = kk;
                self.pc += 2;
            }

            // 8xy0 - LD Vx, Vy
            // Set Vx = Vy.
            (0x8, _, _, 0x0) => {
                self.v[x] = vy;
                self.pc += 2;
            }

            // Annn - LD I, addr
            // Set I = nnn.
            (0xA, _, _, _) => {
                self.i = nnn;
                self.pc += 2;
            }

            // Dxyn - DRW Vx, Vy, nibble
            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            (0xD, _, _, _) => {
                
            }

            _ => println!("[{:04X}] - Unknown op: {:04X}", self.pc, opcode),
        }
    }

    pub fn print_state(&self) {
        println!("PC {:04X}", self.pc);
        for i in 0..4 {
            println!(
                "V{:02}: {:02X}      V{:02}: {:02X}      V{:02}: {:02X}      V{:02}: {:02X}",
                i,
                self.v[i],
                i + 4,
                self.v[i + 4],
                i + 8,
                self.v[i + 8],
                i + 12,
                self.v[i + 12]
            )
        }
        println!("SP {:X}", self.sp);
        for i in 0..4 {
            println!(
                "V{:02}: {:04X}    V{:02}: {:04X}    V{:02}: {:04X}    V{:02}: {:04X}",
                i,
                self.stack[i],
                i + 4,
                self.stack[i + 4],
                i + 8,
                self.stack[i + 8],
                i + 12,
                self.stack[i + 12]
            )
        }
    }
}
