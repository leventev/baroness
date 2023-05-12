use super::{logic, branch, flags, control, stack, arithmetic, load, trans, misc, Emulator};

#[derive(Clone, Copy)]
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
    ZeroPageIndirectIndexedY
}

#[derive(Clone, Copy)]
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
    ZeroPageIndirectIndexedY(u8)
}

/// Returns the number of extra cycles
pub type InstructionCallback = fn(emu: &mut Emulator, operand: Operand) -> usize;

pub struct Instruction {
    pub name: &'static str,
    pub addressing_mode: AddressingMode,
    pub bytes: usize,
    pub cycles: usize,
    pub callback: InstructionCallback,
}

impl Instruction {
    pub const fn new(
        name: &'static str,
        addressing_mode: AddressingMode,
        bytes: usize,
        cycles: usize,
        callback: InstructionCallback,
    ) -> Instruction {
        Instruction {
            name,
            addressing_mode,
            bytes,
            cycles,
            callback,
        }
    }
}

pub const INSTRUCTIONS: [Option<Instruction>; 256] = [
    // 0x00
    Some(Instruction::new("brk", AddressingMode::Implied, 1, 7, control::brk)),
    // 0x01
    Some(Instruction::new("ora", AddressingMode::ZeroPageIndexedXIndirect, 2, 6, logic::ora)),
    // 0x02
    None,
    // 0x03
    None,
    // 0x04
    None,
    // 0x05
    Some(Instruction::new("ora", AddressingMode::ZeroPage, 2, 3, logic::ora)),
    // 0x06
    Some(Instruction::new("asl", AddressingMode::ZeroPage, 2, 5, logic::asl)),
    // 0x07
    None,
    // 0x08
    Some(Instruction::new("php", AddressingMode::Implied, 1, 3, stack::php)),
    // 0x09
    Some(Instruction::new("ora", AddressingMode::Immediate, 2, 2, logic::ora)),
    // 0x0A
    Some(Instruction::new("asl", AddressingMode::Accumulator, 1, 2, logic::asl)),
    // 0x0B
    None,
    // 0x0C
    None,
    // 0x0D
    Some(Instruction::new("ora", AddressingMode::Absolute, 2, 4, logic::ora)),
    // 0x0E
    Some(Instruction::new("asl", AddressingMode::Absolute, 3, 6, logic::asl)),
    // 0x0F
    None,
    // 0x10
    Some(Instruction::new("bpl", AddressingMode::Relative, 2, 2, branch::bpl)),
    // 0x11
    Some(Instruction::new("ora", AddressingMode::ZeroPageIndirectIndexedY, 2, 5, logic::ora)),
    // 0x12
    None,
    // 0x13
    None,
    // 0x14
    None,
    // 0x15
    Some(Instruction::new("ora", AddressingMode::ZeroPageIndexedX, 2, 4, logic::ora)),
    // 0x16
    Some(Instruction::new("asl", AddressingMode::ZeroPageIndexedX, 2, 6, logic::asl)),
    // 0x17
    None,
    // 0x18
    Some(Instruction::new("clc", AddressingMode::Implied, 1, 2, flags::clc)),
    // 0x19
    Some(Instruction::new("ora", AddressingMode::AbsoluteIndexedY, 3, 4, logic::ora)),
    // 0x1A
    None,
    // 0x1B
    None,
    // 0x1C
    None,
    // 0x1D
    Some(Instruction::new("ora", AddressingMode::AbsoluteIndexedX, 3, 4, logic::ora)),
    // 0x1E
    Some(Instruction::new("asl", AddressingMode::AbsoluteIndexedX, 3, 7, logic::asl)),
    // 0x1F,
    None,
    // 0x20
    Some(Instruction::new("jsr", AddressingMode::Absolute, 3,6, control::jsr)),
    // 0x21
    Some(Instruction::new("and", AddressingMode::ZeroPageIndexedXIndirect, 2,6, logic::and)),
    // 0x22
    None,
    // 0x23
    None,
    // 0x24
    Some(Instruction::new("bit", AddressingMode::ZeroPage, 2, 3, logic::bit)),
    // 0x25
    Some(Instruction::new("and", AddressingMode::ZeroPage, 2, 3, logic::and)),
    // 0x26
    Some(Instruction::new("rol", AddressingMode::ZeroPage, 2, 5, logic::rol)),
    // 0x27
    None,
    // 0x28
    Some(Instruction::new("plp", AddressingMode::Implied, 1, 4, stack::plp)),
    // 0x29
    Some(Instruction::new("and", AddressingMode::Immediate, 2, 2, logic::and)),
    // 0x2A
    Some(Instruction::new("rol", AddressingMode::Accumulator, 1, 2, logic::rol)),
    // 0x2B
    None,
    // 0x2C
    Some(Instruction::new("bit", AddressingMode::Absolute, 3, 4, logic::bit)),
    // 0x2D
    Some(Instruction::new("and", AddressingMode::Absolute, 3, 4, logic::and)),
    // 0x2E
    Some(Instruction::new("rol", AddressingMode::Absolute, 3, 6, logic::rol)),
    // 0x2F
    None,
    // 0x30
    Some(Instruction::new("bmi", AddressingMode::Relative, 2, 2, branch::bmi)),
    // 0x31
    Some(Instruction::new("and", AddressingMode::ZeroPageIndirectIndexedY, 2, 5, logic::and)),
    // 0x32
    None, 
    // 0x33,
    None,
    // 0x34
    None,
    // 0x35
    Some(Instruction::new("and", AddressingMode::ZeroPageIndexedX, 2, 4, logic::and)),
    // 0x36
    Some(Instruction::new("rol", AddressingMode::ZeroPageIndexedX, 2, 6, logic::rol)),
    // 0x37
    None,
    // 0x38
    Some(Instruction::new("sec", AddressingMode::Implied, 1, 2, flags::sec)),
    // 0x39
    Some(Instruction::new("and", AddressingMode::AbsoluteIndexedY, 3, 4, logic::and)),
    // 0x3A
    None,
    // 0x3B
    None,
    // 0x3C
    None,
    // 0x3D
    Some(Instruction::new("and", AddressingMode::AbsoluteIndexedX, 3, 4, logic::and)),
    // 0x3E
    Some(Instruction::new("rol", AddressingMode::AbsoluteIndexedX, 3, 7, logic::rol)),
    // 0x3F
    None,
    // 0x40
    Some(Instruction::new("rti", AddressingMode::Implied, 1, 6, control::rti)),
    // 0x41
    Some(Instruction::new("eor", AddressingMode::ZeroPageIndexedXIndirect, 2, 6, logic::eor)),
    // 0x42
    None,
    // 0x43
    None,
    // 0x44
    None,
    // 0x45
    Some(Instruction::new("eor", AddressingMode::ZeroPage, 2, 3, logic::eor)),
    // 0x46
    Some(Instruction::new("lsr", AddressingMode::ZeroPage, 2, 5, logic::lsr)),
    // 0x47
    None,
    // 0x48
    Some(Instruction::new("pha", AddressingMode::Implied, 1, 3, stack::pha)),
    // 0x49
    Some(Instruction::new("eor", AddressingMode::Immediate, 2, 2, logic::eor)),
    // 0x4A
    Some(Instruction::new("lsr", AddressingMode::Accumulator, 1, 2, logic::lsr)),
    // 0x4B
    None,
    // 0x4C
    Some(Instruction::new("jmp", AddressingMode::Absolute, 3, 3, control::jmp)),
    // 0x4D
    Some(Instruction::new("eor", AddressingMode::Absolute, 3, 4, logic::eor)),
    // 0x4E
    Some(Instruction::new("lsr", AddressingMode::Absolute, 3, 6, logic::lsr)),
    // 0x4F
    None,
    // 0x50
    Some(Instruction::new("bvc", AddressingMode::Relative, 2, 2, branch::bvc)),
    // 0x51
    Some(Instruction::new("eor", AddressingMode::ZeroPageIndirectIndexedY, 2, 5, logic::eor)),
    // 0x52
    None,
    // 0x53
    None,
    // 0x54
    None,
    // 0x55
    Some(Instruction::new("eor", AddressingMode::ZeroPageIndexedX, 2, 4, logic::eor)),
    // 0x56
    Some(Instruction::new("lsr", AddressingMode::Absolute, 3, 6, logic::lsr)),
    // 0x57
    None,
    // 0x58
    Some(Instruction::new("cli", AddressingMode::Implied, 1, 2, flags::cli)),
    // 0x59
    Some(Instruction::new("eor", AddressingMode::AbsoluteIndexedY, 3, 4, logic::eor)),
    // 0x5A
    None,
    // 0x5B
    None,
    // 0x5C
    None,
    // 0x5D
    Some(Instruction::new("eor", AddressingMode::AbsoluteIndexedX, 3, 4, logic::eor)),
    // 0x5E
    Some(Instruction::new("lsr", AddressingMode::AbsoluteIndexedX, 3, 7, logic::lsr)),
    // 0x5F
    None,
    // 0x60
    Some(Instruction::new("rts", AddressingMode::Implied, 1, 6, control::rts)),
    // 0x61
    Some(Instruction::new("adc", AddressingMode::ZeroPageIndexedXIndirect, 2, 6, arithmetic::adc)),
    // 0x62
    None,
    // 0x63
    None,
    // 0x64
    None,
    // 0x65
    Some(Instruction::new("adc", AddressingMode::ZeroPage, 2, 3, arithmetic::adc)),
    // 0x66
    Some(Instruction::new("ror", AddressingMode::ZeroPage, 2, 5, logic::ror)),
    // 0x67
    None,
    // 0x68
    Some(Instruction::new("pla", AddressingMode::Implied, 1, 4, stack::pla)),
    // 0x69
    Some(Instruction::new("adc", AddressingMode::Immediate, 2, 2, arithmetic::adc)),
    // 0x6A
    Some(Instruction::new("ror", AddressingMode::Accumulator, 1, 2, logic::ror)),
    // 0x6B
    None,
    // 0x6C
    Some(Instruction::new("jmp", AddressingMode::AbsoluteIndirect, 3, 5, control::jmp)),
    // 0x6D
    Some(Instruction::new("adc", AddressingMode::Absolute, 3, 4, arithmetic::adc)),
    // 0x6E
    Some(Instruction::new("ror", AddressingMode::Absolute, 3, 6, logic::ror)),
    // 0x6F
    None,
    // 0x70
    Some(Instruction::new("bvs", AddressingMode::Relative, 2, 2, branch::bvs)),
    // 0x71
    Some(Instruction::new("adc", AddressingMode::ZeroPageIndirectIndexedY, 2, 5, arithmetic::adc)),
    // 0x72
    None,
    // 0x73
    None,
    // 0x74
    None,
    // 0x75
    Some(Instruction::new("adc", AddressingMode::ZeroPageIndexedX, 2, 4, arithmetic::adc)),
    // 0x76
    Some(Instruction::new("ror", AddressingMode::ZeroPageIndexedX, 2, 6, logic::ror)),
    // 0x77
    None,
    // 0x78
    Some(Instruction::new("sei", AddressingMode::Implied, 1, 2, flags::sei)),
    // 0x79
    Some(Instruction::new("adc", AddressingMode::AbsoluteIndexedY, 3, 4, arithmetic::adc)),
    // 0x7A
    None,
    // 0x7B
    None,
    // 0x7C
    None,
    // 0x7D
    Some(Instruction::new("adc", AddressingMode::AbsoluteIndexedX, 3, 4, arithmetic::adc)),
    // 0x7E
    Some(Instruction::new("ror", AddressingMode::AbsoluteIndexedX, 3, 7, logic::ror)),
    // 0x7F
    None,
    // 0x80
    None,
    // 0x81
    Some(Instruction::new("sta", AddressingMode::ZeroPageIndexedXIndirect, 2, 6, load::sta)),
    // 0x82
    None,
    // 0x83
    None,
    // 0x84
    Some(Instruction::new("sty", AddressingMode::ZeroPage, 2, 3, load::sty)),
    // 0x85
    Some(Instruction::new("sta", AddressingMode::ZeroPage, 2, 3, load::sta)),
    // 0x86
    Some(Instruction::new("stx", AddressingMode::ZeroPage, 2, 3, load::stx)),
    // 0x87
    None,
    // 0x88
    Some(Instruction::new("dey", AddressingMode::Implied, 1, 2, arithmetic::dey)),
    // 0x89
    None,
    // 0x8A
    Some(Instruction::new("txa", AddressingMode::Implied, 1, 2, trans::txa)),
    // 0x8B
    None,
    // 0x8C
    Some(Instruction::new("sty", AddressingMode::Absolute, 3, 4, load::sty)),
    // 0x8D
    Some(Instruction::new("sta", AddressingMode::Absolute, 3, 4, load::sta)),
    // 0x8E
    Some(Instruction::new("stx", AddressingMode::Absolute, 3, 4, load::stx)),
    // 0x8F
    None,
    // 0x90
    Some(Instruction::new("bcc", AddressingMode::Relative, 2, 2, branch::bcc)),
    // 0x91
    Some(Instruction::new("sta", AddressingMode::ZeroPageIndirectIndexedY, 2, 6, load::sta)),
    // 0x92
    None,
    // 0x93
    None,
    // 0x94
    Some(Instruction::new("sty", AddressingMode::ZeroPageIndexedX, 2, 4, load::sty)),
    // 0x95
    Some(Instruction::new("sta", AddressingMode::ZeroPageIndexedX, 2, 4, load::sta)),
    // 0x96
    Some(Instruction::new("stx", AddressingMode::ZeroPageIndexedY, 2, 4, load::stx)),
    // 0x97
    None,
    // 0x98
    Some(Instruction::new("tya", AddressingMode::Implied, 1, 2, trans::tya)),
    // 0x99
    Some(Instruction::new("sta", AddressingMode::AbsoluteIndexedY, 3, 5, load::sta)),
    // 0x9A
    Some(Instruction::new("txs", AddressingMode::Implied, 1, 2, trans::txs)),
    // 0x9B
    None,
    // 0x9C
    None,
    // 0x9D
    Some(Instruction::new("sta", AddressingMode::AbsoluteIndexedX, 3, 5, load::sta)),
    // 0x9E
    None,
    // 0x9F
    None,
    // 0xA0
    Some(Instruction::new("ldy", AddressingMode::Immediate, 2, 2, load::ldy)),
    // 0xA1
    Some(Instruction::new("lda", AddressingMode::ZeroPageIndexedXIndirect, 2, 6, load::lda)),
    // 0xA2
    Some(Instruction::new("ldx", AddressingMode::Immediate, 2, 2, load::ldx)),
    // 0xA3
    None,
    // 0xA4
    Some(Instruction::new("ldy", AddressingMode::ZeroPage, 2, 3, load::ldy)),
    // 0xA5
    Some(Instruction::new("lda", AddressingMode::ZeroPage, 2, 3, load::lda)),
    // 0xA6
    Some(Instruction::new("ldx", AddressingMode::ZeroPage, 2, 3, load::ldx)),
    // 0xA7
    None,
    // 0xA8
    Some(Instruction::new("tay", AddressingMode::Implied, 1, 2, trans::tay)),
    // 0xA9
    Some(Instruction::new("lda", AddressingMode::Immediate, 2, 2, load::lda)),
    // 0xAA
    Some(Instruction::new("tax", AddressingMode::Implied, 1, 2, trans::tax)),
    // 0xAB
    None,
    // 0xAC
    Some(Instruction::new("ldy", AddressingMode::Absolute, 3, 4, load::ldy)),
    // 0xAD
    Some(Instruction::new("lda", AddressingMode::Absolute, 3, 4, load::lda)),
    // 0xAE
    Some(Instruction::new("ldx", AddressingMode::Absolute, 3, 4, load::ldx)),
    // 0xAF
    None,
    // 0xB0
    Some(Instruction::new("bcs", AddressingMode::Relative, 2, 2, branch::bcs)),
    // 0xB1
    Some(Instruction::new("lda", AddressingMode::ZeroPageIndirectIndexedY, 2, 5, load::lda)),
    // 0xB2
    None,
    // 0xB3
    None,
    // 0xB4
    Some(Instruction::new("ldy", AddressingMode::ZeroPageIndexedX, 2, 4, load::ldy)),
    // 0xB5
    Some(Instruction::new("lda", AddressingMode::ZeroPageIndexedX, 2, 4, load::lda)),
    // 0xB6
    Some(Instruction::new("ldx", AddressingMode::ZeroPageIndexedX, 2, 4, load::ldx)),
    // 0xB7
    None,
    // 0xB8
    Some(Instruction::new("clv", AddressingMode::Implied, 1, 2, flags::clv)),
    // 0xB9
    Some(Instruction::new("lda", AddressingMode::AbsoluteIndexedY, 3, 4, load::lda)),
    // 0xBA
    Some(Instruction::new("tsx", AddressingMode::Implied, 1, 2, trans::tsx)),
    // 0xBB
    None,
    // 0xBC
    Some(Instruction::new("ldy", AddressingMode::AbsoluteIndexedX, 3, 4, load::ldy)),
    // 0xBD
    Some(Instruction::new("lda", AddressingMode::AbsoluteIndexedX, 3, 4, load::lda)),
    // 0xBE
    Some(Instruction::new("ldx", AddressingMode::AbsoluteIndexedX, 3, 4, load::ldx)),
    // 0xBF
    None,
    // 0xC0
    Some(Instruction::new("cpy", AddressingMode::Immediate, 2, 2, arithmetic::cpy)),
    // 0xC1
    Some(Instruction::new("cmp", AddressingMode::ZeroPageIndexedXIndirect, 2, 6, arithmetic::cmp)),
    // 0xC2
    None,
    // 0xC3
    None,
    // 0xC4
    Some(Instruction::new("cpy", AddressingMode::ZeroPage, 2, 3, arithmetic::cpy)),
    // 0xC5
    Some(Instruction::new("cmp", AddressingMode::ZeroPage, 2, 3, arithmetic::cmp)),
    // 0xC6
    Some(Instruction::new("dec", AddressingMode::ZeroPage, 2, 5, arithmetic::dec)),
    // 0xC7
    None,
    // 0xC8
    Some(Instruction::new("iny", AddressingMode::Implied, 1, 2, arithmetic::iny)),
    // 0xC9
    Some(Instruction::new("cmp", AddressingMode::Immediate, 2, 2, arithmetic::cmp)),
    // 0xCA
    Some(Instruction::new("dex", AddressingMode::ZeroPage, 1, 2, arithmetic::dex)),
    // 0xCB
    None,
    // 0xCC
    Some(Instruction::new("cpy", AddressingMode::Absolute, 2, 4, arithmetic::cpy)),
    // 0xCD
    Some(Instruction::new("cmp", AddressingMode::Absolute, 3, 4, arithmetic::cmp)),
    // 0xCE
    Some(Instruction::new("dec", AddressingMode::Absolute, 3, 6, arithmetic::dec)),
    // 0xCF
    None,
    // 0xD0
    Some(Instruction::new("bne", AddressingMode::Relative, 2, 2, branch::bne)),
    // 0xD1
    Some(Instruction::new("cmp", AddressingMode::ZeroPageIndirectIndexedY, 2, 5, arithmetic::cmp)),
    // 0xD2
    None,
    // 0xD3
    None,
    // 0xD4
    None,
    // 0xD5
    Some(Instruction::new("cmp", AddressingMode::ZeroPageIndexedX, 2, 4, arithmetic::cmp)),
    // 0xD6
    Some(Instruction::new("dec", AddressingMode::ZeroPageIndexedX, 2, 6, arithmetic::dec)),
    // 0xD7
    None,
    // 0xD8
    Some(Instruction::new("cld", AddressingMode::Implied, 1, 2, flags::cld)),
    // 0xD9
    Some(Instruction::new("cmp", AddressingMode::AbsoluteIndexedY, 3, 4, arithmetic::cmp)),
    // 0xDA
    None,
    // 0xDB
    None,
    // 0xDC
    None,
    // 0xDD
    Some(Instruction::new("cmp", AddressingMode::AbsoluteIndexedX, 3, 4, arithmetic::cmp)),
    // 0xDE
    Some(Instruction::new("dec", AddressingMode::AbsoluteIndexedX, 3, 7, arithmetic::dec)),
    // 0xDF
    None,
    // 0xE0
    Some(Instruction::new("cpx", AddressingMode::Immediate, 2, 2, arithmetic::cpx)),
    // 0xE1
    Some(Instruction::new("sbc", AddressingMode::ZeroPageIndexedXIndirect, 2, 6, arithmetic::sbc)),
    // 0xE2
    None,
    // 0xE3
    None,
    // 0xE4
    Some(Instruction::new("cpx", AddressingMode::ZeroPage, 2, 3, arithmetic::cpx)),
    // 0xE5
    Some(Instruction::new("sbc", AddressingMode::ZeroPage, 2, 3, arithmetic::sbc)),
    // 0xE6
    Some(Instruction::new("inc", AddressingMode::ZeroPage, 2, 5, arithmetic::inc)),
    // 0xE7
    None,
    // 0xE8
    Some(Instruction::new("inx", AddressingMode::Implied, 1, 2, arithmetic::inx)),
    // 0xE9
    Some(Instruction::new("sbc", AddressingMode::Immediate, 2, 2, arithmetic::sbc)),
    // 0xEA
    Some(Instruction::new("nop", AddressingMode::Implied, 1, 2, misc::nop)),
    // 0xEB
    None,
    // 0xEC
    Some(Instruction::new("cpx", AddressingMode::Absolute, 3, 4, arithmetic::cpx)),
    // 0xED
    Some(Instruction::new("sbc", AddressingMode::Absolute, 3, 4, arithmetic::sbc)),
    // 0xEE
    Some(Instruction::new("inc", AddressingMode::Absolute, 3, 6, arithmetic::inc)),
    // 0xEF
    None,
    // 0xF0
    Some(Instruction::new("beq", AddressingMode::Relative, 2, 2, branch::beq)),
    // 0xF1
    Some(Instruction::new("sbc", AddressingMode::ZeroPageIndirectIndexedY, 2, 5, arithmetic::sbc)),
    // 0xF2
    None,
    // 0xF3
    None,
    // 0xF4
    None,
    // 0xF5
    Some(Instruction::new("sbc", AddressingMode::ZeroPageIndexedX, 2, 4, arithmetic::sbc)),
    // 0xF6
    Some(Instruction::new("inc", AddressingMode::ZeroPageIndexedX, 2, 6, arithmetic::inc)),
    // 0xF7
    None,
    // 0xF8
    Some(Instruction::new("sed", AddressingMode::Implied, 1, 2, flags::sed)),
    // 0xF9
    Some(Instruction::new("sbc", AddressingMode::AbsoluteIndexedY, 3, 4, arithmetic::sbc)),
    // 0xFA
    None,
    // 0xFB
    None,
    // 0xFC
    None,
    // 0xFD
    Some(Instruction::new("sbc", AddressingMode::AbsoluteIndexedX, 3, 4, arithmetic::sbc)),
    // 0xFE
    Some(Instruction::new("inc", AddressingMode::AbsoluteIndexedX, 3, 7, arithmetic::inc)),
    // 0xFF
    None
];
