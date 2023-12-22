use phf::{phf_map, Map};

use crate::cpu::{AddressingMode, Cpu};

pub mod arrith;
pub mod branch;
pub mod flags;
pub mod inc_dec;
pub mod jmp;
pub mod load;
pub mod logic;
pub mod shift;
pub mod stack;
pub mod sys;
pub mod transfer;

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
        op: fn(&mut Cpu, AddressingMode),
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

macro_rules! opcodes {
    ($($code:literal => ($path:ident::$name:ident, $addr_mode:ident, $bytes:literal, $cycles:literal),)*) => {
        phf_map! {
            $(
                $code => OpCode::new($code, stringify!($name), $path::$name, AddressingMode::$addr_mode, $bytes, $cycles),
            )*
        }
    };
}

pub static OPCODES: Map<u8, OpCode> = opcodes! {
    0xA9u8 => (load::lda, Immediate, 2, 2),
    0xA5u8 => (load::lda, ZeroPage, 2, 3),
    0xB5u8 => (load::lda, ZeroPageX, 2, 4),
    0xADu8 => (load::lda, Absolute, 3, 4),
    0xBDu8 => (load::lda, AbsoluteX, 3, 4),
    0xB9u8 => (load::lda, AbsoluteY, 3, 4),
    0xA1u8 => (load::lda, IndirectX, 2, 6),
    0xB1u8 => (load::lda, IndirectY, 2, 5),

    0xA2u8 => (load::ldx, Immediate, 2, 2),
    0xA6u8 => (load::ldx, ZeroPage, 2, 3),
    0xB6u8 => (load::ldx, ZeroPageY, 2, 4),
    0xAEu8 => (load::ldx, Absolute, 3, 4),
    0xBEu8 => (load::ldx, AbsoluteY, 3, 4),

    0xA0u8 => (load::ldy, Immediate, 2, 2),
    0xA4u8 => (load::ldy, ZeroPage, 2, 3),
    0xB4u8 => (load::ldy, ZeroPageX, 2, 4),
    0xACu8 => (load::ldy, Absolute, 3, 4),
    0xBCu8 => (load::ldy, AbsoluteX, 3, 4),

    0x85u8 => (load::sta, ZeroPage, 2, 3),
    0x95u8 => (load::sta, ZeroPageX, 2, 4),
    0x8Du8 => (load::sta, Absolute, 3, 4),
    0x9Du8 => (load::sta, AbsoluteX, 3, 5),
    0x99u8 => (load::sta, AbsoluteY, 3, 5),
    0x81u8 => (load::sta, IndirectX, 2, 6),
    0x91u8 => (load::sta, IndirectY, 2, 6),

    0x86u8 => (load::stx, ZeroPage, 2, 3),
    0x96u8 => (load::stx, ZeroPageY, 2, 4),
    0x8Eu8 => (load::stx, Absolute, 3, 4),

    0x84u8 => (load::sty, ZeroPage, 2, 3),
    0x94u8 => (load::sty, ZeroPageX, 2, 4),
    0x8Cu8 => (load::sty, Absolute, 3, 4),


    0xAAu8 => (transfer::tax, NoneAddressing, 1, 2),
    0xA8u8 => (transfer::tay, NoneAddressing, 1, 2),
    0x8Au8 => (transfer::txa, NoneAddressing, 1, 2),
    0x98u8 => (transfer::tya, NoneAddressing, 1, 2),


    0x9Au8 => (stack::txs, NoneAddressing, 1, 2),
    0xBAu8 => (stack::tsx, NoneAddressing, 1, 2),

    0x48u8 => (stack::pha, NoneAddressing, 1, 3),
    0x68u8 => (stack::pla, NoneAddressing, 1, 3),

    0x08u8 => (stack::php, NoneAddressing, 1, 3),
    0x28u8 => (stack::plp, NoneAddressing, 1, 3),


    0xE8u8 => (inc_dec::inx, NoneAddressing, 1, 2),

    0xC8u8 => (inc_dec::iny, NoneAddressing, 1, 2),

    0xE6u8 => (inc_dec::inc, ZeroPage, 2, 5),
    0xF6u8 => (inc_dec::inc, ZeroPageX, 2, 6),
    0xEEu8 => (inc_dec::inc, Absolute, 3, 6),
    0xFEu8 => (inc_dec::inc, AbsoluteX, 3, 7),


    0x69u8 => (arrith::adc, Immediate, 2, 2),
    0x65u8 => (arrith::adc, ZeroPage, 2, 3),
    0x75u8 => (arrith::adc, ZeroPageX, 2, 4),
    0x6Du8 => (arrith::adc, Absolute, 3, 4),
    0x7Du8 => (arrith::adc, AbsoluteX, 3, 4),
    0x79u8 => (arrith::adc, AbsoluteY, 3, 4),
    0x61u8 => (arrith::adc, IndirectX, 2, 6),
    0x71u8 => (arrith::adc, IndirectY, 2, 5),

    0xE9u8 => (arrith::sbc, Immediate, 2, 2),
    0xE5u8 => (arrith::sbc, ZeroPage, 2, 3),
    0xF5u8 => (arrith::sbc, ZeroPageX, 2, 4),
    0xEDu8 => (arrith::sbc, Absolute, 3, 4),
    0xFDu8 => (arrith::sbc, AbsoluteX, 3, 4),
    0xF9u8 => (arrith::sbc, AbsoluteY, 3, 4),
    0xE1u8 => (arrith::sbc, IndirectX, 2, 6),
    0xF1u8 => (arrith::sbc, IndirectY, 2, 5),

    0xC9u8 => (arrith::cmp, Immediate, 2, 2),
    0xC5u8 => (arrith::cmp, ZeroPage, 2, 3),
    0xD5u8 => (arrith::cmp, ZeroPageX, 2, 4),
    0xCDu8 => (arrith::cmp, Absolute, 3, 4),
    0xDDu8 => (arrith::cmp, AbsoluteX, 3, 4),
    0xD9u8 => (arrith::cmp, AbsoluteY, 3, 4),
    0xC1u8 => (arrith::cmp, IndirectX, 2, 6),
    0xD1u8 => (arrith::cmp, IndirectY, 2, 5),

    0xE0u8 => (arrith::cpx, Immediate, 2, 2),
    0xE4u8 => (arrith::cpx, ZeroPage, 2, 3),
    0xECu8 => (arrith::cpx, Absolute, 3, 4),

    0xC0u8 => (arrith::cpy, Immediate, 2, 2),
    0xC4u8 => (arrith::cpy, ZeroPage, 2, 3),
    0xCCu8 => (arrith::cpy, Absolute, 3, 4),


    0x29u8 => (logic::and, Immediate, 2, 2),
    0x25u8 => (logic::and, ZeroPage, 2, 3),
    0x35u8 => (logic::and, ZeroPageX, 2, 4),
    0x2Du8 => (logic::and, Absolute, 3, 4),
    0x3Du8 => (logic::and, AbsoluteX, 3, 4),
    0x39u8 => (logic::and, AbsoluteY, 3, 4),
    0x21u8 => (logic::and, IndirectX, 2, 6),
    0x31u8 => (logic::and, IndirectY, 2, 5),

    0x49u8 => (logic::eor, Immediate, 2, 2),
    0x45u8 => (logic::eor, ZeroPage, 2, 3),
    0x55u8 => (logic::eor, ZeroPageX, 2, 4),
    0x4Du8 => (logic::eor, Absolute, 3, 4),
    0x5Du8 => (logic::eor, AbsoluteX, 3, 4),
    0x59u8 => (logic::eor, AbsoluteY, 3, 4),
    0x41u8 => (logic::eor, IndirectX, 2, 6),
    0x51u8 => (logic::eor, IndirectY, 2, 5),

    0x09u8 => (logic::ora, Immediate, 2, 2),
    0x05u8 => (logic::ora, ZeroPage, 2, 3),
    0x15u8 => (logic::ora, ZeroPageX, 2, 4),
    0x0Du8 => (logic::ora, Absolute, 3, 4),
    0x1Du8 => (logic::ora, AbsoluteX, 3, 4),
    0x19u8 => (logic::ora, AbsoluteY, 3, 4),
    0x01u8 => (logic::ora, IndirectX, 2, 6),
    0x11u8 => (logic::ora, IndirectY, 2, 5),

    0x24u8 => (logic::bit, ZeroPage, 2, 3),
    0x2Cu8 => (logic::bit, Absolute, 3, 4),


    0x0Au8 => (shift::asl, NoneAddressing, 1, 2),
    0x06u8 => (shift::asl, ZeroPage, 2, 5),
    0x16u8 => (shift::asl, ZeroPageX, 2, 6),
    0x0Eu8 => (shift::asl, Absolute, 3, 6),
    0x1Eu8 => (shift::asl, AbsoluteX, 3, 7),

    0x4Au8 => (shift::lsr, NoneAddressing, 1, 2),
    0x46u8 => (shift::lsr, ZeroPage, 2, 5),
    0x56u8 => (shift::lsr, ZeroPageX, 2, 6),
    0x4Eu8 => (shift::lsr, Absolute, 3, 6),
    0x5Eu8 => (shift::lsr, AbsoluteX, 3, 7),

    0x2Au8 => (shift::rol, NoneAddressing, 1, 2),
    0x26u8 => (shift::rol, ZeroPage, 2, 5),
    0x36u8 => (shift::rol, ZeroPageX, 2, 6),
    0x2Eu8 => (shift::rol, Absolute, 3, 6),
    0x3Eu8 => (shift::rol, AbsoluteX, 3, 7),

    0x6Au8 => (shift::ror, NoneAddressing, 1, 2),
    0x66u8 => (shift::ror, ZeroPage, 2, 5),
    0x76u8 => (shift::ror, ZeroPageX, 2, 6),
    0x6Eu8 => (shift::ror, Absolute, 3, 6),
    0x7Eu8 => (shift::ror, AbsoluteX, 3, 7),


    0x90u8 => (branch::bcc, Relative, 2, 2),
    0xB0u8 => (branch::bcs, Relative, 2, 2),

    0xF0u8 => (branch::beq, Relative, 2, 2),
    0xD0u8 => (branch::bne, Relative, 2, 2),

    0x30u8 => (branch::bmi, Relative, 2, 2),
    0x10u8 => (branch::bpl, Relative, 2, 2),

    0x50u8 => (branch::bvc, Relative, 2, 2),
    0x70u8 => (branch::bvs, Relative, 2, 2),


    0x4Cu8 => (jmp::jmp, Absolute, 1, 3),
    0x6Cu8 => (jmp::jmp, Indirect, 1, 5),

    0x20u8 => (jmp::jsr, Absolute, 3, 6),

    0x60u8 => (jmp::rts, NoneAddressing, 1, 6),

    0x38u8 => (flags::sec, NoneAddressing, 1, 2),
    0x18u8 => (flags::clc, NoneAddressing, 1, 2),

    0xF8u8 => (flags::sed, NoneAddressing, 1, 2),
    0xD8u8 => (flags::cld, NoneAddressing, 1, 2),

    0x78u8 => (flags::sei, NoneAddressing, 1, 2),
    0x58u8 => (flags::cli, NoneAddressing, 1, 2),

    0xB8u8 => (flags::clv, NoneAddressing, 1, 2),


    0x40u8 => (sys::rti, NoneAddressing, 1, 6),
    0xEAu8 => (sys::nop, NoneAddressing, 1, 2),
    0x00u8 => (sys::brk, NoneAddressing, 1, 7),
};
