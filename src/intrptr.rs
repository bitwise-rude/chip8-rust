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
    pub frame_buffer:[[u8;32];64],
}

impl CPU {
    pub fn new(mem:memory::Memory) -> Self {
        Self { 
            pc: 0x100,
            sp: 0xF,
            reg: [0;16],
            i: 0x00,
            st: 0x00,
            dt: 0x00,
            stack: [0;16],
            memory:mem,
            frame_buffer: [[0;32];64], 
        }
    }

    pub fn step(&mut self){
        // steps the cpu
        
        // fetch 
        let hi_opcode:u8 =  self.memory.memory[self.pc as usize];
        let lo_opcode:u8 = self.memory.memory[(self.pc +1 )  as usize];
        
        let op_code = u16::from_be_bytes([hi_opcode,lo_opcode]);
        self.pc += 2;

        let nnn:u16 = op_code & 0x0FFF;
        let n: u8 = (lo_opcode & 0x0F ) as u8;
        let x: u8 = (hi_opcode & 0x0F) as u8;
        let y: u8 = (lo_opcode & 0xF0) as u8;
        let kk = lo_opcode;
        let f: u8 = ((hi_opcode & 0xF0) >> 4 )as u8;
        
        // logs
        println!("Currently Executing Instruction {:04x} at PC: {:03x}",
            op_code,
            self.pc);
        
        // opcode decode and execution
        match f{
                0xA => self.i = nnn,

                0x0 => {
                    match lo_opcode {
                        0xE0 => (),
                        _ => println!("Not implemented EO INS"),
                    }
                },

                0x6 => self.reg[x as usize] = kk,

                0xD => (), 
                
                0x7 => self.reg[x as usize] += kk,

                0x1 => self.pc = nnn,

                _ => {
                panic!("Not Implemented");
            }
        } 

    }


}

