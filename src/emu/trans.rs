use super::{instructions::Operand, Emulator};

pub fn tax(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => {
            emu.set_x(emu.regs.x);
        }
        _ => unreachable!(),
    }

    0
}

pub fn tay(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => {
            emu.set_y(emu.regs.x);
        }
        _ => unreachable!(),
    }

    0
}

pub fn tsx(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => {
            emu.set_x(emu.regs.sp);
        }
        _ => unreachable!(),
    }

    0
}

pub fn txa(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => {
            emu.set_a(emu.regs.a);
        }
        _ => unreachable!(),
    }

    0
}

pub fn txs(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => {
            emu.regs.sp = emu.regs.x;
        }
        _ => unreachable!(),
    }

    0
}

pub fn tya(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => {
            emu.set_a(emu.regs.y);
        }
        _ => unreachable!(),
    }

    0
}
