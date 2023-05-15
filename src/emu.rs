use self::instructions::{AddressingMode, Instruction, Operand, INSTRUCTIONS};

const MEMORY_SIZE: usize = usize::pow(2, 16);

mod arithmetic;
mod branch;
mod control;
mod flags;
mod load;
mod logic;
mod misc;
mod stack;
mod trans;

mod utils;

mod instructions;

bitflags::bitflags! {
    #[derive(Debug)]
    struct StatusRegister: u8 {
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

struct Registers {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
    flags: StatusRegister,
}

pub struct Emulator {
    mem: Box<[u8]>,
    regs: Registers,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            mem: vec![0; MEMORY_SIZE].into_boxed_slice(),
            regs: Registers {
                a: 0,
                x: 0,
                y: 0,
                sp: 0xFD,
                pc: 0,
                flags: StatusRegister::ALWAYS_SET,
            },
        }
    }

    fn get_operand(&self, addresing_mode: AddressingMode) -> Operand {
        let single_operand = self.mem[self.regs.pc as usize + 1];
        let address_operand = {
            let high = self.mem[self.regs.pc as usize + 1];
            let low = self.mem[self.regs.pc as usize + 2];
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
            Operand::AbsoluteIndexedX(addr) => format!("{} ${:04X}, X", inst.name, addr),
            Operand::AbsoluteIndexedY(addr) => format!("{} ${:04X}, Y", inst.name, addr),
            Operand::ZeroPageIndexedX(operand) => format!("{} ${:02X}, X", inst.name, operand),
            Operand::ZeroPageIndexedY(operand) => format!("{} ${:02X}, Y", inst.name, operand),
            Operand::ZeroPageIndexedXIndirect(operand) => {
                format!("{} (${:02X}, X)", inst.name, operand)
            }
            Operand::ZeroPageIndirectIndexedY(operand) => {
                format!("{} (${:02X}), Y", inst.name, operand)
            }
        }
    }

    fn load_buff_to_mem(&mut self, buff: &[u8], offset: usize) {
        println!("{} {}", buff.len(), self.mem.len());
        assert!(offset + buff.len() <= self.mem.len());

        let dest = &mut self.mem[offset..offset + buff.len()];
        dest.copy_from_slice(buff);
    }

    fn push_on_stack(&mut self, val: u8) {
        let addr = 0x100 + self.regs.sp as usize;

        self.mem[addr] = val;
        self.regs.sp -= 1;
    }

    fn pop_stack(&mut self) -> u8 {
        self.regs.sp += 1;
        let addr = 0x100 + self.regs.sp as usize;

        self.mem[addr]
    }

    fn get_indirect_address(&mut self, addr: u16) -> u16 {
        let low = self.mem[addr as usize];
        let high = self.mem[addr as usize + 1];
        u16::from_le_bytes([low, high])
    }

    fn set_zero_and_negative_flags(&mut self, val: u8) {
        self.regs.flags.set(StatusRegister::ZERO, val == 0);
        self.regs
            .flags
            .set(StatusRegister::NEGATIVE, val & 1 << 7 != 0);
    }

    fn emulate(&mut self) {
        loop {
            let opcode = self.mem[self.regs.pc as usize];
            let instruction = &INSTRUCTIONS[opcode as usize];
            match instruction {
                Some(ins) => {
                    let operand = self.get_operand(ins.addressing_mode);

                    let ins_str = self.format_instruction(ins, operand);
                    println!(
                        "{:X}:\t{:<12}A: ${:<02X} X: ${:<02X} Y: ${:<02X} SP: ${:<02X} P: {:?}",
                        self.regs.pc,
                        ins_str,
                        self.regs.a,
                        self.regs.x,
                        self.regs.y,
                        self.regs.sp,
                        self.regs.flags
                    );

                    self.regs.pc += ins.bytes as u16;
                    (ins.callback)(self, operand);
                }
                None => panic!("invalid opcode {}", opcode),
            }
        }
    }

    fn set_a(&mut self, val: u8) {
        self.regs.a = val;
        self.set_zero_and_negative_flags(self.regs.a);
    }

    fn set_x(&mut self, val: u8) {
        self.regs.x = val;
        self.set_zero_and_negative_flags(self.regs.x);
    }

    fn set_y(&mut self, val: u8) {
        self.regs.y = val;
        self.set_zero_and_negative_flags(self.regs.y);
    }

    pub fn load_program(&mut self, file: &[u8], load_offset: usize) {
        assert!(load_offset < u16::MAX as usize);
        self.load_buff_to_mem(file, load_offset);
        self.regs.pc = load_offset as u16;
    }

    pub fn start_emulation(&mut self) {
        self.emulate();
    }
}
