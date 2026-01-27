pub struct Memory{
    pub memory:[u8;0x1000],
}

impl Memory{
    pub fn new(rom:Vec<u8>) -> Self{
        let mut temp = Self { memory:[0;0x1000]};
        
        // fill memory with rom
       for i in 0..rom.len(){
            temp.memory[0x100 + i] = rom[i];
        }
        return temp;
    }
}
