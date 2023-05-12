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

mod instructions;

bitflags::bitflags! {
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
            Operand::Accumulator | Operand::Implied => inst.name.to_string(),
            Operand::Immediate(operand)
            | Operand::ZeroPage(operand)
            | Operand::Relative(operand) => format!("{} ${:02X}", inst.name, operand),
            Operand::Absolute(addr) => format!("{} ${:04X}", inst.name, addr),
            Operand::AbsoluteIndirect(addr) => format!("{} (${:04X})", inst.name, addr),
            Operand::AbsoluteIndexedX(addr) => format!("{} ${:02X}, X", inst.name, addr),
            Operand::AbsoluteIndexedY(addr) => format!("{} ${:02X}, Y", inst.name, addr),
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

    fn get_byte_at_pc(&self) -> u8 {
        self.mem[self.regs.pc as usize]
    }

    fn forward_pc(&mut self) -> u8 {
        let b = self.get_byte_at_pc();
        println!(
            "forward pc {:#x} -> {:#x} byte: {}",
            self.regs.pc,
            self.regs.pc + 1,
            b
        );
        self.regs.pc += 1;
        b
    }

    fn push_on_stack(&mut self, val: u8) {
        self.regs.sp -= 1;
        self.mem[self.regs.sp as usize] = val;
    }

    fn get_indirect_address(&mut self, addr: u16) -> u16{
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
        /*loop {
            let opcode = self.forward_pc();
            println!("opcode: {:#x}", opcode);

            match opcode {
                // ORA $nn,x
                0x01 => {
                    let operand = self.forward_pc();
                    let zero_page_idx = (operand + self.regs.x) as usize;

                    let addr = {
                        let low = self.mem[zero_page_idx];
                        let high = self.mem[zero_page_idx + 1];
                        u16::from_le_bytes([low, high]) as usize
                    };

                    let val = self.mem[addr];
                    self.regs.a |= val;

                    self.set_zero_and_negative_flags(self.regs.a);
                }
                // PHP
                0x08 => {
                    self.push_on_stack(self.regs.flags.bits());
                }
                // LDA #$nn
                0xA9 => {
                    self.regs.a = self.forward_pc();
                    self.set_zero_and_negative_flags(self.regs.a);
                }
                // LDX #$nn
                0xA2 => {
                    self.regs.x = self.forward_pc();
                    self.set_zero_and_negative_flags(self.regs.x);
                }
                // STA $nn
                0x85 => {
                    let operand = self.forward_pc();
                    self.mem[operand as usize] = self.regs.a;
                }
                0x86 => {
                    let operand = self.forward_pc();
                    self.mem[operand as usize] = self.regs.x;
                }
                0x20 => {
                    let jmp_addr = {
                        let high = self.forward_pc();
                        let low = self.forward_pc();
                        u16::from_le_bytes([high, low])
                    };

                    let (ret_high, ret_low) = {
                        let ret = self.regs.pc;
                        ((ret >> 8) as u8, (ret & 0xFF) as u8)
                    };

                    self.push_on_stack(ret_low);
                    self.push_on_stack(ret_high);

                    println!("{}", jmp_addr);
                    self.regs.pc = jmp_addr;
                }
                _ => panic!("unsupported opcode {:#x}", opcode),
            }
        }*/

        loop {
            let opcode = self.mem[self.regs.pc as usize];
            let instruction = &INSTRUCTIONS[opcode as usize];
            match instruction {
                Some(ins) => {
                    let operand = self.get_operand(ins.addressing_mode);

                    let ins_str = self.format_instruction(ins, operand);
                    println!("{:X}:\t{}", self.regs.pc, ins_str);

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
