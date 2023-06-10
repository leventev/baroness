use crate::emu::Emulator;

use super::Operand;

macro_rules! branch_fn {
    ($name: ident, $flag: ident, $cond: expr) => {
        pub fn $name(emu: &mut Emulator, op: Operand) -> usize {
            let mut extra_cycles = 0;

            match op {
                Operand::Relative(off) => {
                    let final_addr = emu.regs.pc.wrapping_add_signed(off as i8 as i16);

                    if emu.regs.flags.$flag() == $cond {
                        extra_cycles += 1;

                        if final_addr & 0xFF00 != emu.regs.pc & 0xFF00 {
                            extra_cycles += 1;
                        }

                        emu.regs.pc = final_addr;
                    }
                }
                _ => unreachable!(),
            }

            extra_cycles
        }
    };
}

branch_fn!(bcc, carry, 0);
branch_fn!(bcs, carry, 1);
branch_fn!(beq, zero, 1);
branch_fn!(bne, zero, 0);
branch_fn!(bmi, negative, 1);
branch_fn!(bpl, negative, 0);
branch_fn!(bvc, overflow, 0);
branch_fn!(bvs, overflow, 1);
