use phf::{phf_map, Map};

use crate::cpu::{AddressingMode, Cpu};

pub struct OpCode {
    pub code: u8,
    pub name: &'static str,
    pub op: fn(&mut crate::cpu::Cpu, AddressingMode),
    pub mode: AddressingMode,
    // not used anywhere but tests
    pub bytes: u8,
    pub cycles: u8,
}

impl OpCode {
    #[allow(clippy::similar_names)]
    pub const fn new(
        code: u8,
        name: &'static str,
        op: fn(&mut crate::cpu::Cpu, AddressingMode),
        mode: AddressingMode,
        bytes: u8,
        cycles: u8,
    ) -> Self {
        Self {
            code,
            name,
            op,
            mode,
            bytes,
            cycles,
        }
    }
}

pub static OPCODES: Map<u8, OpCode> = phf_map! {
    0xA9u8 => OpCode::new(0xA9, "LDA", Cpu::lda, AddressingMode::Immediate, 2, 2),
    0xA5u8 => OpCode::new(0xA5, "LDA", Cpu::lda, AddressingMode::ZeroPage, 2, 3),
    0xB5u8 => OpCode::new(0xB5, "LDA", Cpu::lda, AddressingMode::ZeroPageX, 2, 4),
    0xADu8 => OpCode::new(0xAD, "LDA", Cpu::lda, AddressingMode::Absolute, 3, 4),
    0xBDu8 => OpCode::new(0xBD, "LDA", Cpu::lda, AddressingMode::AbsoluteX, 3, 4),
    0xB9u8 => OpCode::new(0xB9, "LDA", Cpu::lda, AddressingMode::AbsoluteY, 3, 4),
    0xA1u8 => OpCode::new(0xA1, "LDA", Cpu::lda, AddressingMode::IndirectX, 2, 6),
    0xB1u8 => OpCode::new(0xB1, "LDA", Cpu::lda, AddressingMode::IndirectY, 2, 5),

    0xA2u8 => OpCode::new(0xA2, "LDX", Cpu::ldx, AddressingMode::Immediate, 2, 2),
    0xA6u8 => OpCode::new(0xA6, "LDX", Cpu::ldx, AddressingMode::ZeroPage, 2, 3),
    0xB6u8 => OpCode::new(0xB6, "LDX", Cpu::ldx, AddressingMode::ZeroPageY, 2, 4),
    0xAEu8 => OpCode::new(0xAE, "LDX", Cpu::ldx, AddressingMode::Absolute, 3, 4),
    0xBEu8 => OpCode::new(0xBE, "LDX", Cpu::ldx, AddressingMode::AbsoluteY, 3, 4),

    0xA0u8 => OpCode::new(0xA0, "LDY", Cpu::ldy, AddressingMode::Immediate, 2, 2),
    0xA4u8 => OpCode::new(0xA4, "LDY", Cpu::ldy, AddressingMode::ZeroPage, 2, 3),
    0xB4u8 => OpCode::new(0xB4, "LDY", Cpu::ldy, AddressingMode::ZeroPageX, 2, 4),
    0xACu8 => OpCode::new(0xAC, "LDY", Cpu::ldy, AddressingMode::Absolute, 3, 4),
    0xBCu8 => OpCode::new(0xBC, "LDY", Cpu::ldy, AddressingMode::AbsoluteX, 3, 4),

    0x85u8 => OpCode::new(0x85, "STA", Cpu::sta, AddressingMode::ZeroPage, 2, 3),
    0x95u8 => OpCode::new(0x95, "STA", Cpu::sta, AddressingMode::ZeroPageX, 2, 4),
    0x8Du8 => OpCode::new(0x8D, "STA", Cpu::sta, AddressingMode::Absolute, 3, 4),
    0x9Du8 => OpCode::new(0x9D, "STA", Cpu::sta, AddressingMode::AbsoluteX, 3, 5),
    0x99u8 => OpCode::new(0x99, "STA", Cpu::sta, AddressingMode::AbsoluteY, 3, 5),
    0x81u8 => OpCode::new(0x81, "STA", Cpu::sta, AddressingMode::IndirectX, 2, 6),
    0x91u8 => OpCode::new(0x91, "STA", Cpu::sta, AddressingMode::IndirectY, 2, 6),

    0x86u8 => OpCode::new(0x86, "STX", Cpu::stx, AddressingMode::ZeroPage, 2, 3),
    0x96u8 => OpCode::new(0x96, "STX", Cpu::stx, AddressingMode::ZeroPageY, 2, 4),
    0x8Eu8 => OpCode::new(0x8E, "STX", Cpu::stx, AddressingMode::Absolute, 3, 4),

    0x84u8 => OpCode::new(0x84, "STY", Cpu::sty, AddressingMode::ZeroPage, 2, 3),
    0x94u8 => OpCode::new(0x94, "STY", Cpu::sty, AddressingMode::ZeroPageX, 2, 4),
    0x8Cu8 => OpCode::new(0x8C, "STY", Cpu::sty, AddressingMode::Absolute, 3, 4),

    0xAAu8 => OpCode::new(0xAA, "TAX", Cpu::tax, AddressingMode::NoneAddressing, 1, 2),

    0xA8u8 => OpCode::new(0xA8, "TAY", Cpu::tay, AddressingMode::NoneAddressing, 1, 2),

    0xBAu8 => OpCode::new(0xBA, "TSX", Cpu::tsx, AddressingMode::NoneAddressing, 1, 2),

    0x8Au8 => OpCode::new(0x8A, "TXA", Cpu::txa, AddressingMode::NoneAddressing, 1, 2),

    0x9Au8 => OpCode::new(0x9A, "TXS", Cpu::txs, AddressingMode::NoneAddressing, 1, 2),

    0x98u8 => OpCode::new(0x98, "TYA", Cpu::tya, AddressingMode::NoneAddressing, 1, 2),

    0xE8u8 => OpCode::new(0xE8, "INX", Cpu::inx, AddressingMode::NoneAddressing, 1, 2),

    0xC8u8 => OpCode::new(0xC8, "INY", Cpu::iny, AddressingMode::NoneAddressing, 1, 2),

    0x00u8 => OpCode::new(0x00, "BRK", Cpu::brk, AddressingMode::NoneAddressing, 1, 7),
};
