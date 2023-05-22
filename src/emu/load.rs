use super::{instructions::Operand, Emulator, StatusRegister};

pub fn lda(emu: &mut Emulator, op: Operand) -> usize {
    let mut extra_cycles = 0;

    match op {
        Operand::Immediate(val) => {
            emu.set_a(val);
        }
        Operand::Absolute(addr) => {
            emu.set_a(emu.mem[addr as usize]);
        }
        Operand::AbsoluteIndexedX(addr) => {
            let final_addr = addr.wrapping_add(emu.regs.x as u16);
            // page crossed
            if final_addr & 0xFF00 != addr & 0xFF00 {
                extra_cycles += 1;
            }

            emu.set_a(emu.mem[final_addr as usize]);
        }
        Operand::AbsoluteIndexedY(addr) => {
            let final_addr = addr.wrapping_add(emu.regs.y as u16);
            // page crossed
            if final_addr & 0xFF00 != addr & 0xFF00 {
                extra_cycles += 1;
            }

            emu.set_a(emu.mem[final_addr as usize]);
        }
        Operand::ZeroPage(off) => {
            emu.set_a(emu.mem[off as usize]);
        }
        Operand::ZeroPageIndexedX(off) => {
            let final_off = off.wrapping_add(emu.regs.x);
            emu.set_a(emu.mem[final_off as usize]);
        }
        Operand::ZeroPageIndexedXIndirect(off) => {
            let zp_off = off.wrapping_add(emu.regs.x);
            let final_addr = emu.get_zero_page_indirect_address(zp_off);
            emu.set_a(emu.mem[final_addr as usize]);
        }
        Operand::ZeroPageIndirectIndexedY(off) => {
            let addr = emu.get_zero_page_indirect_address(off);
            let final_addr = addr.wrapping_add(emu.regs.y as u16);
            // page crossed
            if final_addr & 0xFF00 != addr & 0xFF00 {
                extra_cycles += 1;
            }

            emu.set_a(emu.mem[final_addr as usize]);
        }
        _ => unreachable!(),
    }

    extra_cycles
}

pub fn ldx(emu: &mut Emulator, op: Operand) -> usize {
    let mut extra_cycles = 0;

    match op {
        Operand::Immediate(val) => {
            emu.set_x(val);
        }
        Operand::Absolute(addr) => {
            emu.set_x(emu.mem[addr as usize]);
        }
        Operand::AbsoluteIndexedY(addr) => {
            let final_addr = addr.wrapping_add(emu.regs.y as u16);
            // page crossed
            if final_addr & 0xFF00 != addr & 0xFF00 {
                extra_cycles += 1;
            }

            emu.set_x(emu.mem[final_addr as usize]);
        }
        Operand::ZeroPage(off) => {
            emu.set_x(emu.mem[off as usize]);
        }
        Operand::ZeroPageIndexedY(off) => {
            let final_off = off.wrapping_add(emu.regs.y);
            emu.set_x(emu.mem[final_off as usize]);
        }
        _ => unreachable!(),
    }

    extra_cycles
}

pub fn ldy(emu: &mut Emulator, op: Operand) -> usize {
    let mut extra_cycles = 0;

    match op {
        Operand::Immediate(val) => {
            emu.set_y(val);
        }
        Operand::Absolute(addr) => {
            emu.set_y(emu.mem[addr as usize]);
        }
        Operand::AbsoluteIndexedX(addr) => {
            let final_addr = addr.wrapping_add(emu.regs.x as u16);
            // page crossed
            if final_addr & 0xFF00 != addr & 0xFF00 {
                extra_cycles += 1;
            }

            emu.set_y(emu.mem[final_addr as usize]);
        }
        Operand::ZeroPage(off) => {
            emu.set_y(emu.mem[off as usize]);
        }
        Operand::ZeroPageIndexedX(off) => {
            let final_off = off.wrapping_add(emu.regs.x);
            emu.set_y(emu.mem[final_off as usize]);
        }
        _ => unreachable!(),
    }

    extra_cycles
}

