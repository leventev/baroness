use super::{arithmetic, Emulator, Operand};

fn rotate_left(emu: &mut Emulator, val: u8, set_flags: bool) -> u8 {
    let new_val = val << 1 | emu.regs.flags.carry();

    if set_flags {
        let carry = val & (1 << 7) > 0;
        let negative = new_val & (1 << 7) > 0;

        emu.regs.flags.set_carry(carry.into());
        emu.regs.flags.set_negative(negative.into());
        emu.regs.flags.set_zero((new_val == 0).into());
    }

    new_val
}

fn rotate_right(emu: &mut Emulator, val: u8, set_flags: bool) -> u8 {
    let new_val = val << 1 | emu.regs.flags.carry();

    if set_flags {
        let carry = val & (1 << 0) > 0;
        let negative = new_val & (1 << 7) > 0;

        emu.regs.flags.set_carry(carry.into());
        emu.regs.flags.set_negative(negative.into());
        emu.regs.flags.set_zero((new_val == 0).into());
    }

    new_val
}

fn get_logical_operand(emu: &mut Emulator, op: Operand) -> Option<u16> {
    match op {
        Operand::Accumulator => None,
        _ => Some(emu.get_addr_from_operand(op)),
    }
}

pub fn asl(emu: &mut Emulator, op: Operand) -> usize {
    let addr = get_logical_operand(emu, op);

    let (new_val, carry, negative) = if let Some(addr) = addr {
        let val = emu.read(addr);

        let new_val = val << 1;
        emu.write(addr, new_val);

        (new_val, val & (1 << 7) > 0, new_val & (1 << 7) > 0)
    } else {
        let val = emu.regs.a;

        let new_val = val << 1;
        emu.regs.a = new_val;

        (new_val, val & (1 << 7) > 0, new_val & (1 << 7) > 0)
    };

    emu.regs.flags.set_carry(carry.into());
    emu.regs.flags.set_negative(negative.into());
    emu.regs.flags.set_zero((new_val == 0).into());

    0
}

pub fn lsr(emu: &mut Emulator, op: Operand) -> usize {
    let addr = get_logical_operand(emu, op);

    let (new_val, carry) = if let Some(addr) = addr {
        let val = emu.read(addr);

        let new_val = val >> 1;
        emu.write(addr, new_val);

        (new_val, val & (1 << 0) > 0)
    } else {
        let val = emu.regs.a;

        let new_val = val >> 1;
        emu.regs.a = new_val;

        (new_val, val & (1 << 0) > 0)
    };

    emu.regs.flags.set_carry(carry.into());
    emu.regs.flags.set_negative(0);
    emu.regs.flags.set_zero((new_val == 0).into());

    0
}

pub fn rol(emu: &mut Emulator, op: Operand) -> usize {
    match get_logical_operand(emu, op) {
        Some(addr) => {
            let val = emu.read(addr);
            let rotated = rotate_left(emu, val, true);
            emu.write(addr, rotated)
        }
        None => {
            let val = emu.regs.a;
            let rotated = rotate_left(emu, val, true);
            emu.regs.a = rotated;
        }
    };

    0
}

pub fn ror(emu: &mut Emulator, op: Operand) -> usize {
    match get_logical_operand(emu, op) {
        Some(addr) => {
            let val = emu.read(addr);
            let rotated = rotate_right(emu, val, true);
            emu.write(addr, rotated)
        }
        None => {
            let val = emu.regs.a;
            let rotated = rotate_right(emu, val, true);
            emu.regs.a = rotated;
        }
    };

    0
}

pub fn and(emu: &mut Emulator, op: Operand) -> usize {
    let val = emu.get_val_from_operand(op);

    let new_val = val & emu.regs.a;
    emu.set_a(new_val);

    0
}

pub fn bit(emu: &mut Emulator, op: Operand) -> usize {
    let val = emu.get_val_from_operand(op);

    emu.regs.flags.set_negative((val & (1 << 7) != 0).into());
    emu.regs.flags.set_overflow((val & (1 << 6) != 0).into());

    let result = val & emu.regs.a;
    emu.regs.flags.set_zero((result == 0).into());

    0
}

pub fn eor(emu: &mut Emulator, op: Operand) -> usize {
    let val = emu.get_val_from_operand(op);

    let new_val = val ^ emu.regs.a;
    emu.set_a(new_val);

    0
}

pub fn ora(emu: &mut Emulator, op: Operand) -> usize {
    let val = emu.get_val_from_operand(op);

    let new_val = val | emu.regs.a;
    emu.set_a(new_val);

    0
}

//
//
// Unofficial
//
//

pub fn slo(emu: &mut Emulator, op: Operand) -> usize {
    let addr = emu.get_addr_from_operand(op);
    let val = emu.read(addr);
    let new_val = val << 1;

    emu.regs.flags.set_carry((val & (1 << 7) > 0).into());

    let res = emu.regs.a | new_val;

    emu.set_a(res);
    emu.write(addr, new_val);

    0
}

pub fn sre(emu: &mut Emulator, op: Operand) -> usize {
    let addr = emu.get_addr_from_operand(op);
    let val = emu.read(addr);
    let new_val = val >> 1;

    emu.regs.flags.set_carry((val & (1 << 0) > 0).into());

    let res = emu.regs.a ^ new_val;

    emu.set_a(res);
    emu.write(addr, new_val);

    0
}

pub fn rla(emu: &mut Emulator, op: Operand) -> usize {
    let addr = emu.get_addr_from_operand(op);
    let val = emu.read(addr);

    let new_val = rotate_left(emu, val, true);
    let new_acc = emu.regs.a & new_val;

    emu.set_a(new_acc);
    emu.write(addr, new_val);

    0
}

pub fn rra(emu: &mut Emulator, op: Operand) -> usize {
    let addr = emu.get_addr_from_operand(op);
    let val = emu.read(addr);

    let new_val = rotate_right(emu, val, true);

    arithmetic::do_adc(emu, new_val);
    emu.write(addr, new_val);

    0
}
