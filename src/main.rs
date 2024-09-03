use std::default;

#[derive(Debug)]
struct Chip8 {
    // registers
    PC: u16,
    SP: u8,

    Vx: Vec<u8>,
    I: u8,

    DT: u8,
    ST: u8,

    STACK: Vec<u16>,
    MEMORY: Vec<u8>,
    PIXEL_BUFFER: Vec<u8>,
}

impl Chip8 {
    fn new(program: Vec<u8>) -> Chip8 {
        let mut chip = Chip8 {
            PC: 0x200, // point to this location
            SP: 0,
            I: 0,
            DT: 0,
            ST: 0,

            Vx: vec![0; 16],
            STACK: vec![0; 16],
            MEMORY: vec![0; 4096],
            PIXEL_BUFFER: vec![0; 64 * 32],
        };

        // Load fonts in memory

        let fonts: Vec<u8> = vec![
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];

        for idx in 0..fonts.len() {
            chip.MEMORY[idx] = fonts[idx];
        }

        // Read Program in memory

        for i in 0..program.len() {
            chip.MEMORY[512 + i] = program[i];
        }

        chip
    }

    fn execute(&mut self) {
        loop {
            let loc = self.PC as usize;
            let opcode: u16 = (self.MEMORY[loc] as u16) << 8 | self.MEMORY[loc + 1] as u16;
            let first_nibble = (0xF000 & opcode) >> 12;

            match first_nibble {
                0x0000 => {
                    match opcode {
                        0x00E0 => {
                            // clear screen
                        }

                        0x00EE => {
                            // return from subrouting
                        }

                        _ => {
                            // call machine routing
                        }
                    }
                }

                0x1 => {
                    // 1NNN
                    let adddr = opcode & 0x0FFF;
                    println!("Address: {}", adddr);
                }

                0x2 => {
                    // 2NNN
                    let adddr = opcode & 0x0FFF;
                }

                0x3 => {
                    // 3XNN = if (Vx == NN)
                    let register = (opcode & 0x0F00) >> 8;
                    let con = opcode & 0x00FF;
                }

                0x4 => {
                    // 4XNN = if (vx != NN)
                    let register = (opcode & 0x0F00) >> 8;
                    let nn = opcode & 0x00FF;
                }

                0x5 => {
                    // 5XY0 = if(vx == vy)
                    let register_x = (opcode & 0x0F00) >> 8;
                    let register_y = (opcode & 0x0F00) >> 4;
                }

                0x6 => {
                    // 6XNN = Vx == NN
                    let register = (opcode & 0x0F00) >> 8;
                    let nn = opcode & 0x00FF;
                }

                0x7 => {
                    // 7XNN = Vx += NN
                    let register = (opcode & 0x0F00) >> 8;
                    let nn = opcode & 0x00FF;
                }

                0x8 => {
                    let last_bit = opcode & 0x000F;
                    let register_x = (opcode & 0x0F00) >> 8;
                    let register_y = (opcode & 0x0F00) >> 4;

                    match last_bit {
                        0x0 => {
                            // 8XY0 = Vx = Vy
                        }

                        0x1 => {
                            // 8XY1 => VX = VX | VY ( bitwise OR )
                        }

                        0x2 => {
                            // 8XY1 => VX = VX & VY ( bitwise AND )
                        }

                        0x3 => {
                            // 8XY1 => VX = VX ^ VY ( bitwise XOR )
                        }

                        0x4 => {
                            // 8XY1 => VX = VX + VY ( Addition )
                        }

                        0x5 => {
                            // 8XY1 => VX = VX - VY ( Subtract )
                        }

                        0x6 => {
                            // 8XY1 => VX = VX - VY ( >> 1 shift by 1 )
                        }

                        0x7 => {
                            // 8XY1 => VX = VY - VX
                        }

                        0x7 => {
                            // 8XY1 => VX = VY - VX
                        }

                        0xE => {
                            // 8XYE => VX << 1
                        }
                        _ => {}
                    }
                }

                0x9 => {
                    // 9XY0 == if (Vx != Vy)

                    let register_x = opcode & 0x0F00 >> 8;
                    let register_y = opcode & 0x00F0 >> 4;
                }

                0xA => {
                    // ANNN => I = NNN
                    let nnn = opcode & 0x0FFF;
                }

                0xB => {
                    // ANNN => PC = V0 + NNN
                    let nnn = opcode & 0x0FFF;
                }

                0xC => {
                    // CXNN => Vx = rand() & NNN
                    let register_x = (opcode & 0x0F00) >> 8;
                    let NN = opcode & 0x00F;
                }

                0xD => {
                    // DXYN => draw(Vx, VY, N)
                    let register_x = (opcode & 0x0F00) >> 8;
                    let register_y = (opcode & 0x00F0) >> 4;
                    let n = opcode & 0x000F;
                }

                0xE => {
                    let register_x = (opcode & 0x0F00) >> 8;
                    let last_2_nibbles = opcode & 0x00FF;

                    match last_2_nibbles {
                        0x9E => {
                            // EX9E
                        }

                        0xA1 => {
                            // EXA1
                        }

                        _ => {}
                    }
                }

                0xF => {
                    let register_x = (opcode & 0x0F00) >> 8;
                    let last_2_nibbles = opcode & 0x00FF;

                    match last_2_nibbles {
                        0x07 => {
                            // FX07
                        }

                        0x0A => {
                            // FX0A
                        }

                        0x15 => {
                            // FX15
                        }

                        0x18 => {
                            // FX18
                        }

                        0x1E => {
                            // FX1E
                        }

                        0x29 => {
                            // FX29
                        }

                        0x33 => {
                            // FX33
                        }

                        0x55 => {
                            // FX55
                        }

                        0x65 => {
                            // FX65
                        }

                        _ => {}
                    }
                }

                _ => {}
            }
            break;
        }
    }
}

fn main() {
    let program: Vec<u8> = vec![
        0x00, 0xE0, 0xA2, 0xF0, 0x60, 0x00, 0x61, 0x00, 0xD0, 0x15, 0x12, 0x00,
    ];
    let mut chip8 = Chip8::new(program);
    chip8.execute();
}