pub fn sta(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Absolute(addr) => {
            emu.mem[addr as usize] = emu.regs.a;
        }
        Operand::AbsoluteIndexedX(addr) => {
            let final_addr = addr.wrapping_add(emu.regs.x as u16);
            emu.mem[final_addr as usize] = emu.regs.a;
        }
        Operand::AbsoluteIndexedY(addr) => {
            let final_addr = addr.wrapping_add(emu.regs.y as u16);
            emu.mem[final_addr as usize] = emu.regs.a;
        }
        Operand::ZeroPage(off) => {
            emu.mem[off as usize] = emu.regs.a;
        }
        Operand::ZeroPageIndexedX(off) => {
            let final_off = off.wrapping_add(emu.regs.x);
            emu.mem[final_off as usize] = emu.regs.a;
        }
        Operand::ZeroPageIndexedY(off) => {
            let final_off = off.wrapping_add(emu.regs.y);
            emu.mem[final_off as usize] = emu.regs.a;
        }
        Operand::ZeroPageIndexedXIndirect(off) => {
            let zp_off = off.wrapping_add(emu.regs.x);
            let final_addr = emu.get_zero_page_indirect_address(zp_off);
            emu.mem[final_addr as usize] = emu.regs.a;
        }
        Operand::ZeroPageIndirectIndexedY(off) => {
            let addr = emu.get_zero_page_indirect_address(off);
            let final_addr = addr.wrapping_add(emu.regs.y as u16);
            emu.mem[final_addr as usize] = emu.regs.a;
        }
        _ => unreachable!(),
    }

    0
}

pub fn stx(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Absolute(addr) => {
            emu.mem[addr as usize] = emu.regs.x;
        }
        Operand::ZeroPage(off) => {
            emu.mem[off as usize] = emu.regs.x;
        }
        Operand::ZeroPageIndexedY(off) => {
            let final_off = off.wrapping_add(emu.regs.y);
            emu.mem[final_off as usize] = emu.regs.x;
        }
        _ => unreachable!(),
    }

    0
}

pub fn sty(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::Absolute(addr) => {
            emu.mem[addr as usize] = emu.regs.y;
        }
        Operand::ZeroPage(off) => {
            emu.mem[off as usize] = emu.regs.y;
        }
        Operand::ZeroPageIndexedX(off) => {
            let final_off = off.wrapping_add(emu.regs.x);
            emu.mem[final_off as usize] = emu.regs.y;
        }
        _ => unreachable!(),
    }

    0
}

// Unofficial

pub fn lax(emu: &mut Emulator, op: Operand) -> usize {
    let mut extra_cycles = 0;

    let val = match op {
        Operand::Immediate(val) => val,
        Operand::Absolute(addr) => emu.mem[addr as usize],
        Operand::AbsoluteIndexedY(addr) => {
            let final_addr = addr.wrapping_add(emu.regs.y as u16);
            // page crossed
            if final_addr & 0x00FF != addr & 0x00FF {
                extra_cycles += 1;
            }

            emu.mem[final_addr as usize]
        }
        Operand::ZeroPage(off) => emu.mem[off as usize],
        Operand::ZeroPageIndexedY(off) => {
            let addr = off.wrapping_add(emu.regs.y);
            emu.mem[addr as usize]
        }
        Operand::ZeroPageIndexedXIndirect(off) => {
            let zp_off = off.wrapping_add(emu.regs.x);
            let final_addr = emu.get_zero_page_indirect_address(zp_off);
            emu.mem[final_addr as usize]
        }
        Operand::ZeroPageIndirectIndexedY(off) => {
            let addr = emu.get_zero_page_indirect_address(off);
            let final_addr = addr.wrapping_add(emu.regs.y as u16);
            // page crossed
            if final_addr & 0x00FF != addr & 0x00FF {
                extra_cycles += 1;
            }

            emu.mem[final_addr as usize]
        }
        _ => unreachable!(),
    };

    emu.regs.a = val;
    emu.regs.x = val;

    emu.regs.flags.set(StatusRegister::ZERO, val == 0);
    emu.regs
        .flags
        .set(StatusRegister::NEGATIVE, val & (1 << 7) > 0);

    extra_cycles
}

pub fn sax(emu: &mut Emulator, op: Operand) -> usize {
    let res = emu.regs.a & emu.regs.x;

    match op {
        Operand::Absolute(addr) => emu.mem[addr as usize] = res,
        Operand::ZeroPage(off) => emu.mem[off as usize] = res,
        Operand::ZeroPageIndexedY(off) => {
            let addr = off.wrapping_add(emu.regs.y);
            emu.mem[addr as usize] = res;
        }
        Operand::ZeroPageIndexedXIndirect(off) => {
            let zp_off = off.wrapping_add(emu.regs.x);
            let final_addr = emu.get_zero_page_indirect_address(zp_off);
            emu.mem[final_addr as usize] = res;
        }
        _ => unreachable!(),
    };

    0
}
