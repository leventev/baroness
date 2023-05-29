use std::time::{SystemTime, UNIX_EPOCH};

use crate::inst::INSTRUCTIONS;

use super::Emulator;

/// https://www.nesdev.org/wiki/CPU
const CPU_CLOCK_FREQUENCY: usize = 1_789_773;
const NANOSECONDS_PER_CPU_CLOCK: usize = 1_000_000_000 / CPU_CLOCK_FREQUENCY;

pub struct CPUData {
    cycle_advance: usize,
    cycle_debt: usize,
    last_clock_time: u128,
}

impl CPUData {
    pub fn new() -> CPUData {
        CPUData {
            cycle_advance: 0,
            cycle_debt: 0,
            last_clock_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        }
    }
}

impl Emulator {
    pub fn clock_cpu(&mut self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let elapsed = current_time - self.cpu.last_clock_time;
        let mut cycles_left = elapsed as usize / NANOSECONDS_PER_CPU_CLOCK;

        println!("elapsed: {} cycles_left: {}", elapsed, cycles_left);

        self.cpu.last_clock_time = current_time;

        cycles_left += self.cpu.cycle_advance;
        self.cpu.cycle_advance = 0;

        if self.cpu.cycle_debt > cycles_left {
            self.cpu.cycle_debt -= cycles_left;
            return;
        } else {
            cycles_left -= self.cpu.cycle_debt;
            self.cpu.cycle_debt = 0;
        }

        while cycles_left > 0 {
            let opcode = self.read(self.regs.pc);
            let instruction = &INSTRUCTIONS[opcode as usize];

            match instruction {
                Some(ins) => {
                    if ins.cycles > cycles_left {
                        self.cpu.cycle_advance = cycles_left;
                        return;
                    }

                    let operand = self.get_operand(ins.addressing_mode);

                    // TODO: this slows down execution tremendously
                    /*let ins_str = self.format_instruction(ins, operand);
                    println!(
                        "{:<04X}:\t{:<12}A: ${:<02X} X: ${:<02X} Y: ${:<02X} SP: ${:<02X} P: {:?}",
                        self.regs.pc,
                        ins_str,
                        self.regs.a,
                        self.regs.x,
                        self.regs.y,
                        self.regs.sp,
                        self.regs.flags,
                    );*/

                    self.regs.pc += ins.bytes as u16;
                    let extra_cycles = (ins.callback)(self, operand);

                    let total_cycles = ins.cycles + extra_cycles;
                    if total_cycles > cycles_left {
                        self.cpu.cycle_debt = total_cycles - cycles_left;
                        cycles_left = 0;
                    } else {
                        cycles_left -= total_cycles;
                    }
                }
                None => panic!("invalid opcode {}", opcode),
            }
        }
    }
}
