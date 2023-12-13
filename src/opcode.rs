use phf::{phf_map, Map};

use crate::cpu::{AddressingMode, Cpu, Status};

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

/// Loads the given value into the accumulator, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x00]);
///
/// assert_eq!(cpu.reg_a, 0x05);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn lda(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.mem_read(addr);

    cpu.set_reg_a(val);
}

/// Loads the given value into the X register, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // LDX #$05
/// // BRK
/// cpu.load_and_run(&[0xA2, 0x05, 0x00]);
///
/// assert_eq!(cpu.reg_x, 0x05);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn ldx(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.mem_read(addr);

    cpu.reg_x = val;
    cpu.zero_and_neg_flags(val);
}

/// Loads the given value into the Y register, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // LDY #$05
/// // BRK
/// cpu.load_and_run(&[0xA0, 0x05, 0x00]);
///
/// assert_eq!(cpu.reg_y, 0x05);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn ldy(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.mem_read(addr);

    cpu.reg_y = val;
    cpu.zero_and_neg_flags(val);
}

/// Stores the value in the accumulator into memory.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::Cpu;
///
/// let mut cpu = Cpu::new();
/// cpu.reg_a = 0x05;
///
/// // LDA #$05
/// // STA $8000
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x8D, 0x00, 0x80, 0x00]); // keep in mind that the 16-bit address is stored in little-endian
///
/// assert_eq!(cpu.mem_read(0x8000), 0x05);
/// ```
pub fn sta(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    cpu.mem_write(addr, cpu.reg_a);
}

/// Stores the value in the X register into memory.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDX #$05
/// // STX $8000
/// // BRK
/// cpu.load_and_run(&[0xA2, 0x05, 0x8E, 0x00, 0x80, 0x00]); // keep in mind that the 16-bit address is stored in little-endian
///
/// assert_eq!(cpu.mem_read(0x8000), 0x05);
/// ```
pub fn stx(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    cpu.mem_write(addr, cpu.reg_x);
}

/// Stores the value in the Y register into memory.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDY #$05
/// // STY $8000
/// // BRK
/// cpu.load_and_run(&[0xA0, 0x05, 0x8C, 0x00, 0x80, 0x00]); // keep in mind that the 16-bit address is stored in little-endian
///
/// assert_eq!(cpu.mem_read(0x8000), 0x05);
/// ```
pub fn sty(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    cpu.mem_write(addr, cpu.reg_y);
}

/// Transfers the value in the accumulator to the X register, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // TAX
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0xAA, 0x00]);
///
/// assert_eq!(cpu.reg_x, 0x05);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn tax(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.reg_x = cpu.reg_a;
    cpu.zero_and_neg_flags(cpu.reg_x);
}

/// Transfers the value in the accumulator to the Y register, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // TAY
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0xA8, 0x00]);
///
/// assert_eq!(cpu.reg_y, 0x05);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn tay(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.reg_y = cpu.reg_a;
    cpu.zero_and_neg_flags(cpu.reg_y);
}

/// Transfers the value in the stack pointer to the X register, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // LDX #$05
/// // TXS
/// // TSX
/// // BRK
/// cpu.load_and_run(&[0xA2, 0x05, 0x9A, 0xBA, 0x00]);
///
/// assert_eq!(cpu.reg_x, 0x05);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn tsx(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.reg_x = cpu.sp;
    cpu.zero_and_neg_flags(cpu.reg_x);
}

/// Transfers the value in the X register to the accumulator, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // LDX #$05
/// // TXA
/// // BRK
/// cpu.load_and_run(&[0xA2, 0x05, 0x8A, 0x00]);
///
/// assert_eq!(cpu.reg_a, 0x05);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn txa(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.set_reg_a(cpu.reg_x);
}

/// Transfers the value in the X register to the stack pointer.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // LDX #$05
/// // TXS
/// // BRK
/// cpu.load_and_run(&[0xA2, 0x05, 0x9A, 0x00]);
///
/// assert_eq!(cpu.sp, 0x05);
/// ```
pub fn txs(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.sp = cpu.reg_x;
}

/// Transfers the value in the Y register to the accumulator, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // LDY #$05
/// // TYA
/// // BRK
/// cpu.load_and_run(&[0xA0, 0x05, 0x98, 0x00]);
///
/// assert_eq!(cpu.reg_a, 0x05);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn tya(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.set_reg_a(cpu.reg_y);
}

/// Increments the X register, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // LDX #$05
/// // INX
/// // BRK
/// cpu.load_and_run(&[0xA2, 0x05, 0xE8, 0x00]);
///
/// assert_eq!(cpu.reg_x, 0x06);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn inx(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.reg_x += 1;
    cpu.zero_and_neg_flags(cpu.reg_x);
}

