use std::fs;

fn get_file_contents(file_path:&str) -> Vec<u8>{
    fs::read(file_path).expect("Some Error Occured")
}

fn main() {
    let file_data = get_file_contents("roms\\UFO.ch8");
}
