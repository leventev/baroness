use crate::inst::{AddressingMode, Instruction, Operand, INSTRUCTIONS};

use super::{Emulator, StatusRegister};

pub struct CPUData {
    cycle_advance: usize,
    cycle_debt: usize,
}

impl CPUData {
    pub fn new() -> CPUData {
        CPUData {
            cycle_advance: 0,
            cycle_debt: 0,
        }
    }
}

impl Emulator {
    pub fn set_a(&mut self, val: u8) {
        self.regs.a = val;
        self.set_zero_and_negative_flags(self.regs.a);
    }

    pub fn set_x(&mut self, val: u8) {
        self.regs.x = val;
        self.set_zero_and_negative_flags(self.regs.x);
    }

    pub fn set_y(&mut self, val: u8) {
        self.regs.y = val;
        self.set_zero_and_negative_flags(self.regs.y);
    }

    pub fn push_on_stack(&mut self, val: u8) {
        let addr = 0x100 + self.regs.sp as u16;

        self.write(addr, val);
        self.regs.sp = self.regs.sp.wrapping_sub(1);
    }

    pub fn pop_stack(&mut self) -> u8 {
        self.regs.sp = self.regs.sp.wrapping_add(1);
        let addr = 0x100 + self.regs.sp as u16;

        self.read(addr)
    }

    fn get_zero_page_indirect_address(&mut self, off: u8) -> u16 {
        let low = self.read(off as u16);
        let high = self.read(off.wrapping_add(1) as u16);
        u16::from_le_bytes([low, high])
    }

    pub fn get_indirect_address_wrapping(&mut self, addr: u16) -> u16 {
        let low = self.read(addr);
        let high_addr = (addr & 0xFF00) + ((addr + 1) & 0x00FF);
        let high = self.read(high_addr);
        u16::from_le_bytes([low, high])
    }

    pub fn set_zero_and_negative_flags(&mut self, val: u8) {
        self.regs.flags.set_zero((val == 0).into());
        self.regs.flags.set_negative((val & (1 << 7) > 0).into());
    }

    pub fn get_val_from_operand(&mut self, op: Operand) -> u8 {
        if let Operand::Immediate(val) = op {
            val
        } else {
            let addr = self.get_addr_from_operand(op);
            self.read(addr)
        }
    }

    pub fn get_addr_from_operand(&mut self, op: Operand) -> u16 {
        match op {
            Operand::Absolute(addr) => addr,
            Operand::AbsoluteIndexedX(addr) => addr.wrapping_add(self.regs.x as u16),
            Operand::AbsoluteIndexedY(addr) => addr.wrapping_add(self.regs.y as u16),
            Operand::ZeroPage(off) => off as u16,
            Operand::ZeroPageIndexedX(off) => off.wrapping_add(self.regs.x) as u16,
            Operand::ZeroPageIndexedY(off) => off.wrapping_add(self.regs.y) as u16,
            Operand::ZeroPageIndexedXIndirect(off) => {
                let zp_off = off.wrapping_add(self.regs.x);
                self.get_zero_page_indirect_address(zp_off)
            }
            Operand::ZeroPageIndirectIndexedY(off) => {
                let addr = self.get_zero_page_indirect_address(off);
                addr.wrapping_add(self.regs.y as u16)
            }
            _ => unreachable!(),
        }
    }

    pub fn get_val_from_operand_cross(&mut self, op: Operand) -> (u8, bool) {
        let mut page_crossed = false;

        let val = match op {
            Operand::AbsoluteIndexedX(addr) => {
                let final_addr = addr.wrapping_add(self.regs.x as u16);

                page_crossed = final_addr & 0xFF00 != addr & 0xFF00;
                self.read(final_addr)
            }
            Operand::AbsoluteIndexedY(addr) => {
                let final_addr = addr.wrapping_add(self.regs.y as u16);

                page_crossed = final_addr & 0xFF00 != addr & 0xFF00;
                self.read(final_addr)
            }
            Operand::ZeroPageIndirectIndexedY(off) => {
                let addr = self.get_zero_page_indirect_address(off);
                let final_addr = addr.wrapping_add(self.regs.y as u16);

                page_crossed = final_addr & 0xFF00 != addr & 0xFF00;
                self.read(final_addr)
            }
            _ => self.get_val_from_operand(op),
        };

        (val, page_crossed)
    }

