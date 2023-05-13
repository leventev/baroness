use super::{instructions::Operand, Emulator, StatusRegister};

pub fn adc(emu: &mut Emulator, op: Operand) -> usize {
    let mut extra_cycles = 0;

    let val = match op {
        Operand::Immediate(v) => v,
        Operand::Absolute(addr) => emu.mem[addr as usize],
        Operand::AbsoluteIndexedX(addr) => {
            let final_addr = addr + emu.regs.x as u16;
            // page crossed
            if final_addr & 0xFF00 != addr & 0xFF00 {
                extra_cycles += 1;
            }

            emu.mem[final_addr as usize]
        }
        Operand::AbsoluteIndexedY(addr) => {
            let final_addr = addr + emu.regs.y as u16;
            // page crossed
            if final_addr & 0xFF00 != addr & 0xFF00 {
                extra_cycles += 1;
            }

            emu.mem[final_addr as usize]
        }
        Operand::ZeroPage(off) => emu.mem[off as usize],
        Operand::ZeroPageIndexedX(off) => emu.mem[off.overflowing_add(emu.regs.x).0 as usize],
        Operand::ZeroPageIndexedXIndirect(off) => {
            let zp_off = off + emu.regs.x;
            let final_addr = emu.get_indirect_address(zp_off as u16);
            emu.mem[final_addr as usize]
        }
        Operand::ZeroPageIndirectIndexedY(off) => {
            let addr = emu.get_indirect_address(off as u16);
            let final_addr = addr + emu.regs.y as u16;
            // page crossed
            if final_addr & 0xFF00 != addr & 0xFF00 {
                extra_cycles += 1;
            }

            emu.mem[final_addr as usize]
        }
        _ => unreachable!(),
    };

    let carry = emu.regs.flags.bits() & StatusRegister::CARRY.bits();

    let (new_val, overflow_1) = emu.regs.a.overflowing_add_signed(val as i8);
    let (new_val, overflow_2) = new_val.overflowing_add_signed(carry as i8);
    emu.set_a(new_val);

    emu.regs
        .flags
        .set(StatusRegister::CARRY, overflow_1 | overflow_2);
    emu.regs.flags.set(
        StatusRegister::OVERFLOW,
        new_val & (1 << 7) != emu.regs.a & (1 << 7),
    );

    extra_cycles
}

pub fn cmp(emu: &mut Emulator, op: Operand) -> usize {
    let mut extra_cycles = 0;

    let val = match op {
        Operand::Immediate(v) => v,
        Operand::Absolute(addr) => emu.mem[addr as usize],
        Operand::AbsoluteIndexedX(addr) => {
            let final_addr = addr + emu.regs.x as u16;
            // page crossed
            if final_addr & 0xFF00 != addr & 0xFF00 {
                extra_cycles += 1;
            }

            emu.mem[final_addr as usize]
        }
        Operand::AbsoluteIndexedY(addr) => {
            let final_addr = addr + emu.regs.y as u16;
            // page crossed
            if final_addr & 0xFF00 != addr & 0xFF00 {
                extra_cycles += 1;
            }

            emu.mem[final_addr as usize]
        }
        Operand::ZeroPage(off) => emu.mem[off as usize],
        Operand::ZeroPageIndexedX(off) => emu.mem[off.overflowing_add(emu.regs.x).0 as usize],
        Operand::ZeroPageIndexedXIndirect(off) => {
            let zp_off = off + emu.regs.x;
            let final_addr = emu.get_indirect_address(zp_off as u16);
            emu.mem[final_addr as usize]
        }
        Operand::ZeroPageIndirectIndexedY(off) => {
            let addr = emu.get_indirect_address(off as u16);
            let final_addr = addr + emu.regs.y as u16;
            // page crossed
            if final_addr & 0xFF00 != addr & 0xFF00 {
                extra_cycles += 1;
            }

            emu.mem[final_addr as usize]
        }
        _ => unreachable!(),
    };

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
    };

    emu.regs.flags.set(StatusRegister::CARRY, emu.regs.x >= val);
    emu.regs.flags.set(StatusRegister::ZERO, emu.regs.x == val);
    emu.regs
        .flags
        .set(StatusRegister::NEGATIVE, val > emu.regs.x);

    0
}

pub fn cpy(emu: &mut Emulator, op: Operand) -> usize {
    let val = match op {
        Operand::Immediate(val) => val,
        Operand::Absolute(addr) => emu.mem[addr as usize],
        Operand::ZeroPage(off) => emu.mem[off as usize],
        _ => unreachable!(),
    };

    emu.regs.flags.set(StatusRegister::CARRY, emu.regs.y >= val);
    emu.regs.flags.set(StatusRegister::ZERO, emu.regs.y == val);
    emu.regs
        .flags
        .set(StatusRegister::NEGATIVE, val > emu.regs.y);

    0
}

