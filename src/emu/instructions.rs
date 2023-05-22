use super::{arithmetic, branch, control, flags, load, logic, misc, stack, trans, Emulator};

#[derive(Clone, Copy, Debug)]
pub enum AddressingMode {
    Accumulator,
    Implied,
    Immediate,
    Absolute,
    ZeroPage,
    Relative,
    AbsoluteIndirect,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    ZeroPageIndexedX,
    ZeroPageIndexedY,
    ZeroPageIndexedXIndirect,
    ZeroPageIndirectIndexedY,
}

#[derive(Clone, Copy, Debug)]
pub enum Operand {
    Accumulator,
    Implied,
    Immediate(u8),
    Absolute(u16),
    ZeroPage(u8),
    Relative(u8),
    AbsoluteIndirect(u16),
    AbsoluteIndexedX(u16),
    AbsoluteIndexedY(u16),
    ZeroPageIndexedX(u8),
    ZeroPageIndexedY(u8),
    ZeroPageIndexedXIndirect(u8),
    ZeroPageIndirectIndexedY(u8),
}

/// Returns the number of extra cycles
pub type InstructionCallback = fn(emu: &mut Emulator, operand: Operand) -> usize;

pub struct Instruction {
    pub name: &'static str,
    pub addressing_mode: AddressingMode,
    pub bytes: usize,
    pub cycles: usize,
    pub callback: InstructionCallback,
    pub unofficial: bool,
}

impl Instruction {
    pub const fn new(
        name: &'static str,
        addressing_mode: AddressingMode,
        bytes: usize,
        cycles: usize,
        callback: InstructionCallback,
        unofficial: bool,
    ) -> Instruction {
        Instruction {
            name,
            addressing_mode,
            bytes,
            cycles,
            callback,
            unofficial,
        }
    }
}

macro_rules! inst {
    ($name: expr, $addressing_mode: expr, $bytes: expr, $cycles: expr, $callback: expr) => {
        Some(Instruction::new(
            $name,
            $addressing_mode,
            $bytes,
            $cycles,
            $callback,
            false,
        ))
    };
}

macro_rules! unofficial_inst {
    ($name: expr, $addressing_mode: expr, $bytes: expr, $cycles: expr, $callback: expr) => {
        Some(Instruction::new(
            $name,
            $addressing_mode,
            $bytes,
            $cycles,
            $callback,
            true,
        ))
    };
}

