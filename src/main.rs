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
        for _ in 0..6 {
            let loc = self.PC as usize;
            let opcode: u16 = (self.MEMORY[loc] as u16) << 8 | self.MEMORY[loc + 1] as u16;
            let first_nibble = (0xF000 & opcode) >> 12;


            println!("{:#04x}, {}", opcode, self.I);
            match first_nibble {
                0x0 => {
                    match opcode {
                        0x00E0 => {
                            // clear screen
                            self.PIXEL_BUFFER.fill(0);
                            self.PC += 2;
                        }

                        0x00EE => {
                            // return from subroutine i.e
                            // set PC to the address on the stack???
                            self.PC = self.STACK.pop().unwrap();
                        }

                        _ => {
                            // call machine routing
                        }
                    }
                }

                0x1 => {
                    // 1NNN -> goto; NNN
                    let addr = opcode & 0x0FFF;
                    self.PC = addr;
                }

                0x2 => {
                    // 2NNN -> Calls sub routine at NNN
                    let addr = opcode & 0x0FFF;
                    self.STACK.push(self.PC);
                    self.PC = addr;
                }

                0x3 => {
                    // 3XNN = if (Vx == NN)
                    let register = ((opcode & 0x0F00) >> 8) as usize;
                    let nn = (opcode & 0x00FF) as u8;
                    if self.Vx[register] == nn {
                        self.PC += 4
                    }
                }

                0x4 => {
                    // 4XNN = if (vx != NN)
                    let register = ((opcode & 0x0F00) >> 8) as usize;
                    let nn = (opcode & 0x00FF) as u8;
                    if self.Vx[register] != nn {
                        self.PC += 4
                    }
                }

                0x5 => {
                    // 5XY0 = if(vx == vy)
                    let register_x = ((opcode & 0x0F00) >> 8) as usize;
                    let register_y = ((opcode & 0x0F00) >> 4) as usize;
                    if self.Vx[register_x] == self.Vx[register_y] {
                        self.PC += 4
                    }
                }

                0x6 => {
                    // 6XNN = Vx = NN
                    let register = ((opcode & 0x0F00) >> 8) as usize;
                    let nn = (opcode & 0x00FF) as u8;
                    self.Vx[register] = nn;
                    self.PC += 2;
                }

                0x7 => {
                    // 7XNN = Vx += NN
                    let register = ((opcode & 0x0F00) >> 8) as usize;
                    let nn = (opcode & 0x00FF) as u8;
                    self.Vx[register] += nn;
                    self.PC += 2;
                }

                0x8 => {
                    let last_bit = opcode & 0x000F;
                    let register_x = ((opcode & 0x0F00) >> 8) as usize;
                    let register_y = ((opcode & 0x0F00) >> 4) as usize;

                    match last_bit {
                        0x0 => {
                            // 8XY0 = Vx = Vy
                            self.Vx[register_x] = self.Vx[register_y];
                            self.PC += 2;
                        }

                        0x1 => {
                            // 8XY1 => VX = VX | VY ( bitwise OR )
                            self.Vx[register_x] = self.Vx[register_x] | self.Vx[register_y];
                            self.PC += 2;
                        }

                        0x2 => {
                            // 8XY1 => VX = VX & VY ( bitwise AND )
                            self.Vx[register_x] = self.Vx[register_x] & self.Vx[register_y];
                            self.PC += 2;
                        }

                        0x3 => {
                            // 8XY1 => VX = VX ^ VY ( bitwise XOR )
                            self.Vx[register_x] = self.Vx[register_x] ^ self.Vx[register_y];
                            self.PC += 2;
                        }

                        0x4 => {
                            // 8XY1 => VX = VX + VY ( Addition )
                            let sum = self.Vx[register_x] + self.Vx[register_y];
                            if sum > 254 {
                                self.Vx[15] = 1
                            }
                            self.Vx[register_x] = sum;
                            self.PC += 2;
                        }

                        0x5 => {
                            // 8XY1 => VX = VX - VY ( Subtract )
                            if self.Vx[register_x] >= self.Vx[register_y] {
                                self.Vx[15] = 1
                            } else {
                                self.Vx[15] = 0
                            }

                            self.Vx[register_x] -= self.Vx[register_y];
                            self.PC += 2;
                        }

                        0x6 => {
                            // 8XY1 => VX = VX - VY ( >> 1 shift by 1 )
                            self.Vx[15] = self.Vx[register_x] & 0x01;
                            self.Vx[register_x] >>= 1;
                            self.PC += 2;
                        }

                        0x7 => {
                            // 8XY1 => VX = VY - VX
                            if self.Vx[register_y] >= self.Vx[register_x] {
                                self.Vx[15] = 1
                            } else {
                                self.Vx[15] = 0
                            }
                            self.Vx[register_x] = self.Vx[register_y] - self.Vx[register_x];
                            self.PC += 2;
                        }

                        0xE => {
                            // 8XYE => VX << 1
                            self.Vx[15] = self.Vx[register_x] & 0x10;
                            self.Vx[register_x] <<= 1;
                            self.PC += 2;
                        }
                        _ => {}
                    }
                }

                0x9 => {
                    // 9XY0 == if (Vx != Vy)

                    let register_x = (opcode & 0x0F00 >> 8) as usize;
                    let register_y = (opcode & 0x00F0 >> 4) as usize;

                    if self.Vx[register_x] != self.Vx[register_y] {
                        self.PC += 4;
                    }
                }

                0xA => {
                    // ANNN => I = NNN
                    let nnn = (opcode & 0x0FFF) as u8;
                    self.I = nnn;
                    self.PC += 2;
                }

                0xB => {
                    // ANNN => PC = V0 + NNN
                    let nnn = opcode & 0x0FFF;
                    self.PC = (self.Vx[0] as u16) + nnn;
                }

                0xC => {
                    // CXNN => Vx = rand() & NNN
                    let register_x = ((opcode & 0x0F00) >> 8) as usize;
                    let nn = (opcode & 0x00F) as u8;
                    let rand_num = rand::random::<u8>();
                    self.Vx[register_x] = rand_num & nn;
                }

                0xD => {
                    // DXYN => draw(Vx, VY, N)
                    let register_x = ((opcode & 0x0F00) >> 8) as usize;
                    let register_y = ((opcode & 0x00F0) >> 4) as usize;
                    let n = (opcode & 0x000F) as u8;

                    let coord_x = self.Vx[register_x];
                    let coord_y = self.Vx[register_y];

                    // let

                    
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
                    let register_x = ((opcode & 0x0F00) >> 8) as usize;
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
                            self.I += self.Vx[register_x];
                            self.PC += 2;
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
            // break;
            // println!("{:?}", self.PC);
        }
    }
}

fn main() {
    // let program: Vec<u8> = vec![
    //     0x00, 0xE0, 0xA2, 0xF0, 0x60, 0x00, 0x61, 0x00, 0xD0, 0x15, 0x12, 0x00,
    // ];

    // let program: Vec<u8> = vec![0x60,0x20,0x61, 0x12, 0x80, 0x14];

    let program: Vec<u8> = vec![0x00, 0xE0, 0x60, 0x00, 0x61, 0x00, 0x62, 0x41, 0xF2, 0x2A, 0xD0, 0x15];
    let mut chip8 = Chip8::new(program);
    chip8.execute();

    println!("{:?}, I: {:?}", chip8.Vx, chip8.I)
}
