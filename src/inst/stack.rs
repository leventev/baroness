use crate::emu::StatusRegister;

use super::{Emulator, Operand};

pub fn pha(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => emu.push_on_stack(emu.regs.a),
        _ => unreachable!(),
    }

    0
}

pub fn php(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => emu.push_on_stack(emu.regs.flags.clone().into_bytes()[0]),
        _ => unreachable!(),
    }

    0
}

pub fn pla(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => {
            let val = emu.pop_stack();
            emu.set_a(val);
        }
        _ => unreachable!(),
    }

    0
}

pub fn plp(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => {
            let val = emu.pop_stack();
            emu.regs.flags = StatusRegister::from_bytes([val]);
        }
        _ => unreachable!(),
    }

    0
}
