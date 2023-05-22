use super::{instructions::Operand, Emulator};

pub fn nop(_: &mut Emulator, _: Operand) -> usize {
    0
}
