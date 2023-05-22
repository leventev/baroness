use super::{instructions::Operand, Emulator};

pub fn get_val_from_operand(emu: &mut Emulator, op: Operand) -> u8 {
    if let Operand::Immediate(val) = op {
        val
    } else {
        let addr = get_addr_from_operand(emu, op);
        emu.mem[addr as usize]
    }
}

pub fn get_addr_from_operand(emu: &mut Emulator, op: Operand) -> u16 {
    match op {
        Operand::Absolute(addr) => addr,
        Operand::AbsoluteIndexedX(addr) => addr.wrapping_add(emu.regs.x as u16),
        Operand::AbsoluteIndexedY(addr) => addr.wrapping_add(emu.regs.y as u16),
        Operand::ZeroPage(off) => off as u16,
        Operand::ZeroPageIndexedX(off) => off.wrapping_add(emu.regs.x) as u16,
        Operand::ZeroPageIndexedXIndirect(off) => {
            let zp_off = off + emu.regs.x;
            emu.get_zero_page_indirect_address(zp_off)
        }
        Operand::ZeroPageIndirectIndexedY(off) => {
            let addr = emu.get_zero_page_indirect_address(off);
            addr + emu.regs.y as u16
        }
        _ => unreachable!(),
    }
}
