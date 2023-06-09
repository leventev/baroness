#![feature(const_mut_refs)]

use std::fs;

mod emu;
mod inst;
mod mapper;
mod nes;

fn main() {
    let filepath = std::env::args().nth(1).expect("NES file path not provided");

    let file_buff = fs::read(filepath).expect("Could not read NES file");

    let file = nes::parse_nes_file(&file_buff).unwrap();
    let mut emu = emu::Emulator::new(file_buff, file);

    emu.start_emulation();
}