    fn get_operand(&mut self, addresing_mode: AddressingMode) -> Operand {
        let single_operand = self.read(self.regs.pc.wrapping_add(1));
        let address_operand = {
            let high = self.read(self.regs.pc.wrapping_add(1));
            let low = self.read(self.regs.pc.wrapping_add(2));
            u16::from_le_bytes([high, low])
        };

        match addresing_mode {
            AddressingMode::Accumulator => Operand::Accumulator,
            AddressingMode::Implied => Operand::Implied,
            AddressingMode::Immediate => Operand::Immediate(single_operand),
            AddressingMode::Absolute => Operand::Absolute(address_operand),
            AddressingMode::ZeroPage => Operand::ZeroPage(single_operand),
            AddressingMode::Relative => Operand::Relative(single_operand),
            AddressingMode::AbsoluteIndirect => Operand::AbsoluteIndirect(address_operand),
            AddressingMode::AbsoluteIndexedX => Operand::AbsoluteIndexedX(address_operand),
            AddressingMode::AbsoluteIndexedY => Operand::AbsoluteIndexedY(address_operand),
            AddressingMode::ZeroPageIndexedX => Operand::ZeroPageIndexedX(single_operand),
            AddressingMode::ZeroPageIndexedY => Operand::ZeroPageIndexedY(single_operand),
            AddressingMode::ZeroPageIndexedXIndirect => {
                Operand::ZeroPageIndexedXIndirect(single_operand)
            }
            AddressingMode::ZeroPageIndirectIndexedY => {
                Operand::ZeroPageIndirectIndexedY(single_operand)
            }
        }
    }

    fn format_instruction(&self, inst: &Instruction, op: Operand) -> String {
        match op {
            Operand::Implied => inst.name.to_string(),
            Operand::Accumulator => format!("{} a", inst.name),
            Operand::Immediate(operand) => format!("{} #${:02X}", inst.name, operand),
            Operand::ZeroPage(operand) => {
                format!("{} ${:02X}", inst.name, operand)
            }
            Operand::Relative(operand) => {
                format!(
                    "{} ${:04X}",
                    inst.name,
                    self.regs
                        .pc
                        .wrapping_add_signed(inst.bytes as i16)
                        .wrapping_add_signed(operand as i8 as i16)
                )
            }
            Operand::Absolute(addr) => format!("{} ${:04X}", inst.name, addr),
            Operand::AbsoluteIndirect(addr) => format!("{} (${:04X})", inst.name, addr),
            Operand::AbsoluteIndexedX(addr) => format!("{} ${:04X},X", inst.name, addr),
            Operand::AbsoluteIndexedY(addr) => format!("{} ${:04X},Y", inst.name, addr),
            Operand::ZeroPageIndexedX(operand) => format!("{} ${:02X},X", inst.name, operand),
            Operand::ZeroPageIndexedY(operand) => format!("{} ${:02X},Y", inst.name, operand),
            Operand::ZeroPageIndexedXIndirect(operand) => {
                format!("{} (${:02X},X)", inst.name, operand)
            }
            Operand::ZeroPageIndirectIndexedY(operand) => {
                format!("{} (${:02X}),Y", inst.name, operand)
            }
        }
    }

    pub fn clock_cpu(&mut self) {
        let mut cycles_left = 1 + self.cpu.cycle_advance;
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
                        self.regs.flags
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

    pub fn nmi(&mut self) {
        let (ret_high, ret_low) = {
            let ret = self.regs.pc;
            ((ret >> 8) as u8, (ret & 0xFF) as u8)
        };

        self.push_on_stack(ret_high);
        self.push_on_stack(ret_low);

        self.regs.flags.set_break_command(0);
        self.regs.flags.set_interrupt_disable(1);

        self.push_on_stack(self.regs.flags.bytes[0]);

        let addr_low = self.read(0xFFFA) as u16;
        let addr_high = self.read(0xFFFB) as u16;

        let addr = addr_high << 8 | addr_low;

        self.regs.pc = addr;
    }

    pub fn reset(&mut self) {
        self.regs.flags = StatusRegister::new().with_always_set(1);

        self.regs.a = 0;
        self.regs.x = 0;
        self.regs.y = 0;

        self.regs.sp = 0xFD;

        let addr_low = self.read(0xFFFC) as u16;
        let addr_high = self.read(0xFFFD) as u16;
        let addr = addr_high << 8 | addr_low;

        self.regs.pc = addr;
    }
}