/// Increments the Y register, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // LDY #$05
/// // INY
/// // BRK
/// cpu.load_and_run(&[0xA0, 0x05, 0xC8, 0x00]);
///
/// assert_eq!(cpu.reg_y, 0x06);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn iny(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.reg_y += 1;
    cpu.zero_and_neg_flags(cpu.reg_y);
}

/// Increments the value at the given address, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // STA $8000
/// // INC $8000
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x8D, 0x00, 0x80, 0xEE, 0x00, 0x80, 0x00]);
///
/// assert_eq!(cpu.mem_read(0x8000), 0x06);
/// ```
pub fn inc(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.mem_read(addr).wrapping_add(1);

    cpu.mem_write(addr, val);
    cpu.zero_and_neg_flags(val);
}

// /// Adds the value at the given address to the accumulator, and sets the zero, negative, carry, and overflow flags.
// ///
// /// # Examples
// /// ```
// /// # use pretty_assertions::assert_eq;
// /// use fete::{cpu::Status, Cpu};
// ///
// /// let mut cpu = Cpu::new();
// ///
// /// // LDA #$05
// /// // ADC #$05
// /// // BRK
// /// cpu.load_and_run(&[0xA9, 0x05, 0x69, 0x05, 0x00]);
// ///
// /// assert_eq!(cpu.reg_a, 0x0A);
// /// assert_eq!(cpu.status, Status::empty());
// /// ```
// pub fn adc(cpu: &mut Cpu, mode: AddressingMode) {
//     TODO: impl.
// }

/// Performs a bitwise AND on the accumulator and the value at the given address, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // STA $8000
/// // AND $8000
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x8D, 0x00, 0x80, 0x2D, 0x00, 0x80, 0x00]);
///
/// assert_eq!(cpu.reg_a, 0x05);
/// ```
pub fn and(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.mem_read(addr);

    cpu.set_reg_a(cpu.reg_a & val);
}

/// Breaks the program, and sets the break flag.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
/// cpu.load_and_run(&[0x00]);
///
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn brk(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.pc += 1;
    cpu.status |= Status::BREAK;
    // TODO: impl. stack + interrupts
}

