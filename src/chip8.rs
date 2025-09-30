pub struct Interpreter{
    pub data: Vec<u8>,
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
        Interpreter { data: data, registers:[0;16],pc:0x200,I:0,dt:0,st:0,memory:[0;4096] }
    }
}