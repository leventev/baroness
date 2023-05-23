use super::{instructions::Operand, Emulator};

pub fn nop(emu: &mut Emulator, op: Operand) -> usize {
    match op {
        Operand::AbsoluteIndexedX(addr) => {
            let final_addr = addr.wrapping_add(emu.regs.x as u16);
            if final_addr & 0xFF00 != addr & 0xFF00 {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}
