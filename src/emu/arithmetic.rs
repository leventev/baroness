use super::{
    instructions::Operand,
    utils::{self, get_addr_from_operand},
    Emulator, StatusRegister,
};

pub fn do_adc(emu: &mut Emulator, val: u8) {
    let acc = emu.regs.a as usize;
    let val = val as usize;

    let carry = if emu.regs.flags.contains(StatusRegister::CARRY) {
        1
    } else {
        0
    };

    let temp = acc + val + carry;

    let carry = temp >> 8 > 0;
    let overflow = (!(acc ^ val) & (acc ^ temp) & 0x80) > 0;
    let new_a = (temp & 0xFF) as u8;

    emu.regs.flags.set(StatusRegister::OVERFLOW, overflow);
    emu.regs.flags.set(StatusRegister::CARRY, carry);

    emu.set_a(new_a);
}

fn do_sbc(emu: &mut Emulator, val: u8) {
    do_adc(emu, val ^ 0xFF);
}

fn do_cmp(emu: &mut Emulator, val: u8) {
    let val = emu.regs.a.wrapping_sub(val);

    emu.regs.flags.set(StatusRegister::CARRY, val <= emu.regs.a);

    emu.regs
        .flags
        .set(StatusRegister::NEGATIVE, val & (1 << 7) != 0);

    emu.regs.flags.set(StatusRegister::ZERO, val == 0);
}

pub fn adc(emu: &mut Emulator, op: Operand) -> usize {
    let val = utils::get_val_from_operand(emu, op);

    do_adc(emu, val);

    0
}

pub fn cmp(emu: &mut Emulator, op: Operand) -> usize {
    let val = utils::get_val_from_operand(emu, op);

    do_cmp(emu, val);

    0
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
    let val = utils::get_val_from_operand(emu, op);

    do_sbc(emu, val);

    0
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

//
//
// Unofficial
//
//

pub fn dcp(emu: &mut Emulator, op: Operand) -> usize {
    let addr = get_addr_from_operand(emu, op);
    let val = emu.mem[addr as usize];

    let subbed = (val as i8).wrapping_sub(1);
    do_cmp(emu, subbed as u8);
    emu.mem[addr as usize] = subbed as u8;

    0
}

pub fn isc(emu: &mut Emulator, op: Operand) -> usize {
    let addr = get_addr_from_operand(emu, op);
    let val = emu.mem[addr as usize];

    let added = (val as i8).wrapping_add(1);
    do_sbc(emu, added as u8);
    emu.mem[addr as usize] = added as u8;

    0
}