pub const INSTRUCTIONS: [Option<Instruction>; 256] = [
    // 0x00
    inst!("brk", AddressingMode::Implied, 1, 7, control::brk),
    // 0x01
    inst!(
        "ora",
        AddressingMode::ZeroPageIndexedXIndirect,
        2,
        6,
        logic::ora
    ),
    // 0x02
    None,
    // 0x03
    unofficial_inst!(
        "slo",
        AddressingMode::ZeroPageIndexedXIndirect,
        2,
        8,
        logic::slo
    ),
    // 0x04
    unofficial_inst!("nop", AddressingMode::ZeroPage, 2, 3, misc::nop),
    // 0x05
    inst!("ora", AddressingMode::ZeroPage, 2, 3, logic::ora),
    // 0x06
    inst!("asl", AddressingMode::ZeroPage, 2, 5, logic::asl),
    // 0x07
    unofficial_inst!("slo", AddressingMode::ZeroPage, 2, 5, logic::slo),
    // 0x08
    inst!("php", AddressingMode::Implied, 1, 3, stack::php),
    // 0x09
    inst!("ora", AddressingMode::Immediate, 2, 2, logic::ora),
    // 0x0A
    inst!("asl", AddressingMode::Accumulator, 1, 2, logic::asl),
    // 0x0B
    None,
    // 0x0C
    unofficial_inst!("nop", AddressingMode::Absolute, 3, 4, misc::nop),
    // 0x0D
    inst!("ora", AddressingMode::Absolute, 3, 4, logic::ora),
    // 0x0E
    inst!("asl", AddressingMode::Absolute, 3, 6, logic::asl),
    // 0x0F
    unofficial_inst!("slo", AddressingMode::Absolute, 3, 6, logic::slo),
    // 0x10
    inst!("bpl", AddressingMode::Relative, 2, 2, branch::bpl),
    // 0x11
    inst!(
        "ora",
        AddressingMode::ZeroPageIndirectIndexedY,
        2,
        5,
        logic::ora
    ),
    // 0x12
    None,
    // 0x13
    unofficial_inst!(
        "slo",
        AddressingMode::ZeroPageIndirectIndexedY,
        2,
        8,
        logic::slo
    ),
    // 0x14
    unofficial_inst!("nop", AddressingMode::ZeroPageIndexedX, 2, 4, misc::nop),
    // 0x15
    inst!("ora", AddressingMode::ZeroPageIndexedX, 2, 4, logic::ora),
    // 0x16
    inst!("asl", AddressingMode::ZeroPageIndexedX, 2, 6, logic::asl),
    // 0x17
    unofficial_inst!("slo", AddressingMode::ZeroPageIndexedX, 2, 6, logic::slo),
    // 0x18
    inst!("clc", AddressingMode::Implied, 1, 2, flags::clc),
    // 0x19
    inst!("ora", AddressingMode::AbsoluteIndexedY, 3, 4, logic::ora),
    // 0x1A
    unofficial_inst!("nop", AddressingMode::Implied, 1, 2, misc::nop),
    // 0x1B
    unofficial_inst!("slo", AddressingMode::AbsoluteIndexedY, 3, 7, logic::slo),
    // 0x1C
    unofficial_inst!("nop", AddressingMode::AbsoluteIndexedX, 3, 4, misc::nop),
    // 0x1D
    inst!("ora", AddressingMode::AbsoluteIndexedX, 3, 4, logic::ora),
    // 0x1E
    inst!("asl", AddressingMode::AbsoluteIndexedX, 3, 7, logic::asl),
    // 0x1F,
    unofficial_inst!("slo", AddressingMode::AbsoluteIndexedX, 3, 7, logic::slo),
    // 0x20
    inst!("jsr", AddressingMode::Absolute, 3, 6, control::jsr),
    // 0x21
    inst!(
        "and",
        AddressingMode::ZeroPageIndexedXIndirect,
        2,
        6,
        logic::and
    ),
    // 0x22
    None,
    // 0x23
    unofficial_inst!(
        "rla",
        AddressingMode::ZeroPageIndexedXIndirect,
        2,
        8,
        logic::rla
    ),
    // 0x24
    inst!("bit", AddressingMode::ZeroPage, 2, 3, logic::bit),
    // 0x25
    inst!("and", AddressingMode::ZeroPage, 2, 3, logic::and),
    // 0x26
    inst!("rol", AddressingMode::ZeroPage, 2, 5, logic::rol),
    // 0x27
    unofficial_inst!("rla", AddressingMode::ZeroPage, 2, 5, logic::rla),
    // 0x28
    inst!("plp", AddressingMode::Implied, 1, 4, stack::plp),
    // 0x29
    inst!("and", AddressingMode::Immediate, 2, 2, logic::and),
    // 0x2A
    inst!("rol", AddressingMode::Accumulator, 1, 2, logic::rol),
    // 0x2B
    None,
    // 0x2C
    inst!("bit", AddressingMode::Absolute, 3, 4, logic::bit),
    // 0x2D
    inst!("and", AddressingMode::Absolute, 3, 4, logic::and),
    // 0x2E
    inst!("rol", AddressingMode::Absolute, 3, 6, logic::rol),
    // 0x2F
    unofficial_inst!("rla", AddressingMode::Absolute, 3, 6, logic::rla),
    // 0x30
    inst!("bmi", AddressingMode::Relative, 2, 2, branch::bmi),
    // 0x31
    inst!(
        "and",
        AddressingMode::ZeroPageIndirectIndexedY,
        2,
        5,
        logic::and
    ),
    // 0x32
    None,
    // 0x33,
    unofficial_inst!(
        "rla",
        AddressingMode::ZeroPageIndirectIndexedY,
        2,
        8,
        logic::rla
    ),
    // 0x34
    unofficial_inst!("nop", AddressingMode::ZeroPageIndexedX, 2, 4, misc::nop),
    // 0x35
    inst!("and", AddressingMode::ZeroPageIndexedX, 2, 4, logic::and),
    // 0x36
    inst!("rol", AddressingMode::ZeroPageIndexedX, 2, 6, logic::rol),
    // 0x37
    unofficial_inst!("rla", AddressingMode::ZeroPageIndexedX, 2, 6, logic::rla),
    // 0x38
    inst!("sec", AddressingMode::Implied, 1, 2, flags::sec),
    // 0x39
    inst!("and", AddressingMode::AbsoluteIndexedY, 3, 4, logic::and),
    // 0x3A
    unofficial_inst!("nop", AddressingMode::Implied, 1, 2, misc::nop),
    // 0x3B
    unofficial_inst!("rla", AddressingMode::AbsoluteIndexedY, 3, 7, logic::rla),
    // 0x3C
    unofficial_inst!("nop", AddressingMode::AbsoluteIndexedX, 3, 4, misc::nop),
    // 0x3D
    inst!("and", AddressingMode::AbsoluteIndexedX, 3, 4, logic::and),
    // 0x3E
    inst!("rol", AddressingMode::AbsoluteIndexedX, 3, 7, logic::rol),
    // 0x3F
    unofficial_inst!("rla", AddressingMode::AbsoluteIndexedX, 3, 7, logic::rla),
    // 0x40
    inst!("rti", AddressingMode::Implied, 1, 6, control::rti),
    // 0x41
    inst!(
        "eor",
        AddressingMode::ZeroPageIndexedXIndirect,
        2,
        6,
        logic::eor
    ),
    // 0x42
    None,
    // 0x43
    unofficial_inst!(
        "sre",
        AddressingMode::ZeroPageIndexedXIndirect,
        2,
        8,
        logic::sre
    ),
    // 0x44
    unofficial_inst!("nop", AddressingMode::ZeroPage, 2, 3, misc::nop),
    // 0x45
    inst!("eor", AddressingMode::ZeroPage, 2, 3, logic::eor),
    // 0x46
    inst!("lsr", AddressingMode::ZeroPage, 2, 5, logic::lsr),
    // 0x47
    unofficial_inst!("sre", AddressingMode::ZeroPage, 2, 5, logic::sre),
    // 0x48
    inst!("pha", AddressingMode::Implied, 1, 3, stack::pha),
    // 0x49
    inst!("eor", AddressingMode::Immediate, 2, 2, logic::eor),
    // 0x4A
    inst!("lsr", AddressingMode::Accumulator, 1, 2, logic::lsr),
    // 0x4B
    None,
    // 0x4C
    inst!("jmp", AddressingMode::Absolute, 3, 3, control::jmp),
    // 0x4D
    inst!("eor", AddressingMode::Absolute, 3, 4, logic::eor),
    // 0x4E
    inst!("lsr", AddressingMode::Absolute, 3, 6, logic::lsr),
    // 0x4F
    unofficial_inst!("sre", AddressingMode::Absolute, 3, 6, logic::sre),
    // 0x50
    inst!("bvc", AddressingMode::Relative, 2, 2, branch::bvc),
    // 0x51
    inst!(
        "eor",
        AddressingMode::ZeroPageIndirectIndexedY,
        2,
        5,
        logic::eor
    ),
    // 0x52
    None,
    // 0x53
    unofficial_inst!(
        "sre",
        AddressingMode::ZeroPageIndirectIndexedY,
        2,
        8,
        logic::sre
    ),
    // 0x54
    unofficial_inst!("nop", AddressingMode::ZeroPageIndexedX, 2, 4, misc::nop),
    // 0x55
    inst!("eor", AddressingMode::ZeroPageIndexedX, 2, 4, logic::eor),
    // 0x56
    inst!("lsr", AddressingMode::ZeroPageIndexedX, 2, 6, logic::lsr),
    // 0x57
    unofficial_inst!("sre", AddressingMode::ZeroPageIndexedX, 2, 6, logic::sre),
    // 0x58
    inst!("cli", AddressingMode::Implied, 1, 2, flags::cli),
    // 0x59
    inst!("eor", AddressingMode::AbsoluteIndexedY, 3, 4, logic::eor),
    // 0x5A
    unofficial_inst!("nop", AddressingMode::Implied, 1, 2, misc::nop),
    // 0x5B
    unofficial_inst!("sre", AddressingMode::AbsoluteIndexedY, 3, 7, logic::sre),
    // 0x5C
    unofficial_inst!("nop", AddressingMode::AbsoluteIndexedX, 3, 4, misc::nop),
    // 0x5D
    inst!("eor", AddressingMode::AbsoluteIndexedX, 3, 4, logic::eor),
    // 0x5E
    inst!("lsr", AddressingMode::AbsoluteIndexedX, 3, 7, logic::lsr),
    // 0x5F
    unofficial_inst!("sre", AddressingMode::AbsoluteIndexedX, 3, 7, logic::sre),
    // 0x60
    inst!("rts", AddressingMode::Implied, 1, 6, control::rts),
    // 0x61
    inst!(
        "adc",
        AddressingMode::ZeroPageIndexedXIndirect,
        2,
        6,
        arithmetic::adc
    ),
    // 0x62
    None,
    // 0x63
    unofficial_inst!(
        "rra",
        AddressingMode::ZeroPageIndexedXIndirect,
        2,
        8,
        logic::rra
    ),
    // 0x64
    unofficial_inst!("nop", AddressingMode::ZeroPage, 2, 3, misc::nop),
    // 0x65
    inst!("adc", AddressingMode::ZeroPage, 2, 3, arithmetic::adc),
    // 0x66
    inst!("ror", AddressingMode::ZeroPage, 2, 5, logic::ror),
    // 0x67
    unofficial_inst!("rra", AddressingMode::ZeroPage, 2, 5, logic::rra),
    // 0x68
    inst!("pla", AddressingMode::Implied, 1, 4, stack::pla),
    // 0x69
    inst!("adc", AddressingMode::Immediate, 2, 2, arithmetic::adc),
    // 0x6A
    inst!("ror", AddressingMode::Accumulator, 1, 2, logic::ror),
    // 0x6B
    None,
    // 0x6C
    inst!("jmp", AddressingMode::AbsoluteIndirect, 3, 5, control::jmp),
    // 0x6D
    inst!("adc", AddressingMode::Absolute, 3, 4, arithmetic::adc),
    // 0x6E
    inst!("ror", AddressingMode::Absolute, 3, 6, logic::ror),
    // 0x6F
    unofficial_inst!("rra", AddressingMode::Absolute, 3, 6, logic::rra),
    // 0x70
    inst!("bvs", AddressingMode::Relative, 2, 2, branch::bvs),
    // 0x71
    inst!(
        "adc",
        AddressingMode::ZeroPageIndirectIndexedY,
        2,
        5,
        arithmetic::adc
    ),
    // 0x72
    None,
    // 0x73
    unofficial_inst!(
        "rra",
        AddressingMode::ZeroPageIndirectIndexedY,
        2,
        8,
        logic::rra
    ),
    // 0x74
    unofficial_inst!("nop", AddressingMode::ZeroPageIndexedX, 2, 4, misc::nop),
    // 0x75
    inst!(
        "adc",
        AddressingMode::ZeroPageIndexedX,
        2,
        4,
        arithmetic::adc
    ),
    // 0x76
    inst!("ror", AddressingMode::ZeroPageIndexedX, 2, 6, logic::ror),
    // 0x77
    unofficial_inst!("rra", AddressingMode::ZeroPageIndexedX, 2, 6, logic::rra),
    // 0x78
    inst!("sei", AddressingMode::Implied, 1, 2, flags::sei),
    // 0x79
    inst!(
        "adc",
        AddressingMode::AbsoluteIndexedY,
        3,
        4,
        arithmetic::adc
    ),
    // 0x7A
    unofficial_inst!("nop", AddressingMode::Implied, 1, 2, misc::nop),
    // 0x7B
    unofficial_inst!("rra", AddressingMode::AbsoluteIndexedY, 3, 7, logic::rra),
    // 0x7C
    unofficial_inst!("nop", AddressingMode::AbsoluteIndexedX, 3, 4, misc::nop),
    // 0x7D
    inst!(
        "adc",
        AddressingMode::AbsoluteIndexedX,
        3,
        4,
        arithmetic::adc
    ),
    // 0x7E
    inst!("ror", AddressingMode::AbsoluteIndexedX, 3, 7, logic::ror),
    // 0x7F
    unofficial_inst!("rra", AddressingMode::AbsoluteIndexedX, 3, 7, logic::rra),
    // 0x80
    unofficial_inst!("nop", AddressingMode::Absolute, 2, 2, misc::nop),
    // 0x81
    inst!(
        "sta",
        AddressingMode::ZeroPageIndexedXIndirect,
        2,
        6,
        load::sta
    ),
    // 0x82
    unofficial_inst!("nop", AddressingMode::Absolute, 2, 2, misc::nop),
    // 0x83
    unofficial_inst!(
        "sax",
        AddressingMode::ZeroPageIndexedXIndirect,
        2,
        6,
        load::sax
    ),
    // 0x84
    inst!("sty", AddressingMode::ZeroPage, 2, 3, load::sty),
    // 0x85
    inst!("sta", AddressingMode::ZeroPage, 2, 3, load::sta),
    // 0x86
    inst!("stx", AddressingMode::ZeroPage, 2, 3, load::stx),
    // 0x87
    unofficial_inst!("sax", AddressingMode::ZeroPage, 2, 3, load::sax),
    // 0x88
    inst!("dey", AddressingMode::Implied, 1, 2, arithmetic::dey),
    // 0x89
    unofficial_inst!("nop", AddressingMode::Absolute, 2, 2, misc::nop),
    // 0x8A
    inst!("txa", AddressingMode::Implied, 1, 2, trans::txa),
    // 0x8B
    None,
    // 0x8C
    inst!("sty", AddressingMode::Absolute, 3, 4, load::sty),
    // 0x8D
    inst!("sta", AddressingMode::Absolute, 3, 4, load::sta),
    // 0x8E
    inst!("stx", AddressingMode::Absolute, 3, 4, load::stx),
    // 0x8F
    unofficial_inst!("sax", AddressingMode::Absolute, 3, 4, load::sax),
    // 0x90
    inst!("bcc", AddressingMode::Relative, 2, 2, branch::bcc),
    // 0x91
    inst!(
        "sta",
        AddressingMode::ZeroPageIndirectIndexedY,
        2,
        6,
        load::sta
    ),
    // 0x92
    None,
    // 0x93
    None,
    // 0x94
    inst!("sty", AddressingMode::ZeroPageIndexedX, 2, 4, load::sty),
    // 0x95
    inst!("sta", AddressingMode::ZeroPageIndexedX, 2, 4, load::sta),
    // 0x96
    inst!("stx", AddressingMode::ZeroPageIndexedY, 2, 4, load::stx),
    // 0x97
    unofficial_inst!("sax", AddressingMode::ZeroPageIndexedY, 2, 4, load::sax),
    // 0x98
    inst!("tya", AddressingMode::Implied, 1, 2, trans::tya),
    // 0x99
    inst!("sta", AddressingMode::AbsoluteIndexedY, 3, 5, load::sta),
    // 0x9A
    inst!("txs", AddressingMode::Implied, 1, 2, trans::txs),
    // 0x9B
    None,
    // 0x9C
    None,
    // 0x9D
    inst!("sta", AddressingMode::AbsoluteIndexedX, 3, 5, load::sta),
    // 0x9E
    None,
    // 0x9F
    None,
    // 0xA0
    inst!("ldy", AddressingMode::Immediate, 2, 2, load::ldy),
    // 0xA1
    inst!(
        "lda",
        AddressingMode::ZeroPageIndexedXIndirect,
        2,
        6,
        load::lda
    ),
    // 0xA2
    inst!("ldx", AddressingMode::Immediate, 2, 2, load::ldx),
    // 0xA3
    unofficial_inst!(
        "lax",
        AddressingMode::ZeroPageIndexedXIndirect,
        2,
        6,
        load::lax
    ),
    // 0xA4
    inst!("ldy", AddressingMode::ZeroPage, 2, 3, load::ldy),
    // 0xA5
    inst!("lda", AddressingMode::ZeroPage, 2, 3, load::lda),
    // 0xA6
    inst!("ldx", AddressingMode::ZeroPage, 2, 3, load::ldx),
    // 0xA7
    unofficial_inst!("lax", AddressingMode::ZeroPage, 2, 3, load::lax),
    // 0xA8
    inst!("tay", AddressingMode::Implied, 1, 2, trans::tay),
    // 0xA9
    inst!("lda", AddressingMode::Immediate, 2, 2, load::lda),
    // 0xAA
    inst!("tax", AddressingMode::Implied, 1, 2, trans::tax),
    // 0xAB
    unofficial_inst!("lax", AddressingMode::Immediate, 2, 2, load::lax),
    // 0xAC
    inst!("ldy", AddressingMode::Absolute, 3, 4, load::ldy),
    // 0xAD
    inst!("lda", AddressingMode::Absolute, 3, 4, load::lda),
    // 0xAE
    inst!("ldx", AddressingMode::Absolute, 3, 4, load::ldx),
    // 0xAF
    unofficial_inst!("lax", AddressingMode::Absolute, 3, 4, load::lax),
    // 0xB0
    inst!("bcs", AddressingMode::Relative, 2, 2, branch::bcs),
    // 0xB1
    inst!(
        "lda",
        AddressingMode::ZeroPageIndirectIndexedY,
        2,
        5,
        load::lda
    ),
    // 0xB2
    None,
    // 0xB3
    unofficial_inst!(
        "lax",
        AddressingMode::ZeroPageIndirectIndexedY,
        2,
        5,
        load::lax
    ),
    // 0xB4
    inst!("ldy", AddressingMode::ZeroPageIndexedX, 2, 4, load::ldy),
    // 0xB5
    inst!("lda", AddressingMode::ZeroPageIndexedX, 2, 4, load::lda),
    // 0xB6
    inst!("ldx", AddressingMode::ZeroPageIndexedY, 2, 4, load::ldx),
    // 0xB7
    unofficial_inst!("lax", AddressingMode::ZeroPageIndexedY, 2, 4, load::lax),
    // 0xB8
    inst!("clv", AddressingMode::Implied, 1, 2, flags::clv),
    // 0xB9
    inst!("lda", AddressingMode::AbsoluteIndexedY, 3, 4, load::lda),
    // 0xBA
    inst!("tsx", AddressingMode::Implied, 1, 2, trans::tsx),
    // 0xBB
    None,
    // 0xBC
    inst!("ldy", AddressingMode::AbsoluteIndexedX, 3, 4, load::ldy),
    // 0xBD
    inst!("lda", AddressingMode::AbsoluteIndexedX, 3, 4, load::lda),
    // 0xBE
    inst!("ldx", AddressingMode::AbsoluteIndexedY, 3, 4, load::ldx),
    // 0xBF
    unofficial_inst!("lax", AddressingMode::AbsoluteIndexedY, 3, 4, load::lax),
    // 0xC0
    inst!("cpy", AddressingMode::Immediate, 2, 2, arithmetic::cpy),
    // 0xC1
    inst!(
        "cmp",
        AddressingMode::ZeroPageIndexedXIndirect,
        2,
        6,
        arithmetic::cmp
    ),
    // 0xC2
    unofficial_inst!("nop", AddressingMode::Absolute, 2, 2, misc::nop),
    // 0xC3
    unofficial_inst!(
        "dcp",
        AddressingMode::ZeroPageIndexedXIndirect,
        2,
        8,
        arithmetic::dcp
    ),
    // 0xC4
    inst!("cpy", AddressingMode::ZeroPage, 2, 3, arithmetic::cpy),
    // 0xC5
    inst!("cmp", AddressingMode::ZeroPage, 2, 3, arithmetic::cmp),
    // 0xC6
    inst!("dec", AddressingMode::ZeroPage, 2, 5, arithmetic::dec),
    // 0xC7
    unofficial_inst!("dcp", AddressingMode::ZeroPage, 2, 5, arithmetic::dcp),
    // 0xC8
    inst!("iny", AddressingMode::Implied, 1, 2, arithmetic::iny),
    // 0xC9
    inst!("cmp", AddressingMode::Immediate, 2, 2, arithmetic::cmp),
    // 0xCA
    inst!("dex", AddressingMode::Implied, 1, 2, arithmetic::dex),
    // 0xCB
    None,
    // 0xCC
    inst!("cpy", AddressingMode::Absolute, 3, 4, arithmetic::cpy),
    // 0xCD
    inst!("cmp", AddressingMode::Absolute, 3, 4, arithmetic::cmp),
    // 0xCE
    inst!("dec", AddressingMode::Absolute, 3, 6, arithmetic::dec),
    // 0xCF
    unofficial_inst!("dcp", AddressingMode::Absolute, 3, 6, arithmetic::dcp),
    // 0xD0
    inst!("bne", AddressingMode::Relative, 2, 2, branch::bne),
    // 0xD1
    inst!(
        "cmp",
        AddressingMode::ZeroPageIndirectIndexedY,
        2,
        5,
        arithmetic::cmp
    ),
    // 0xD2
    None,
    // 0xD3
    unofficial_inst!(
        "dcp",
        AddressingMode::ZeroPageIndirectIndexedY,
        2,
        8,
        arithmetic::dcp
    ),
    // 0xD4
    unofficial_inst!("nop", AddressingMode::ZeroPageIndexedX, 2, 4, misc::nop),
    // 0xD5
    inst!(
        "cmp",
        AddressingMode::ZeroPageIndexedX,
        2,
        4,
        arithmetic::cmp
    ),
    // 0xD6
    inst!(
        "dec",
        AddressingMode::ZeroPageIndexedX,
        2,
        6,
        arithmetic::dec
    ),
    // 0xD7
    unofficial_inst!(
        "dcp",
        AddressingMode::ZeroPageIndexedX,
        2,
        6,
        arithmetic::dcp
    ),
    // 0xD8
    inst!("cld", AddressingMode::Implied, 1, 2, flags::cld),
    // 0xD9
    inst!(
        "cmp",
        AddressingMode::AbsoluteIndexedY,
        3,
        4,
        arithmetic::cmp
    ),
    // 0xDA
    unofficial_inst!("nop", AddressingMode::Implied, 1, 2, misc::nop),
    // 0xDB
    unofficial_inst!(
        "dcp",
        AddressingMode::AbsoluteIndexedY,
        3,
        7,
        arithmetic::dcp
    ),
    // 0xDC
    unofficial_inst!("nop", AddressingMode::AbsoluteIndexedX, 3, 4, misc::nop),
    // 0xDD
    inst!(
        "cmp",
        AddressingMode::AbsoluteIndexedX,
        3,
        4,
        arithmetic::cmp
    ),
    // 0xDE
    inst!(
        "dec",
        AddressingMode::AbsoluteIndexedX,
        3,
        7,
        arithmetic::dec
    ),
    // 0xDF
    unofficial_inst!(
        "dcp",
        AddressingMode::AbsoluteIndexedX,
        3,
        7,
        arithmetic::dcp
    ),
    // 0xE0
    inst!("cpx", AddressingMode::Immediate, 2, 2, arithmetic::cpx),
    // 0xE1
    inst!(
        "sbc",
        AddressingMode::ZeroPageIndexedXIndirect,
        2,
        6,
        arithmetic::sbc
    ),
    // 0xE2
    unofficial_inst!("nop", AddressingMode::Absolute, 2, 2, misc::nop),
    // 0xE3
    unofficial_inst!(
        "isc",
        AddressingMode::ZeroPageIndexedXIndirect,
        2,
        8,
        arithmetic::isc
    ),
    // 0xE4
    inst!("cpx", AddressingMode::ZeroPage, 2, 3, arithmetic::cpx),
    // 0xE5
    inst!("sbc", AddressingMode::ZeroPage, 2, 3, arithmetic::sbc),
    // 0xE6
    inst!("inc", AddressingMode::ZeroPage, 2, 5, arithmetic::inc),
    // 0xE7
    unofficial_inst!("isc", AddressingMode::ZeroPage, 2, 5, arithmetic::isc),
    // 0xE8
    inst!("inx", AddressingMode::Implied, 1, 2, arithmetic::inx),
    // 0xE9
    inst!("sbc", AddressingMode::Immediate, 2, 2, arithmetic::sbc),
    // 0xEA
    inst!("nop", AddressingMode::Implied, 1, 2, misc::nop),
    // 0xEB
    unofficial_inst!("sbc", AddressingMode::Immediate, 2, 2, arithmetic::sbc),
    // 0xEC
    inst!("cpx", AddressingMode::Absolute, 3, 4, arithmetic::cpx),
    // 0xED
    inst!("sbc", AddressingMode::Absolute, 3, 4, arithmetic::sbc),
    // 0xEE
    inst!("inc", AddressingMode::Absolute, 3, 6, arithmetic::inc),
    // 0xEF
    unofficial_inst!("isc", AddressingMode::Absolute, 3, 6, arithmetic::isc),
    // 0xF0
    inst!("beq", AddressingMode::Relative, 2, 2, branch::beq),
    // 0xF1
    inst!(
        "sbc",
        AddressingMode::ZeroPageIndirectIndexedY,
        2,
        5,
        arithmetic::sbc
    ),
    // 0xF2
    None,
    // 0xF3
    unofficial_inst!(
        "isc",
        AddressingMode::ZeroPageIndirectIndexedY,
        2,
        8,
        arithmetic::isc
    ),
    // 0xF4
    unofficial_inst!("nop", AddressingMode::ZeroPageIndexedX, 2, 4, misc::nop),
    // 0xF5
    inst!(
        "sbc",
        AddressingMode::ZeroPageIndexedX,
        2,
        4,
        arithmetic::sbc
    ),
    // 0xF6
    inst!(
        "inc",
        AddressingMode::ZeroPageIndexedX,
        2,
        6,
        arithmetic::inc
    ),
    // 0xF7
    unofficial_inst!(
        "isc",
        AddressingMode::ZeroPageIndexedX,
        2,
        6,
        arithmetic::isc
    ),
    // 0xF8
    inst!("sed", AddressingMode::Implied, 1, 2, flags::sed),
    // 0xF9
    inst!(
        "sbc",
        AddressingMode::AbsoluteIndexedY,
        3,
        4,
        arithmetic::sbc
    ),
    // 0xFA
    unofficial_inst!("nop", AddressingMode::Implied, 1, 2, misc::nop),
    // 0xFB
    unofficial_inst!(
        "isc",
        AddressingMode::AbsoluteIndexedY,
        3,
        7,
        arithmetic::isc
    ),
    // 0xFC
    unofficial_inst!("nop", AddressingMode::AbsoluteIndexedX, 3, 4, misc::nop),
    // 0xFD
    inst!(
        "sbc",
        AddressingMode::AbsoluteIndexedX,
        3,
        4,
        arithmetic::sbc
    ),
    // 0xFE
    inst!(
        "inc",
        AddressingMode::AbsoluteIndexedX,
        3,
        7,
        arithmetic::inc
    ),
    // 0xFF
    unofficial_inst!(
        "isc",
        AddressingMode::AbsoluteIndexedX,
        3,
        7,
        arithmetic::isc
    ),
];