pub static OPCODES: Map<u8, OpCode> = phf_map! {
    0xA9u8 => OpCode::new(0xA9, "LDA", lda, AddressingMode::Immediate, 2, 2),
    0xA5u8 => OpCode::new(0xA5, "LDA", lda, AddressingMode::ZeroPage, 2, 3),
    0xB5u8 => OpCode::new(0xB5, "LDA", lda, AddressingMode::ZeroPageX, 2, 4),
    0xADu8 => OpCode::new(0xAD, "LDA", lda, AddressingMode::Absolute, 3, 4),
    0xBDu8 => OpCode::new(0xBD, "LDA", lda, AddressingMode::AbsoluteX, 3, 4),
    0xB9u8 => OpCode::new(0xB9, "LDA", lda, AddressingMode::AbsoluteY, 3, 4),
    0xA1u8 => OpCode::new(0xA1, "LDA", lda, AddressingMode::IndirectX, 2, 6),
    0xB1u8 => OpCode::new(0xB1, "LDA", lda, AddressingMode::IndirectY, 2, 5),

    0xA2u8 => OpCode::new(0xA2, "LDX", ldx, AddressingMode::Immediate, 2, 2),
    0xA6u8 => OpCode::new(0xA6, "LDX", ldx, AddressingMode::ZeroPage, 2, 3),
    0xB6u8 => OpCode::new(0xB6, "LDX", ldx, AddressingMode::ZeroPageY, 2, 4),
    0xAEu8 => OpCode::new(0xAE, "LDX", ldx, AddressingMode::Absolute, 3, 4),
    0xBEu8 => OpCode::new(0xBE, "LDX", ldx, AddressingMode::AbsoluteY, 3, 4),

    0xA0u8 => OpCode::new(0xA0, "LDY", ldy, AddressingMode::Immediate, 2, 2),
    0xA4u8 => OpCode::new(0xA4, "LDY", ldy, AddressingMode::ZeroPage, 2, 3),
    0xB4u8 => OpCode::new(0xB4, "LDY", ldy, AddressingMode::ZeroPageX, 2, 4),
    0xACu8 => OpCode::new(0xAC, "LDY", ldy, AddressingMode::Absolute, 3, 4),
    0xBCu8 => OpCode::new(0xBC, "LDY", ldy, AddressingMode::AbsoluteX, 3, 4),

    0x85u8 => OpCode::new(0x85, "STA", sta, AddressingMode::ZeroPage, 2, 3),
    0x95u8 => OpCode::new(0x95, "STA", sta, AddressingMode::ZeroPageX, 2, 4),
    0x8Du8 => OpCode::new(0x8D, "STA", sta, AddressingMode::Absolute, 3, 4),
    0x9Du8 => OpCode::new(0x9D, "STA", sta, AddressingMode::AbsoluteX, 3, 5),
    0x99u8 => OpCode::new(0x99, "STA", sta, AddressingMode::AbsoluteY, 3, 5),
    0x81u8 => OpCode::new(0x81, "STA", sta, AddressingMode::IndirectX, 2, 6),
    0x91u8 => OpCode::new(0x91, "STA", sta, AddressingMode::IndirectY, 2, 6),

    0x86u8 => OpCode::new(0x86, "STX", stx, AddressingMode::ZeroPage, 2, 3),
    0x96u8 => OpCode::new(0x96, "STX", stx, AddressingMode::ZeroPageY, 2, 4),
    0x8Eu8 => OpCode::new(0x8E, "STX", stx, AddressingMode::Absolute, 3, 4),

    0x84u8 => OpCode::new(0x84, "STY", sty, AddressingMode::ZeroPage, 2, 3),
    0x94u8 => OpCode::new(0x94, "STY", sty, AddressingMode::ZeroPageX, 2, 4),
    0x8Cu8 => OpCode::new(0x8C, "STY", sty, AddressingMode::Absolute, 3, 4),

    0xAAu8 => OpCode::new(0xAA, "TAX", tax, AddressingMode::NoneAddressing, 1, 2),

    0xA8u8 => OpCode::new(0xA8, "TAY", tay, AddressingMode::NoneAddressing, 1, 2),

    0xBAu8 => OpCode::new(0xBA, "TSX", tsx, AddressingMode::NoneAddressing, 1, 2),

    0x8Au8 => OpCode::new(0x8A, "TXA", txa, AddressingMode::NoneAddressing, 1, 2),

    0x9Au8 => OpCode::new(0x9A, "TXS", txs, AddressingMode::NoneAddressing, 1, 2),

    0x98u8 => OpCode::new(0x98, "TYA", tya, AddressingMode::NoneAddressing, 1, 2),

    0xE8u8 => OpCode::new(0xE8, "INX", inx, AddressingMode::NoneAddressing, 1, 2),

    0xC8u8 => OpCode::new(0xC8, "INY", iny, AddressingMode::NoneAddressing, 1, 2),

    0xE6u8 => OpCode::new(0xE6, "INC", inc, AddressingMode::ZeroPage, 2, 5),
    0xF6u8 => OpCode::new(0xF6, "INC", inc, AddressingMode::ZeroPageX, 2, 6),
    0xEEu8 => OpCode::new(0xEE, "INC", inc, AddressingMode::Absolute, 3, 6),
    0xFEu8 => OpCode::new(0xFE, "INC", inc, AddressingMode::AbsoluteX, 3, 7),

    // 0x69u8 => OpCode::new(0x69, "ADC", adc, AddressingMode::Immediate, 2, 2),
    // 0x65u8 => OpCode::new(0x65, "ADC", adc, AddressingMode::ZeroPage, 2, 3),
    // 0x75u8 => OpCode::new(0x75, "ADC", adc, AddressingMode::ZeroPageX, 2, 4),
    // 0x6Du8 => OpCode::new(0x6D, "ADC", adc, AddressingMode::Absolute, 3, 4),
    // 0x7Du8 => OpCode::new(0x7D, "ADC", adc, AddressingMode::AbsoluteX, 3, 4),
    // 0x79u8 => OpCode::new(0x79, "ADC", adc, AddressingMode::AbsoluteY, 3, 4),
    // 0x61u8 => OpCode::new(0x61, "ADC", adc, AddressingMode::IndirectX, 2, 6),
    // 0x71u8 => OpCode::new(0x71, "ADC", adc, AddressingMode::IndirectY, 2, 5),

    0x29u8 => OpCode::new(0x29, "AND", and, AddressingMode::Immediate, 2, 2),
    0x25u8 => OpCode::new(0x25, "AND", and, AddressingMode::ZeroPage, 2, 3),
    0x35u8 => OpCode::new(0x35, "AND", and, AddressingMode::ZeroPageX, 2, 4),
    0x2Du8 => OpCode::new(0x2D, "AND", and, AddressingMode::Absolute, 3, 4),
    0x3Du8 => OpCode::new(0x3D, "AND", and, AddressingMode::AbsoluteX, 3, 4),
    0x39u8 => OpCode::new(0x39, "AND", and, AddressingMode::AbsoluteY, 3, 4),
    0x21u8 => OpCode::new(0x21, "AND", and, AddressingMode::IndirectX, 2, 6),
    0x31u8 => OpCode::new(0x31, "AND", and, AddressingMode::IndirectY, 2, 5),

    0x00u8 => OpCode::new(0x00, "BRK", brk, AddressingMode::NoneAddressing, 1, 7),
};
