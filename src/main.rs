#![feature(const_mut_refs)]

use std::fs;

mod nes;
mod emu;

fn main() {
    let filepath = std::env::args().nth(1).expect("NES file path not provided");

    let file_buff = fs::read(filepath).expect("Could not read NES file");

    //parse_nes_file(file_buff).unwrap();

    let mut emu = emu::Emulator::new();
    emu.start_emulation(file_buff, 0x1000);

    println!("Hello, world!");
}
