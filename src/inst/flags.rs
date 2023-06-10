use super::{Emulator, Operand};

macro_rules! flags_fn {
    ($name: ident, $flag: ident, $cond: expr) => {
        pub fn $name(emu: &mut Emulator, op: Operand) -> usize {
            match op {
                Operand::Implied => {
                    emu.regs.flags.$flag($cond);
                }
                _ => unreachable!(),
            }

            0
        }
    };
}

flags_fn!(clc, set_carry, 0);
flags_fn!(cld, set_decimal, 0);
flags_fn!(cli, set_interrupt_disable, 0);
flags_fn!(clv, set_overflow, 0);
flags_fn!(sec, set_carry, 1);
flags_fn!(sed, set_decimal, 1);
flags_fn!(sei, set_interrupt_disable, 1);
