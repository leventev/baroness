use self::instructions::{INSTRUCTIONS, Instruction, AddressingMode};

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

    fn format_instruction(&self, inst: &Instruction) -> String {
        match inst.addressing_mode {
            AddressingMode::Accumulator | AddressingMode::Implied => {
                inst.name.to_string()
            },
            AddressingMode::Immediate => {
                let operand = self.mem[self.regs.pc as usize + 1];
                format!("{} #${:X}", inst.name, operand)
            },
            AddressingMode::Absolute => {
                let addr = {
                    let high = self.mem[self.regs.pc as usize + 1];
                    let low = self.mem[self.regs.pc as usize + 2];
                    u16::from_le_bytes([high, low])
                };
                format!("{} ${:X}", inst.name, addr)
            },
            AddressingMode::ZeroPage | AddressingMode::Relative => {
                let operand = self.mem[self.regs.pc as usize + 1];
                format!("{} ${:X}", inst.name, operand)
            },
            AddressingMode::AbsoluteIndirect => {
                let addr = {
                    let high = self.mem[self.regs.pc as usize + 1];
                    let low = self.mem[self.regs.pc as usize + 2];
                    u16::from_le_bytes([high, low])
                };
                format!("{} (${:X})", inst.name, addr)
            },
            AddressingMode::AbsoluteIndexedX => {
                let addr = {
                    let high = self.mem[self.regs.pc as usize + 1];
                    let low = self.mem[self.regs.pc as usize + 2];
                    u16::from_le_bytes([high, low])
                };
                format!("{} ${:X}, X", inst.name, addr)
            },
            AddressingMode::AbsoluteIndexedY => {
                let addr = {
                    let high = self.mem[self.regs.pc as usize + 1];
                    let low = self.mem[self.regs.pc as usize + 2];
                    u16::from_le_bytes([high, low])
                };
                format!("{} ${:X}, Y", inst.name, addr)
            },
            AddressingMode::ZeroPageIndexedX => {
                let operand = self.mem[self.regs.pc as usize + 1];
                format!("{} ${:X}, X", inst.name, operand)
            },
            AddressingMode::ZeroPageIndexedY => {
                let operand = self.mem[self.regs.pc as usize + 1];
                format!("{} ${:X}, Y", inst.name, operand)
            },
            AddressingMode::ZeroPageIndexedXIndirect => {
                let operand = self.mem[self.regs.pc as usize + 1];
                format!("{} (${:X}, X)", inst.name, operand)
            },
            AddressingMode::ZeroPageIndirectIndexedY => {
                let operand = self.mem[self.regs.pc as usize + 1];
                format!("{} (${:X}), Y", inst.name, operand)
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
                    let ins_str = self.format_instruction(ins);
                    println!("{:X}:\t{}", self.regs.pc, ins_str);
                    self.regs.pc += ins.bytes as u16;
                },
                None => panic!("invalid opcode {}", opcode)
            }
        }
    }

    pub fn start_emulation(&mut self, file: Vec<u8>, load_offset: usize) {
        assert!(load_offset < u16::MAX as usize);
        self.load_buff_to_mem(&file[..], load_offset);
        self.regs.pc = load_offset as u16;

        self.emulate();
    }
}
