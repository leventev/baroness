use super::{instructions::Operand, utils, Emulator, StatusRegister};

fn get_logical_operand(emu: &mut Emulator, op: Operand) -> Option<u16> {
    match op {
        Operand::Accumulator => None,
        Operand::Absolute(addr) => Some(addr),
        Operand::AbsoluteIndexedX(addr) => Some(addr + emu.regs.x as u16),
        Operand::ZeroPage(off) => Some(off as u16),
        Operand::ZeroPageIndexedX(off) => Some(off.wrapping_add(emu.regs.x) as u16),
        _ => unreachable!(),
    }
}

pub fn asl(emu: &mut Emulator, op: Operand) -> usize {
    let addr = get_logical_operand(emu, op);

    let (new_val, carry, negative) = if let Some(addr) = addr {
        let val = emu.mem[addr as usize];

        let new_val = val << 1;
        emu.mem[addr as usize] = new_val;

        (new_val, val & (1 << 7) > 0, new_val & (1 << 7) > 0)
    } else {
        let val = emu.regs.a;

        let new_val = val << 1;
        emu.regs.a = new_val;

        (new_val, val & (1 << 7) > 0, new_val & (1 << 7) > 0)
    };

    emu.regs.flags.set(StatusRegister::CARRY, carry);
    emu.regs.flags.set(StatusRegister::NEGATIVE, negative);
    emu.regs.flags.set(StatusRegister::ZERO, new_val == 0);

    0
}

pub fn lsr(emu: &mut Emulator, op: Operand) -> usize {
    let addr = get_logical_operand(emu, op);

    let (new_val, carry) = if let Some(addr) = addr {
        let val = emu.mem[addr as usize];

        let new_val = val >> 1;
        emu.mem[addr as usize] = new_val;

        (new_val, val & (1 << 0) > 0)
    } else {
        let val = emu.regs.a;

        let new_val = val >> 1;
        emu.regs.a = new_val;

        (new_val, val & (1 << 0) > 0)
    };

    emu.regs.flags.set(StatusRegister::CARRY, carry);
    emu.regs.flags.set(StatusRegister::NEGATIVE, false);
    emu.regs.flags.set(StatusRegister::ZERO, new_val == 0);

    0
}

pub fn rol(emu: &mut Emulator, op: Operand) -> usize {
    let addr = get_logical_operand(emu, op);

    let (new_val, carry, negative) = if let Some(addr) = addr {
        let val = emu.mem[addr as usize];

        let mut new_val = val << 1;
        if emu.regs.flags.contains(StatusRegister::CARRY) {
            new_val |= 1 << 0;
        }
        emu.mem[addr as usize] = new_val;

        (new_val, val & (1 << 7) > 0, val & (1 << 6) > 0)
    } else {
        let val = emu.regs.a;

        let mut new_val = val << 1;
        if emu.regs.flags.contains(StatusRegister::CARRY) {
            new_val |= 1 << 0;
        }
        emu.regs.a = new_val;

        (new_val, val & (1 << 7) > 0, val & (1 << 6) > 0)
    };

    emu.regs.flags.set(StatusRegister::CARRY, carry);
    emu.regs.flags.set(StatusRegister::NEGATIVE, negative);
    emu.regs.flags.set(StatusRegister::ZERO, new_val == 0);

    0
}

pub fn ror(emu: &mut Emulator, op: Operand) -> usize {
    let addr = get_logical_operand(emu, op);

    let (new_val, carry, negative) = if let Some(addr) = addr {
        let val = emu.mem[addr as usize];

        let mut new_val = val >> 1;
        if emu.regs.flags.contains(StatusRegister::CARRY) {
            new_val |= 1 << 7;
        }
        emu.mem[addr as usize] = new_val;

        (new_val, val & (1 << 0) > 0, new_val & (1 << 7) > 0)
    } else {
        let val = emu.regs.a;

        let mut new_val = val >> 1;
        if emu.regs.flags.contains(StatusRegister::CARRY) {
            new_val |= 1 << 7;
        }
        emu.regs.a = new_val;

        (new_val, val & (1 << 0) > 0, new_val & (1 << 7) > 0)
    };

    emu.regs.flags.set(StatusRegister::CARRY, carry);
    emu.regs.flags.set(StatusRegister::NEGATIVE, negative);
    emu.regs.flags.set(StatusRegister::ZERO, new_val == 0);

    0
}

pub fn and(emu: &mut Emulator, op: Operand) -> usize {
    let (val, extra_cycles) = utils::get_val_from_operand(emu, op);

    let new_val = val & emu.regs.a;
    emu.set_a(new_val);

    extra_cycles
}

pub fn bit(emu: &mut Emulator, op: Operand) -> usize {
    let val = match op {
        Operand::Absolute(addr) => emu.mem[addr as usize],
        Operand::ZeroPage(off) => emu.mem[off as usize],
        _ => unreachable!(),
    };

    emu.regs
        .flags
        .set(StatusRegister::NEGATIVE, val & (1 << 7) != 0);
    emu.regs
        .flags
        .set(StatusRegister::OVERFLOW, val & (1 << 6) != 0);

    let result = val & emu.regs.a;
    emu.regs.flags.set(StatusRegister::ZERO, result == 0);

    0
}

pub fn eor(emu: &mut Emulator, op: Operand) -> usize {
    let (val, extra_cycles) = utils::get_val_from_operand(emu, op);

    let new_val = val ^ emu.regs.a;
    emu.set_a(new_val);

    extra_cycles
}

pub fn ora(emu: &mut Emulator, op: Operand) -> usize {
    let (val, extra_cycles) = utils::get_val_from_operand(emu, op);

    let new_val = val | emu.regs.a;
    emu.set_a(new_val);

    extra_cycles
}