pub fn sbc(emu: &mut Emulator, op: Operand) -> usize {
    let mut extra_cycles = 0;

    let val = match op {
        Operand::Immediate(v) => v,
        Operand::Absolute(addr) => emu.mem[addr as usize],
        Operand::AbsoluteIndexedX(addr) => {
            let final_addr = addr + emu.regs.x as u16;
            // page crossed
            if final_addr & 0xFF00 != addr & 0xFF00 {
                extra_cycles += 1;
            }

            emu.mem[final_addr as usize]
        }
        Operand::AbsoluteIndexedY(addr) => {
            let final_addr = addr + emu.regs.y as u16;
            // page crossed
            if final_addr & 0xFF00 != addr & 0xFF00 {
                extra_cycles += 1;
            }

            emu.mem[final_addr as usize]
        }
        Operand::ZeroPage(off) => emu.mem[off as usize],
        Operand::ZeroPageIndexedX(off) => emu.mem[off.overflowing_add(emu.regs.x).0 as usize],
        Operand::ZeroPageIndexedXIndirect(off) => {
            let zp_off = off + emu.regs.x;
            let final_addr = emu.get_indirect_address(zp_off as u16);
            emu.mem[final_addr as usize]
        }
        Operand::ZeroPageIndirectIndexedY(off) => {
            let addr = emu.get_indirect_address(off as u16);
            let final_addr = addr + emu.regs.y as u16;
            // page crossed
            if final_addr & 0xFF00 != addr & 0xFF00 {
                extra_cycles += 1;
            }

            emu.mem[final_addr as usize]
        }
        _ => unreachable!(),
    };

    let carry = !(emu.regs.flags.bits() & StatusRegister::CARRY.bits());

    let (new_val, overflow_1) = emu.regs.a.overflowing_add_signed(val as i8);
    let (new_val, overflow_2) = new_val.overflowing_add_signed(carry as i8);
    emu.set_a(new_val);

    emu.regs
        .flags
        .set(StatusRegister::CARRY, overflow_1 | overflow_2);
    emu.regs.flags.set(
        StatusRegister::OVERFLOW,
        new_val & (1 << 7) != emu.regs.a & (1 << 7),
    );

    extra_cycles
}

pub fn dec(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Absolute(addr) => {
            let val = (emu.mem[addr as usize] as i8).wrapping_add(1) as u8;
            emu.set_zero_and_negative_flags(val);
            emu.mem[addr as usize] = val;
        }
        Operand::AbsoluteIndexedX(addr) => {
            let final_addr = addr + emu.regs.x as u16;
            let val = (emu.mem[final_addr as usize] as i8).wrapping_add(1) as u8;
            emu.set_zero_and_negative_flags(val);
            emu.mem[final_addr as usize] = val;
        }
        Operand::ZeroPage(off) => {
            let val = (emu.mem[off as usize] as i8).wrapping_add(1) as u8;
            emu.set_zero_and_negative_flags(val);
            emu.mem[off as usize] = val;
        }
        Operand::ZeroPageIndexedX(off) => {
            let final_addr = off.overflowing_add(emu.regs.x).0;
            let val = (emu.mem[final_addr as usize] as i8).wrapping_add(1) as u8;
            emu.set_zero_and_negative_flags(val);
            emu.mem[final_addr as usize] = val;
        }
        _ => unreachable!(),
    }

    0
}

pub fn dex(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => emu.set_x(emu.regs.x - 1),
        _ => unreachable!(),
    }

    0
}

pub fn dey(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Implied => emu.set_x(emu.regs.y.overflowing_sub(1).0),
        _ => unreachable!(),
    }

    0
}

pub fn inc(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Absolute(addr) => {
            let val = (emu.mem[addr as usize] as i8).wrapping_add(1) as u8;
            emu.set_zero_and_negative_flags(val);
            emu.mem[addr as usize] = val;
        }
        Operand::AbsoluteIndexedX(addr) => {
            let final_addr = addr + emu.regs.x as u16;
            let val = (emu.mem[final_addr as usize] as i8).wrapping_add(1) as u8;
            emu.set_zero_and_negative_flags(val);
            emu.mem[final_addr as usize] = val;
        }
        Operand::ZeroPage(off) => {
            let val = (emu.mem[off as usize] as i8).wrapping_add(1) as u8;
            emu.set_zero_and_negative_flags(val);
            emu.mem[off as usize] = val;
        }
        Operand::ZeroPageIndexedX(off) => {
            let final_addr = off.overflowing_add(emu.regs.x).0;
            let val = (emu.mem[final_addr as usize] as i8).wrapping_add(1) as u8;
            emu.set_zero_and_negative_flags(val);
            emu.mem[final_addr as usize] = val;
        }
        _ => unreachable!(),
    }

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
