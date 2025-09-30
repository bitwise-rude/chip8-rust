pub struct Interpreter{
    registers:[u8;16],
    pc:u16, // Program counter
    I:u8, // instruction register
    dt:u8,
    st:u8,

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

        Interpreter { registers:[0;16],pc:0x200,I:0,dt:0,st:0,memory:memory }

    }

    pub fn step(&mut self){
        // steps the interpreter by executing an instruction
        let current_byte = self.memory[self.pc as usize];
        self.pc  += 1;
        let next_byte = self.memory[(self.pc+1) as usize];

        println!("CURRENT OPCODE IS {:x}{:x}",current_byte,next_byte);

        

    }

}