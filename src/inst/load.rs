use crate::emu::StatusRegister;

use super::{Emulator, Operand};

pub fn lda(emu: &mut Emulator, op: Operand) -> usize {
    let (val, crossed) = emu.get_val_from_operand_cross(op);
    emu.set_a(val);
    crossed.into()
}

pub fn ldx(emu: &mut Emulator, op: Operand) -> usize {
    let (val, crossed) = emu.get_val_from_operand_cross(op);
    emu.set_x(val);
    crossed.into()
}

pub fn ldy(emu: &mut Emulator, op: Operand) -> usize {
    let (val, crossed) = emu.get_val_from_operand_cross(op);
    emu.set_y(val);
    crossed.into()
}

pub fn sta(emu: &mut Emulator, op: Operand) -> usize {
    let addr = emu.get_addr_from_operand(op);
    emu.write(addr, emu.regs.a);
    0
}

pub fn stx(emu: &mut Emulator, op: Operand) -> usize {
    let addr = emu.get_addr_from_operand(op);
    emu.write(addr, emu.regs.x);
    0
}

pub fn sty(emu: &mut Emulator, op: Operand) -> usize {
    let addr = emu.get_addr_from_operand(op);
    emu.write(addr, emu.regs.y);
    0
}

// Unofficial

pub fn lax(emu: &mut Emulator, op: Operand) -> usize {
    let (val, crossed) = emu.get_val_from_operand_cross(op);

    emu.regs.a = val;
    emu.regs.x = val;

    emu.regs.flags.set(StatusRegister::ZERO, val == 0);
    emu.regs
        .flags
        .set(StatusRegister::NEGATIVE, val & (1 << 7) > 0);

    crossed.into()
}

pub fn sax(emu: &mut Emulator, op: Operand) -> usize {
    let res = emu.regs.a & emu.regs.x;

    let addr = emu.get_addr_from_operand(op);
    emu.write(addr, res);
    0
}
