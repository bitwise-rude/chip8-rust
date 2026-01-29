use std::env;
use std::process::exit; 
use std::fs;
use macroquad::prelude::*;


mod intrptr;
mod memory;

const SCALE: i32  = 15;

fn window_conf() -> Conf {
    Conf {
        window_title: "CHIP 8".to_owned(),
        window_width: 64 * SCALE,
        window_height: 32 * SCALE,
        window_resizable: false, 
        fullscreen: false,      
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
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

    // make cpu
    let mut cpu = intrptr::CPU::new(mem);
   
    for _ in 0..100{
        cpu.step();

        // drawing the framebuffer
        for x in 0..64 {
            for y in 0..32{
                let mut color = BLACK;
                if cpu.frame_buffer[y][x] == 1 {
                    color = WHITE;
                }
                draw_rectangle(
                    (x as i32 * SCALE) as f32,
                    (y as i32 * SCALE) as f32,
                    SCALE as f32,
                    SCALE as f32 ,
                    color);
            }
        }

        next_frame().await;
    }

}
