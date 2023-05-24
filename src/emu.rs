use crate::{
    mapper::{get_mapper, Mapper},
    nes::NESFile,
};

use super::inst::{AddressingMode, Instruction, Operand, INSTRUCTIONS};

/// 2KiB internal memory
const INTERNAL_RAM_SIZE: usize = usize::pow(2, 11);

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct StatusRegister: u8 {
        const CARRY = 1 << 0;
        const ZERO = 1 << 1;
        const INTERRUPT_DISABLE = 1 << 2;
        const DECIMAL = 1 << 3;
        const BREAK = 1 << 4;
        const ALWAYS_SET = 1 << 5;
        const OVERFLOW = 1 << 6;
        const NEGATIVE = 1 << 7;
    }
}

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub flags: StatusRegister,
}

pub struct Emulator {
    internal_ram: Box<[u8]>,
    pub regs: Registers,
    mapper: Box<dyn Mapper>,
}

impl Emulator {
    fn get_operand(&self, addresing_mode: AddressingMode) -> Operand {
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
                    self.regs.pc + operand as u16 + inst.bytes as u16
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

    fn emulate(&mut self) {
        let mut cycles = 7;

        loop {
            let opcode = self.read(self.regs.pc);
            let instruction = &INSTRUCTIONS[opcode as usize];
            match instruction {
                Some(ins) => {
                    let operand = self.get_operand(ins.addressing_mode);

                    let ins_str = self.format_instruction(ins, operand);
                    println!(
                        "{:<04X}:\t{:<12}A: ${:<02X} X: ${:<02X} Y: ${:<02X} SP: ${:<02X} CYCLES: {:<6} P: {:?}",
                        self.regs.pc,
                        ins_str,
                        self.regs.a,
                        self.regs.x,
                        self.regs.y,
                        self.regs.sp,
                        cycles,
                        self.regs.flags,
                    );

                    self.regs.pc += ins.bytes as u16;
                    let extra_cycles = (ins.callback)(self, operand);

                    cycles += ins.cycles + extra_cycles;
                }
                None => panic!("invalid opcode {}", opcode),
            }
        }
    }

    pub fn start_emulation(&mut self) {
        self.emulate();
    }

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
        let addr = 0x100 + self.regs.sp as usize;

        self.internal_ram[addr] = val;
        self.regs.sp -= 1;
    }

    pub fn pop_stack(&mut self) -> u8 {
        self.regs.sp += 1;
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
        self.regs.flags.set(StatusRegister::ZERO, val == 0);
        self.regs
            .flags
            .set(StatusRegister::NEGATIVE, val & 1 << 7 != 0);
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

    pub fn read(&self, addr: u16) -> u8 {
        if addr < 0x2000 {
            // internal ram
            let off = addr & 0x800;
            self.internal_ram[off as usize]
        } else if addr < 0x4000 {
            // ppu regs
            todo!()
        } else if addr < 0x4020 {
            // apu, io registers
            todo!()
        } else {
            // cartridge space
            match self.mapper.read(addr) {
                Ok(val) => val,
                Err(_) => panic!("Open bus"),
            }
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        if addr < 0x2000 {
            // internal ram
            let off = addr & 0x800;
            self.internal_ram[off as usize] = val;
        } else if addr < 0x4000 {
            // ppu regs
            todo!()
        } else if addr < 0x4020 {
            // apu, io registers
            todo!()
        } else {
            // cartridge space
            self.mapper.write(addr, val).unwrap();
        }
    }

    pub fn new(file: Vec<u8>, nes_file: NESFile) -> Emulator {
        let mapper = get_mapper(&file, &nes_file);
        Emulator {
            internal_ram: vec![0; INTERNAL_RAM_SIZE].into_boxed_slice(),
            regs: Registers {
                a: 0,
                x: 0,
                y: 0,
                sp: 0xFD,
                pc: mapper.entrypoint(),
                flags: StatusRegister::ALWAYS_SET,
            },
            mapper,
        }
    }
}
