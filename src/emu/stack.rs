use super::{instructions::Operand, AddressingMode, Emulator, StatusRegister};

pub fn pha(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => emu.push_on_stack(emu.regs.a),
        _ => unreachable!(),
    }

    0
}

pub fn php(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => emu.push_on_stack(emu.regs.flags.bits()),
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
            emu.regs.flags = StatusRegister::from_bits(val).unwrap();
        }
        _ => unreachable!(),
    }

    0
}
