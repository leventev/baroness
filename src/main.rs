#![feature(const_mut_refs)]

use std::fs;

use crate::nes::parse_nes_file;

mod emu;
mod nes;

fn main() {
    let filepath = std::env::args().nth(1).expect("NES file path not provided");

    let file_buff = fs::read(filepath).expect("Could not read NES file");

    let mut emu = emu::Emulator::new();
    parse_nes_file(&mut emu, file_buff).unwrap();

    emu.start_emulation();
}
