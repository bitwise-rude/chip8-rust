use rand::Rng;

pub struct Interpreter{
    registers:[u8;16],
    pc:u16, // Program counter
    I:u16, // instruction register

    memory:[u8;4096],
}

impl Interpreter{
    pub fn new(data:Vec<u8>) -> Self {
        // defaulting all the values to 0, execpt the program counter which begins at 0x200

        // filling the memory with the rom contents
        // starting at 0x200 since the previous
        // is filled with the interpreter and not 
        // meant for program
        let mut memory = [0;4096];

        // for (i, val) in data.iter().enumerate() {
        //     memory[0x200 + i] = *val;
        // }

        memory[0x200..(0x200+data.len())].copy_from_slice(&data);

        Interpreter { registers:[0;16],pc:0x200,I:0,memory:memory }

    }


    pub fn step(&mut self){
        // steps the interpreter by executing an instruction
        let current_byte = self.memory[self.pc as usize];
        self.pc  += 1;
        let next_byte = self.memory[(self.pc+1) as usize];
        println!("CURRENT OPCODE IS {:x}{:x}",current_byte,next_byte);



        // matching for xnnn instructions
        let nnn = (((current_byte & 0x0F) as u16) << 8) | next_byte as u16;
        // for Cx types of intruction
        let x = current_byte & 0x0F;
        let mut rng = rand::thread_rng();

        match current_byte & 0xF0{
            0xA0 => self.I = nnn, 
            0xC0 => self.registers[x as usize] = rng.gen_range(1..=255),

            _ => panic!("Not Implemented")
        }
     
        // debugging
        println!("Registers-{:?}\nI-{}",self.registers,self.I);
    }

}