use super::{Emulator, instructions::Operand, StatusRegister};

macro_rules! flags_fn {
    ($name: ident, $flag: expr, $cond: expr) => {
        pub fn $name(emu: &mut Emulator, op: Operand) -> usize {
            match op {
                Operand::Implied => {
                    emu.regs.flags.set($flag, $cond);
                },
                _ => unreachable!()
            }
        
            0
        }
    };
}

flags_fn!(clc, StatusRegister::CARRY, false);
flags_fn!(cld, StatusRegister::DECIMAL, false);
flags_fn!(cli, StatusRegister::INTERRUPT_DISABLE, false);
flags_fn!(clv, StatusRegister::OVERFLOW, false);
flags_fn!(sec, StatusRegister::CARRY, true);
flags_fn!(sed, StatusRegister::DECIMAL, true);
flags_fn!(sei, StatusRegister::INTERRUPT_DISABLE, true);
