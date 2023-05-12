use super::{Emulator, instructions::Operand, StatusRegister};

macro_rules! branch_fn {
    ($name: ident, $flag: expr, $cond: expr) => {
        pub fn $name(emu: &mut Emulator, op: Operand) -> usize {
            let mut extra_cycles = 0;

            match op {
                Operand::Relative(off) => {
                    let final_addr = emu.regs.pc + off as u16;

                    if emu.regs.flags.contains($flag) == $cond {
                        extra_cycles += 1;

                        if final_addr & 0xFF00 != emu.regs.pc & 0xFF00 {
                            extra_cycles += 1;
                        }

                        emu.regs.pc = final_addr;
                    }
                },
                _ => unreachable!()
            }
        
            extra_cycles
        }
    };
}

branch_fn!(bcc, StatusRegister::CARRY, false);
branch_fn!(bcs, StatusRegister::CARRY, true);
branch_fn!(beq, StatusRegister::ZERO, true);
branch_fn!(bne, StatusRegister::ZERO, false);
branch_fn!(bmi, StatusRegister::NEGATIVE, true);
branch_fn!(bpl, StatusRegister::NEGATIVE, false);
branch_fn!(bvc, StatusRegister::OVERFLOW, false);
branch_fn!(bvs, StatusRegister::OVERFLOW, true);
