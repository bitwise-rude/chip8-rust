use rand::Rng;

pub struct Interpreter {
    registers: [u8; 16],   // V0..VF
    pc: u16,               // Program counter
    I: u16,                // Index register
    sp: u8,                // Stack pointer
    stack: [u16; 16],      // Call stack
    memory: [u8; 4096],    // RAM
}

impl Interpreter {
    pub fn new(data: Vec<u8>) -> Self {
        // Initialize memory with 0
        let mut memory = [0; 4096];

        // Load ROM contents into memory starting at 0x200
        memory[0x200..(0x200 + data.len())].copy_from_slice(&data);

        Interpreter {
            registers: [0; 16],
            pc: 0x200, // Programs start at 0x200
            I: 0,
            sp: 0,
            stack: [0; 16],
            memory,
        }
    }

    pub fn step(&mut self) {
        // Fetch
        let opcode = ((self.memory[self.pc as usize] as u16) << 8)
                   | (self.memory[(self.pc + 1) as usize] as u16);
        self.pc += 2;

        // Decode
        let nnn = opcode & 0x0FFF;
        let n   = (opcode & 0x000F) as u8;
        let x   = ((opcode & 0x0F00) >> 8) as usize;
        let y   = ((opcode & 0x00F0) >> 4) as usize;
        let kk  = (opcode & 0x00FF) as u8;

        let mut rng = rand::thread_rng();

        // Execute
        match opcode & 0xF000 {
            0x1000 => { // JP addr
                self.pc = nnn;
            }
            0x2000 => { // CALL addr
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            0x3000 => { // SE Vx, byte
                if self.registers[x] == kk {
                    self.pc += 2;
                }
            }
            0xA000 => { // LD I, addr
                self.I = nnn;
            }
            0xC000 => { // RND Vx, byte
                self.registers[x] = rng.random::<u8>() & kk;
            }
            0x0000 => match opcode {
                0x00E0 => { // CLS
                    panic!("CLS not implemented");
                }
                0x00EE => { // RET
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }
                _ => panic!("Not implemented: {:#X}", opcode),
            },
            _ => panic!("Unknown opcode: {:#X}", opcode),
        }

        // Debugging output
        println!(
            "Registers: {:?}, I={}, PC={:X}, SP={}",
            self.registers, self.I, self.pc, self.sp
        );
    }
}
