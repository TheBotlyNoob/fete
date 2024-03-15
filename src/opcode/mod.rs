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
    0xA9_u8 => (load::lda, Immediate, 2, 2),
    0xA5_u8 => (load::lda, ZeroPage, 2, 3),
    0xB5_u8 => (load::lda, ZeroPageX, 2, 4),
    0xAD_u8 => (load::lda, Absolute, 3, 4),
    0xBD_u8 => (load::lda, AbsoluteX, 3, 4),
    0xB9_u8 => (load::lda, AbsoluteY, 3, 4),
    0xA1_u8 => (load::lda, IndirectX, 2, 6),
    0xB1_u8 => (load::lda, IndirectY, 2, 5),

    0xA2_u8 => (load::ldx, Immediate, 2, 2),
    0xA6_u8 => (load::ldx, ZeroPage, 2, 3),
    0xB6_u8 => (load::ldx, ZeroPageY, 2, 4),
    0xAE_u8 => (load::ldx, Absolute, 3, 4),
    0xBE_u8 => (load::ldx, AbsoluteY, 3, 4),

    0xA0_u8 => (load::ldy, Immediate, 2, 2),
    0xA4_u8 => (load::ldy, ZeroPage, 2, 3),
    0xB4_u8 => (load::ldy, ZeroPageX, 2, 4),
    0xAC_u8 => (load::ldy, Absolute, 3, 4),
    0xBC_u8 => (load::ldy, AbsoluteX, 3, 4),

    0x85_u8 => (load::sta, ZeroPage, 2, 3),
    0x95_u8 => (load::sta, ZeroPageX, 2, 4),
    0x8D_u8 => (load::sta, Absolute, 3, 4),
    0x9D_u8 => (load::sta, AbsoluteX, 3, 5),
    0x99_u8 => (load::sta, AbsoluteY, 3, 5),
    0x81_u8 => (load::sta, IndirectX, 2, 6),
    0x91_u8 => (load::sta, IndirectY, 2, 6),

    0x86_u8 => (load::stx, ZeroPage, 2, 3),
    0x96_u8 => (load::stx, ZeroPageY, 2, 4),
    0x8E_u8 => (load::stx, Absolute, 3, 4),

    0x84_u8 => (load::sty, ZeroPage, 2, 3),
    0x94_u8 => (load::sty, ZeroPageX, 2, 4),
    0x8C_u8 => (load::sty, Absolute, 3, 4),


    0xAA_u8 => (transfer::tax, NoneAddressing, 1, 2),
    0xA8_u8 => (transfer::tay, NoneAddressing, 1, 2),
    0x8A_u8 => (transfer::txa, NoneAddressing, 1, 2),
    0x98_u8 => (transfer::tya, NoneAddressing, 1, 2),


    0x9A_u8 => (stack::txs, NoneAddressing, 1, 2),
    0xBA_u8 => (stack::tsx, NoneAddressing, 1, 2),

    0x48_u8 => (stack::pha, NoneAddressing, 1, 3),
    0x68_u8 => (stack::pla, NoneAddressing, 1, 3),

    0x08_u8 => (stack::php, NoneAddressing, 1, 3),
    0x28_u8 => (stack::plp, NoneAddressing, 1, 3),


    0xE8_u8 => (inc_dec::inx, NoneAddressing, 1, 2),

    0xC8_u8 => (inc_dec::iny, NoneAddressing, 1, 2),

    0xE6_u8 => (inc_dec::inc, ZeroPage, 2, 5),
    0xF6_u8 => (inc_dec::inc, ZeroPageX, 2, 6),
    0xEE_u8 => (inc_dec::inc, Absolute, 3, 6),
    0xFE_u8 => (inc_dec::inc, AbsoluteX, 3, 7),

    0xCA_u8 => (inc_dec::dex, NoneAddressing, 1, 2),

    0x88_u8 => (inc_dec::dey, NoneAddressing, 1, 2),

    0xC6_u8 => (inc_dec::dec, ZeroPage, 2, 5),
    0xD6_u8 => (inc_dec::dec, ZeroPageX, 2, 6),
    0xCE_u8 => (inc_dec::dec, Absolute, 3, 6),
    0xDE_u8 => (inc_dec::dec, AbsoluteX, 3, 7),

    0x69_u8 => (arrith::adc, Immediate, 2, 2),
    0x65_u8 => (arrith::adc, ZeroPage, 2, 3),
    0x75_u8 => (arrith::adc, ZeroPageX, 2, 4),
    0x6D_u8 => (arrith::adc, Absolute, 3, 4),
    0x7D_u8 => (arrith::adc, AbsoluteX, 3, 4),
    0x79_u8 => (arrith::adc, AbsoluteY, 3, 4),
    0x61_u8 => (arrith::adc, IndirectX, 2, 6),
    0x71_u8 => (arrith::adc, IndirectY, 2, 5),

    0xE9_u8 => (arrith::sbc, Immediate, 2, 2),
    0xE5_u8 => (arrith::sbc, ZeroPage, 2, 3),
    0xF5_u8 => (arrith::sbc, ZeroPageX, 2, 4),
    0xED_u8 => (arrith::sbc, Absolute, 3, 4),
    0xFD_u8 => (arrith::sbc, AbsoluteX, 3, 4),
    0xF9_u8 => (arrith::sbc, AbsoluteY, 3, 4),
    0xE1_u8 => (arrith::sbc, IndirectX, 2, 6),
    0xF1_u8 => (arrith::sbc, IndirectY, 2, 5),

    0xC9_u8 => (arrith::cmp, Immediate, 2, 2),
    0xC5_u8 => (arrith::cmp, ZeroPage, 2, 3),
    0xD5_u8 => (arrith::cmp, ZeroPageX, 2, 4),
    0xCD_u8 => (arrith::cmp, Absolute, 3, 4),
    0xDD_u8 => (arrith::cmp, AbsoluteX, 3, 4),
    0xD9_u8 => (arrith::cmp, AbsoluteY, 3, 4),
    0xC1_u8 => (arrith::cmp, IndirectX, 2, 6),
    0xD1_u8 => (arrith::cmp, IndirectY, 2, 5),

    0xE0_u8 => (arrith::cpx, Immediate, 2, 2),
    0xE4_u8 => (arrith::cpx, ZeroPage, 2, 3),
    0xEC_u8 => (arrith::cpx, Absolute, 3, 4),

    0xC0_u8 => (arrith::cpy, Immediate, 2, 2),
    0xC4_u8 => (arrith::cpy, ZeroPage, 2, 3),
    0xCC_u8 => (arrith::cpy, Absolute, 3, 4),


    0x29_u8 => (logic::and, Immediate, 2, 2),
    0x25_u8 => (logic::and, ZeroPage, 2, 3),
    0x35_u8 => (logic::and, ZeroPageX, 2, 4),
    0x2D_u8 => (logic::and, Absolute, 3, 4),
    0x3D_u8 => (logic::and, AbsoluteX, 3, 4),
    0x39_u8 => (logic::and, AbsoluteY, 3, 4),
    0x21_u8 => (logic::and, IndirectX, 2, 6),
    0x31_u8 => (logic::and, IndirectY, 2, 5),

    0x49_u8 => (logic::eor, Immediate, 2, 2),
    0x45_u8 => (logic::eor, ZeroPage, 2, 3),
    0x55_u8 => (logic::eor, ZeroPageX, 2, 4),
    0x4D_u8 => (logic::eor, Absolute, 3, 4),
    0x5D_u8 => (logic::eor, AbsoluteX, 3, 4),
    0x59_u8 => (logic::eor, AbsoluteY, 3, 4),
    0x41_u8 => (logic::eor, IndirectX, 2, 6),
    0x51_u8 => (logic::eor, IndirectY, 2, 5),

    0x09_u8 => (logic::ora, Immediate, 2, 2),
    0x05_u8 => (logic::ora, ZeroPage, 2, 3),
    0x15_u8 => (logic::ora, ZeroPageX, 2, 4),
    0x0D_u8 => (logic::ora, Absolute, 3, 4),
    0x1D_u8 => (logic::ora, AbsoluteX, 3, 4),
    0x19_u8 => (logic::ora, AbsoluteY, 3, 4),
    0x01_u8 => (logic::ora, IndirectX, 2, 6),
    0x11_u8 => (logic::ora, IndirectY, 2, 5),

    0x24_u8 => (logic::bit, ZeroPage, 2, 3),
    0x2C_u8 => (logic::bit, Absolute, 3, 4),


    0x0A_u8 => (shift::asl, NoneAddressing, 1, 2),
    0x06_u8 => (shift::asl, ZeroPage, 2, 5),
    0x16_u8 => (shift::asl, ZeroPageX, 2, 6),
    0x0E_u8 => (shift::asl, Absolute, 3, 6),
    0x1E_u8 => (shift::asl, AbsoluteX, 3, 7),

    0x4A_u8 => (shift::lsr, NoneAddressing, 1, 2),
    0x46_u8 => (shift::lsr, ZeroPage, 2, 5),
    0x56_u8 => (shift::lsr, ZeroPageX, 2, 6),
    0x4E_u8 => (shift::lsr, Absolute, 3, 6),
    0x5E_u8 => (shift::lsr, AbsoluteX, 3, 7),

    0x2A_u8 => (shift::rol, NoneAddressing, 1, 2),
    0x26_u8 => (shift::rol, ZeroPage, 2, 5),
    0x36_u8 => (shift::rol, ZeroPageX, 2, 6),
    0x2E_u8 => (shift::rol, Absolute, 3, 6),
    0x3E_u8 => (shift::rol, AbsoluteX, 3, 7),

    0x6A_u8 => (shift::ror, NoneAddressing, 1, 2),
    0x66_u8 => (shift::ror, ZeroPage, 2, 5),
    0x76_u8 => (shift::ror, ZeroPageX, 2, 6),
    0x6E_u8 => (shift::ror, Absolute, 3, 6),
    0x7E_u8 => (shift::ror, AbsoluteX, 3, 7),


    0x90_u8 => (branch::bcc, Relative, 2, 2),
    0xB0_u8 => (branch::bcs, Relative, 2, 2),

    0xF0_u8 => (branch::beq, Relative, 2, 2),
    0xD0_u8 => (branch::bne, Relative, 2, 2),

    0x30_u8 => (branch::bmi, Relative, 2, 2),
    0x10_u8 => (branch::bpl, Relative, 2, 2),

    0x50_u8 => (branch::bvc, Relative, 2, 2),
    0x70_u8 => (branch::bvs, Relative, 2, 2),


    0x4C_u8 => (jmp::jmp, Absolute, 1, 3),
    0x6C_u8 => (jmp::jmp, Indirect, 1, 5),

    0x20_u8 => (jmp::jsr, Absolute, 3, 6),

    0x60_u8 => (jmp::rts, NoneAddressing, 1, 6),

    0x38_u8 => (flags::sec, NoneAddressing, 1, 2),
    0x18_u8 => (flags::clc, NoneAddressing, 1, 2),

    0xF8_u8 => (flags::sed, NoneAddressing, 1, 2),
    0xD8_u8 => (flags::cld, NoneAddressing, 1, 2),

    0x78_u8 => (flags::sei, NoneAddressing, 1, 2),
    0x58_u8 => (flags::cli, NoneAddressing, 1, 2),

    0xB8_u8 => (flags::clv, NoneAddressing, 1, 2),


    0x40_u8 => (sys::rti, NoneAddressing, 1, 6),
    0xEA_u8 => (sys::nop, NoneAddressing, 1, 2),
    0x00_u8 => (sys::brk, NoneAddressing, 1, 7),
};
