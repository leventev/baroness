use crate::emu::{Emulator, StatusRegister};

use super::Operand;

pub fn brk(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => {
            emu.push_on_stack(emu.regs.flags.clone().into_bytes()[0]);

            let (ret_high, ret_low) = {
                let ret = emu.regs.pc;
                ((ret >> 8) as u8, (ret & 0xFF) as u8)
            };

            emu.push_on_stack(ret_low);
            emu.push_on_stack(ret_high);

            let addr_high = emu.read(0xFFFE) as u16;
            let addr_low = emu.read(0xFFFF) as u16;

            let addr = addr_high << 8 | addr_low;

            emu.regs.pc = addr;
        }
        _ => unreachable!(),
    };

    0
}

pub fn jmp(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Absolute(addr) => {
            emu.regs.pc = addr;
        }
        Operand::AbsoluteIndirect(addr) => {
            let final_addr = emu.get_indirect_address_wrapping(addr);
            emu.regs.pc = final_addr;
        }
        _ => unreachable!(),
    }

    0
}

pub fn jsr(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Absolute(addr) => {
            let (ret_high, ret_low) = {
                let ret = emu.regs.pc - 1;
                ((ret >> 8) as u8, (ret & 0xFF) as u8)
            };

            emu.push_on_stack(ret_high);
            emu.push_on_stack(ret_low);

            emu.regs.pc = addr;
        }
        _ => unreachable!(),
    };

    0
}

pub fn rti(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => {
            let status = emu.pop_stack();

            let ret_low = emu.pop_stack() as u16;
            let ret_high = emu.pop_stack() as u16;

            let ret_addr = ret_high << 8 | ret_low;

            emu.regs.flags = StatusRegister::from_bytes([status]);
            emu.regs.pc = ret_addr;
        }
        _ => unreachable!(),
    }

    0
}

pub fn rts(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => {
            let ret_low = emu.pop_stack() as u16;
            let ret_high = emu.pop_stack() as u16;

            let ret_addr = ret_high << 8 | ret_low;

            emu.regs.pc = ret_addr + 1;
        }
        _ => unreachable!(),
    }

    0
}
