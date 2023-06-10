use crate::emu::Emulator;

use super::Operand;

pub fn do_adc(emu: &mut Emulator, val: u8) {
    let acc = emu.regs.a as usize;
    let val = val as usize;

    let carry = emu.regs.flags.carry() as usize;

    let temp = acc + val + carry;

    let carry = temp >> 8 > 0;
    let overflow = (!(acc ^ val) & (acc ^ temp) & 0x80) > 0;
    let new_a = (temp & 0xFF) as u8;

    emu.regs.flags.set_overflow(overflow as u8);
    emu.regs.flags.set_carry(carry as u8);

    emu.set_a(new_a);
}

fn do_sbc(emu: &mut Emulator, val: u8) {
    do_adc(emu, val ^ 0xFF);
}

fn do_cmp(emu: &mut Emulator, val: u8) {
    let val = emu.regs.a.wrapping_sub(val);

    emu.regs.flags.set_carry((val <= emu.regs.a).into());

    emu.regs.flags.set_negative((val & (1 << 7)) >> 7);

    emu.regs.flags.set_zero((val == 0).into());
}

pub fn adc(emu: &mut Emulator, op: Operand) -> usize {
    let val = emu.get_val_from_operand(op);
    do_adc(emu, val);

    0
}

pub fn cmp(emu: &mut Emulator, op: Operand) -> usize {
    let val = emu.get_val_from_operand(op);
    do_cmp(emu, val);

    0
}

pub fn cpx(emu: &mut Emulator, op: Operand) -> usize {
    let val = emu.get_val_from_operand(op) as i8;
    let reg = emu.regs.x as i8;

    let res = reg.wrapping_sub(val);

    emu.regs.flags.set_carry((reg as u8 >= val as u8).into());
    emu.regs.flags.set_zero((res == 0).into());
    emu.regs.flags.set_negative((res < 0).into());

    0
}

pub fn cpy(emu: &mut Emulator, op: Operand) -> usize {
    let val = emu.get_val_from_operand(op) as i8;
    let reg = emu.regs.y as i8;

    let res = reg.wrapping_sub(val);

    emu.regs.flags.set_carry((reg as u8 >= val as u8).into());
    emu.regs.flags.set_zero((res == 0).into());
    emu.regs.flags.set_negative((res < 0).into());

    0
}

pub fn sbc(emu: &mut Emulator, op: Operand) -> usize {
    let val = emu.get_val_from_operand(op);
    do_sbc(emu, val);

    0
}

pub fn dec(emu: &mut Emulator, op: Operand) -> usize {
    let addr = emu.get_addr_from_operand(op);

    let val = emu.read(addr) as i8;
    let new_val = val.wrapping_sub(1) as u8;

    emu.set_zero_and_negative_flags(new_val);
    emu.write(addr, new_val);

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
    let addr = emu.get_addr_from_operand(op);

    let val = emu.read(addr) as i8;
    let new_val = val.wrapping_add(1) as u8;

    emu.set_zero_and_negative_flags(new_val);
    emu.write(addr, new_val);

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
    let addr = emu.get_addr_from_operand(op);
    let val = emu.read(addr);

    let subbed = (val as i8).wrapping_sub(1);
    do_cmp(emu, subbed as u8);
    emu.write(addr, subbed as u8);

    0
}

pub fn isc(emu: &mut Emulator, op: Operand) -> usize {
    let addr = emu.get_addr_from_operand(op);
    let val = emu.read(addr);

    let added = (val as i8).wrapping_add(1);
    do_sbc(emu, added as u8);
    emu.write(addr, added as u8);

    0
}
