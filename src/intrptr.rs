use crate::memory;

pub struct CPU {
    pc: u16,
    sp: u16,
    reg: [u8;16],
    i: u16,

    // timers
    st: u8,
    dt: u8,

    // memory
    memory:memory::Memory,
    stack: [u16;16],

    // frame_buffer
    pub frame_buffer:[[u8;64];32],
}

impl CPU {
    pub fn new(mem:memory::Memory) -> Self {
        Self { 
            pc: 0x200,
            sp: 0xF,
            reg: [0;16],
            i: 0x00,
            st: 0x00,
            dt: 0x00,
            stack: [0;16],
            memory:mem,
            frame_buffer: [[0;64];32], 
        }
    }

    pub fn step(&mut self){
        // steps the cpu
        
        // fetch 
        let hi_opcode:u8 =  self.memory.memory[self.pc as usize];
        let lo_opcode:u8 = self.memory.memory[(self.pc +1 )  as usize];
        
        let op_code = u16::from_be_bytes([hi_opcode,lo_opcode]);
        
        println!("Currently Executing Instruction {:04x} at PC: {:03x}",
            op_code,
            self.pc);
 
        self.pc += 2;

        let nnn:u16 = op_code & 0x0FFF;
        let n: u8 = (lo_opcode & 0x0F ) as u8;
        let x: u8 = (hi_opcode & 0x0F) as u8;
        let y: u8 = ((lo_opcode & 0xF0) as u8)>>4;
        let kk = lo_opcode;
        let f: u8 = ((hi_opcode & 0xF0) >> 4 )as u8;
       
        // opcode decode and execution
        match f{
                0xA => self.i = nnn,

                0x6 => self.reg[x as usize] = kk,

                0x0 => {
                    if kk == 0xE0 {
                        for i in 0..31{
                            for j in 0..63{
                                self.frame_buffer[i][j] = 0;
                            }
                        }
                    }
                    else{
                        panic!("NOT IMPLEMENTED");
                    }
                }

                0x7 => self.reg[x as usize] += kk,

                0x1 => self.pc = nnn,

                0xD => {
                   let c_x = self.reg[x as usize];
                   let c_y = self.reg[y as usize];

                    for cnt in 0..n{
                        let sprite:u8 = self.memory.memory[(self.i + (cnt as u16)) as usize]; 

                        for of_x in 0..7{
                            self.frame_buffer[(((cnt as u8) + c_y) % 32) as usize] 
                                [(((of_x as u8) + c_x) % 64) as usize] ^= ((sprite & (0b10000000 >> of_x)) >> (7-of_x)); 
                        }

                    }
                }

                _ => {
                panic!("Not Implemented");
            }
        } 

    }


}

