use std::env;
use std::process::exit; 
use std::fs;

mod intrptr;
mod memory;

fn main() {
    let mut args = env::args();
    
    if args.len() < 2 { 
        println!("Please a ROM File");
        exit(1);
    } 

    // load the rom in the file
    let rom_contents: Vec<u8> = fs::read(args.nth(1)
        .expect("Some Argument error"))
        .expect("Couldn't read the ROM File");
    
    // memory
    let mem = memory::Memory::new(rom_contents);

    // frame buffer
    let frame_buffer:[[u8;32];64] = [[0;32];64];

    // make cpu
    let mut cpu = intrptr::CPU::new(mem);
   
    for i in 0..100{
        cpu.step();
    }




}
