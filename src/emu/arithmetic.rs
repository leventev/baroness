use super::{instructions::Operand, utils, Emulator, StatusRegister};

pub fn adc(emu: &mut Emulator, op: Operand) -> usize {
    let (val, extra_cycles) = utils::get_val_from_operand(emu, op);
    let val = val as i8;

    let acc = emu.regs.a as isize;
    let value = val as isize;

    let (new_val, carry_over, overflow) = if value & (1 << 7) > 0 {
        let mut full_val = acc.wrapping_sub(-value & 0xFF);
        if full_val < 0 {
            full_val |= 1 << 7;
        }

        let mut carry_over = false;

        let mut new_val = full_val as i8;
        if emu.regs.flags.contains(StatusRegister::CARRY) {
            let res = new_val.wrapping_add(1);
            carry_over = res > new_val;
            new_val = res;
        }

        (new_val, carry_over, new_val > acc as i8)
    } else {
        let full_val = acc.wrapping_add(value);

        let mut new_val = full_val as i8;
        if emu.regs.flags.contains(StatusRegister::CARRY) {
            new_val = new_val.wrapping_add(1);
        }

        (new_val, full_val >> 8 != 0, new_val < acc as i8)
    };

    emu.regs.flags.set(StatusRegister::CARRY, carry_over);
    emu.regs.flags.set(StatusRegister::OVERFLOW, overflow);

    emu.set_a(new_val as u8);

    extra_cycles
}

pub fn cmp(emu: &mut Emulator, op: Operand) -> usize {
    let (val, extra_cycles) = utils::get_val_from_operand(emu, op);
    let (val, _) = emu.regs.a.overflowing_sub(val);

    emu.regs.flags.set(StatusRegister::CARRY, val <= emu.regs.a);

    emu.regs
        .flags
        .set(StatusRegister::NEGATIVE, val & (1 << 7) != 0);

    emu.regs.flags.set(StatusRegister::ZERO, val == 0);

    extra_cycles
}

pub fn cpx(emu: &mut Emulator, op: Operand) -> usize {
    let val = match op {
        Operand::Immediate(val) => val,
        Operand::Absolute(addr) => emu.mem[addr as usize],
        Operand::ZeroPage(off) => emu.mem[off as usize],
        _ => unreachable!(),
    } as i8;

    let reg = emu.regs.x as i8;
    let res = reg.wrapping_sub(val);

    emu.regs
        .flags
        .set(StatusRegister::CARRY, reg as u8 >= val as u8);
    emu.regs.flags.set(StatusRegister::ZERO, res == 0);
    emu.regs.flags.set(StatusRegister::NEGATIVE, res < 0);

    0
}

pub fn cpy(emu: &mut Emulator, op: Operand) -> usize {
    let val = match op {
        Operand::Immediate(val) => val,
        Operand::Absolute(addr) => emu.mem[addr as usize],
        Operand::ZeroPage(off) => emu.mem[off as usize],
        _ => unreachable!(),
    } as i8;

    let reg = emu.regs.y as i8;
    let res = reg.wrapping_sub(val);

    emu.regs
        .flags
        .set(StatusRegister::CARRY, reg as u8 >= val as u8);
    emu.regs.flags.set(StatusRegister::ZERO, res == 0);
    emu.regs.flags.set(StatusRegister::NEGATIVE, res < 0);

    0
}

pub fn sbc(emu: &mut Emulator, op: Operand) -> usize {
    let (val, extra_cycles) = utils::get_val_from_operand(emu, op);
    let val = val as i8;

    let acc = emu.regs.a as isize;
    let value = val as isize;

    let (new_val, overflow) = if value & (1 << 7) > 0 {
        let mut full_val = acc.wrapping_add(-value & 0xFF);
        if full_val < 0 {
            full_val |= 1 << 7;
        }

        let mut new_val = full_val as i8;
        if !emu.regs.flags.contains(StatusRegister::CARRY) {
            let res = new_val.wrapping_sub(1);
            new_val = res;
        }

        (new_val, new_val < acc as i8)
    } else {
        let full_val = acc.wrapping_sub(value);

        let mut new_val = full_val as i8;
        if !emu.regs.flags.contains(StatusRegister::CARRY) {
            new_val = new_val.wrapping_sub(1);
        }

        (new_val, new_val > acc as i8)
    };

    emu.regs.flags.set(StatusRegister::CARRY, new_val >= 0);
    emu.regs.flags.set(StatusRegister::OVERFLOW, overflow);

    emu.set_a(new_val as u8);

    extra_cycles
}

pub fn dec(emu: &mut Emulator, op: Operand) -> usize {
    let addr = match op {
        Operand::Absolute(addr) => addr,
        Operand::AbsoluteIndexedX(addr) => addr + emu.regs.x as u16,
        Operand::ZeroPage(off) => off as u16,
        Operand::ZeroPageIndexedX(off) => off.wrapping_add(emu.regs.x) as u16,
        _ => unreachable!(),
    };

    let val = emu.mem[addr as usize] as i8;
    let new_val = val.wrapping_sub(1) as u8;

    emu.set_zero_and_negative_flags(new_val);
    emu.mem[addr as usize] = new_val;

    0
}

pub fn dex(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => emu.set_x(emu.regs.x.wrapping_sub(1)),
        _ => unreachable!(),
    }

    0
}

pub fn dey(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => emu.set_y(emu.regs.y.wrapping_sub(1)),
        _ => unreachable!(),
    }

    0
}

pub fn inc(emu: &mut Emulator, op: Operand) -> usize {
    let addr = match op {
        Operand::Absolute(addr) => addr,
        Operand::AbsoluteIndexedX(addr) => addr + emu.regs.x as u16,
        Operand::ZeroPage(off) => off as u16,
        Operand::ZeroPageIndexedX(off) => off.wrapping_add(emu.regs.x) as u16,
        _ => unreachable!(),
    };

    let val = emu.mem[addr as usize] as i8;
    let new_val = val.wrapping_add(1) as u8;

    emu.set_zero_and_negative_flags(new_val);
    emu.mem[addr as usize] = new_val;

    0
}

pub fn inx(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => emu.set_x(emu.regs.x.wrapping_add(1)),
        _ => unreachable!(),
    }

    0
}

pub fn iny(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => emu.set_y(emu.regs.y.wrapping_add(1)),
        _ => unreachable!(),
    }

    0
}
