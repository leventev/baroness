use std::time::{SystemTime, UNIX_EPOCH};

use super::Emulator;

const PPUCTRL: u8 = 0;
const PPUMASK: u8 = 1;
const PPUSTATUS: u8 = 2;
const OAMADDR: u8 = 3;
const OAMDATA: u8 = 4;
const PPUSCROLL: u8 = 5;
const PPUADDR: u8 = 6;
const PPUDATA: u8 = 7;

const PPU_CLOCK_FREQUENCY: usize = 5_369_318;
const NANOSECONDS_PER_PPU_CLOCK: usize = 1_000_000_000 / PPU_CLOCK_FREQUENCY;

pub struct PPUData {
    last_clock_time: u128,
}

impl PPUData {
    pub fn new() -> PPUData {
        PPUData {
            last_clock_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        }
    }
}

impl Emulator {
    pub fn ppu_read(&self, off: u8) -> u8 {
        return 0;
        assert!(off < 8);
        match off {
            PPUSTATUS => {
                unreachable!()
            }
            _ => unreachable!(),
        }
    }

    pub fn ppu_write(&mut self, off: u8, val: u8) {
        return;
        assert!(off < 8);
        //self.regs[off as usize] = val;
        match off {
            _ => unreachable!(),
        }
    }

    pub fn clock_ppu(&mut self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let elapsed = current_time - self.ppu.last_clock_time;
        self.ppu.last_clock_time = current_time;

        let mut cycles_left = elapsed as usize / NANOSECONDS_PER_PPU_CLOCK;

        println!("cycles_left: {}", cycles_left);
    }
}
