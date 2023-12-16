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

/// Adds the value at the given address to the accumulator, and sets the zero, negative, carry, and overflow flags.
///
/// # Examples
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // ADC #$05
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x69, 0x05, 0x00]);
///
/// assert_eq!(cpu.reg_a, 0x0A);
/// assert_eq!(cpu.status, Status::empty());
/// ```
pub fn adc(_cpu: &mut Cpu, _mode: AddressingMode) {
    // TODO: impl.
}

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

/// Shifts the bits of the accumulator or the value at the given address left by one, and sets the zero, negative, and carry flags.
/// The bit that is shifted out is stored in the carry flag.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // ASL A
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x0A, 0x00]);
///
/// assert_eq!(cpu.reg_a, 0x05 << 1);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn asl(cpu: &mut Cpu, mode: AddressingMode) {
    let accum = mode == AddressingMode::NoneAddressing;
    let (addr, val) = if accum {
        (0x0000, cpu.reg_a)
    } else {
        let addr = cpu.get_op_addr(mode);
        (addr, cpu.mem_read(addr))
    };

    let new_val = val << 1;
    cpu.status.set(Status::CARRY, val & 0x80 != 0);
    if accum {
        cpu.set_reg_a(new_val);
    } else {
        cpu.mem_write(addr, new_val);
    }
}

// TODO: fix branch docs to use proper comparison instructions in examples

fn branch_if(cpu: &mut Cpu, mode: AddressingMode, cond: bool) {
    let addr = cpu.get_op_addr(mode);
    if cond {
        cpu.pc = addr;
    }
}

/// Branches to the given address if the carry flag is clear.
///
/// # Examples
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use fete::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // BCC
/// // BRK
/// cpu.load_and_run(&[0x90, 0x02, 0x00]);
///
/// assert_eq!(cpu.pc, 0x8006);
/// ```
pub fn bcc(cpu: &mut Cpu, mode: AddressingMode) {
    branch_if(cpu, mode, !cpu.status.contains(Status::CARRY));
}

/// Branches to the given address if the carry flag is set.
///
/// # Examples
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use fete::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // SEC
/// // BCS $02
/// // BRK
/// cpu.load_and_run(&[0x38, 0xB0, 0x02, 0x00]);
///
/// assert_eq!(cpu.pc, 0x8007);
/// ```
pub fn bcs(cpu: &mut Cpu, mode: AddressingMode) {
    branch_if(cpu, mode, cpu.status.contains(Status::CARRY));
}

/// Branches to the given address if equal (zero flag is set).
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // CMP #$05
/// // BEQ $02
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0xC9, 0x05, 0xF0, 0x02, 0x00]);
///
/// assert_eq!(cpu.pc, 0x800A);
/// ```
pub fn beq(cpu: &mut Cpu, mode: AddressingMode) {
    branch_if(cpu, mode, cpu.status.contains(Status::ZERO));
}

/// Branches to the given address if not equal (zero flag is clear).
///
/// # Examples
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use fete::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$01
/// // BNE $02
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x01, 0xD0, 0x02, 0x00]);
///
/// assert_eq!(cpu.pc, 0x8008);
/// ```
pub fn bne(cpu: &mut Cpu, mode: AddressingMode) {
    branch_if(cpu, mode, !cpu.status.contains(Status::ZERO));
}

/// Branches to the given address if minus (negative flag set).
///
/// # Examples
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use fete::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$80
/// // BMI $02
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x80, 0x30, 0x02, 0x00]);
///
/// assert_eq!(cpu.pc, 0x8008);
/// ```
pub fn bmi(cpu: &mut Cpu, mode: AddressingMode) {
    branch_if(cpu, mode, cpu.status.contains(Status::NEGATIVE));
}

/// Branches to the given address if positive (negative flag clear).
///
/// # Examples
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use fete::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$01
/// // BMI $02
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x01, 0x10, 0x02, 0x00]);
///
/// assert_eq!(cpu.pc, 0x8008);
/// ```
pub fn bpl(cpu: &mut Cpu, mode: AddressingMode) {
    branch_if(cpu, mode, !cpu.status.contains(Status::NEGATIVE));
}

/// Branches to the given address if the overflow flag is set.
///
/// # Examples
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use fete::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // set overflow flag
/// // BMI $02
/// // BRK
/// cpu.load_and_run(&[0x70, 0x02, 0x00]);
///
/// assert_eq!(cpu.pc, 0x8006);
/// ```
pub fn bvs(cpu: &mut Cpu, mode: AddressingMode) {
    branch_if(cpu, mode, cpu.status.contains(Status::OVERFLOW));
}

/// Branches to the given address if the overflow flag is clear.
///
/// # Examples
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use fete::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // clear overflow flag
/// // BMI $02
/// // BRK
/// cpu.load_and_run(&[0x50, 0x02, 0x00]);
///
/// assert_eq!(cpu.pc, 0x8006);
/// ```
pub fn bvc(cpu: &mut Cpu, mode: AddressingMode) {
    branch_if(cpu, mode, !cpu.status.contains(Status::OVERFLOW));
}

/// Sets the carry flag.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // SEC
/// // BRK
/// cpu.load_and_run(&[0x38, 0x00]);
///
/// assert_eq!(cpu.status, Status::CARRY | Status::BREAK);
/// ```
pub fn sec(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.status |= Status::CARRY;
}

/// Clears the carry flag.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // SEC
/// // CLC
/// // BRK
/// cpu.load_and_run(&[0x38, 0x18, 0x00]);
///
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn clc(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.status &= !Status::CARRY;
}

/// Sets the decimal mode flag.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // SED
/// // BRK
/// cpu.load_and_run(&[0xF8, 0x00]);
///
/// assert_eq!(cpu.status, Status::DECIMAL_MODE | Status::BREAK);
/// ```
pub fn sed(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.status |= Status::DECIMAL_MODE;
}

/// Clears the decimal mode flag.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // SED
/// // CLD
/// // BRK
/// cpu.load_and_run(&[0xF8, 0xD8, 0x00]);
///
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn cld(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.status &= !Status::DECIMAL_MODE;
}

/// Sets the interrupt disable flag.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // SEI
/// // BRK
/// cpu.load_and_run(&[0x78, 0x00]);
///
/// assert_eq!(cpu.status, Status::INTERRUPT_DISABLE | Status::BREAK);
/// ```
pub fn sei(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.status |= Status::INTERRUPT_DISABLE;
}

/// Clears the interrupt disable flag.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // SEI
/// // CLI
/// // BRK
/// cpu.load_and_run(&[0x78, 0x58, 0x00]);
///
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn cli(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.status &= !Status::INTERRUPT_DISABLE;
}

/// Compares the value in the accumulator with the value at the given address, and sets the zero, negative, and carry flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // CMP #$05
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0xC9, 0x05, 0x00]);
///
/// assert_eq!(cpu.status, Status::CARRY | Status::ZERO | Status::BREAK);
/// ```
pub fn cmp(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.mem_read(addr);

    cpu.status.set(Status::CARRY, cpu.reg_a >= val);
    cpu.zero_and_neg_flags(cpu.reg_a.wrapping_sub(val));
}

/// Compares the value in the X register with the value at the given address, and sets the zero, negative, and carry flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // LDX #$05
/// // CPX #$05
/// // BRK
/// cpu.load_and_run(&[0xA2, 0x05, 0xE0, 0x05, 0x00]);
///
/// assert_eq!(cpu.status, Status::CARRY | Status::ZERO | Status::BREAK);
/// ```
pub fn cpx(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.mem_read(addr);

    cpu.status.set(Status::CARRY, cpu.reg_x >= val);
    cpu.zero_and_neg_flags(cpu.reg_x.wrapping_sub(val));
}

/// Compares the value in the Y register with the value at the given address, and sets the zero, negative, and carry flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::{cpu::Status, Cpu};
///
/// let mut cpu = Cpu::new();
///
/// // LDY #$05
/// // CPY #$05
/// // BRK
/// cpu.load_and_run(&[0xA0, 0x05, 0xC0, 0x05, 0x00]);
///
/// assert_eq!(cpu.status, Status::CARRY | Status::ZERO | Status::BREAK);
/// ```
pub fn cpy(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.mem_read(addr);

    cpu.status.set(Status::CARRY, cpu.reg_y >= val);
    cpu.zero_and_neg_flags(cpu.reg_y.wrapping_sub(val));
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

macro_rules! opcodes {
    ($($code:literal => ($name:ident, $addr_mode:ident, $bytes:literal, $cycles:literal),)*) => {
        phf_map! {
            $(
                $code => OpCode::new($code, stringify!($name), $name, AddressingMode::$addr_mode, $bytes, $cycles),
            )*
        }
    };
}

pub static OPCODES: Map<u8, OpCode> = opcodes! {
    0xA9u8 => (lda, Immediate, 2, 2),
    0xA5u8 => (lda, ZeroPage, 2, 3),
    0xB5u8 => (lda, ZeroPageX, 2, 4),
    0xADu8 => (lda, Absolute, 3, 4),
    0xBDu8 => (lda, AbsoluteX, 3, 4),
    0xB9u8 => (lda, AbsoluteY, 3, 4),
    0xA1u8 => (lda, IndirectX, 2, 6),
    0xB1u8 => (lda, IndirectY, 2, 5),

    0xA2u8 => (ldx, Immediate, 2, 2),
    0xA6u8 => (ldx, ZeroPage, 2, 3),
    0xB6u8 => (ldx, ZeroPageY, 2, 4),
    0xAEu8 => (ldx, Absolute, 3, 4),
    0xBEu8 => (ldx, AbsoluteY, 3, 4),

    0xA0u8 => (ldy, Immediate, 2, 2),
    0xA4u8 => (ldy, ZeroPage, 2, 3),
    0xB4u8 => (ldy, ZeroPageX, 2, 4),
    0xACu8 => (ldy, Absolute, 3, 4),
    0xBCu8 => (ldy, AbsoluteX, 3, 4),

    0x85u8 => (sta, ZeroPage, 2, 3),
    0x95u8 => (sta, ZeroPageX, 2, 4),
    0x8Du8 => (sta, Absolute, 3, 4),
    0x9Du8 => (sta, AbsoluteX, 3, 5),
    0x99u8 => (sta, AbsoluteY, 3, 5),
    0x81u8 => (sta, IndirectX, 2, 6),
    0x91u8 => (sta, IndirectY, 2, 6),

    0x86u8 => (stx, ZeroPage, 2, 3),
    0x96u8 => (stx, ZeroPageY, 2, 4),
    0x8Eu8 => (stx, Absolute, 3, 4),

    0x84u8 => (sty, ZeroPage, 2, 3),
    0x94u8 => (sty, ZeroPageX, 2, 4),
    0x8Cu8 => (sty, Absolute, 3, 4),

    0xAAu8 => (tax, NoneAddressing, 1, 2),
    0xA8u8 => (tay, NoneAddressing, 1, 2),
    0xBAu8 => (tsx, NoneAddressing, 1, 2),
    0x8Au8 => (txa, NoneAddressing, 1, 2),
    0x9Au8 => (txs, NoneAddressing, 1, 2),
    0x98u8 => (tya, NoneAddressing, 1, 2),

    0xE8u8 => (inx, NoneAddressing, 1, 2),

    0xC8u8 => (iny, NoneAddressing, 1, 2),

    0xE6u8 => (inc, ZeroPage, 2, 5),
    0xF6u8 => (inc, ZeroPageX, 2, 6),
    0xEEu8 => (inc, Absolute, 3, 6),
    0xFEu8 => (inc, AbsoluteX, 3, 7),

    // 0x69u8 => (adc, Immediate, 2, 2),
    // 0x65u8 => (adc, ZeroPage, 2, 3),
    // 0x75u8 => (adc, ZeroPageX, 2, 4),
    // 0x6Du8 => (adc, Absolute, 3, 4),
    // 0x7Du8 => (adc, AbsoluteX, 3, 4),
    // 0x79u8 => (adc, AbsoluteY, 3, 4),
    // 0x61u8 => (adc, IndirectX, 2, 6),
    // 0x71u8 => (adc, IndirectY, 2, 5),

    0x29u8 => (and, Immediate, 2, 2),
    0x25u8 => (and, ZeroPage, 2, 3),
    0x35u8 => (and, ZeroPageX, 2, 4),
    0x2Du8 => (and, Absolute, 3, 4),
    0x3Du8 => (and, AbsoluteX, 3, 4),
    0x39u8 => (and, AbsoluteY, 3, 4),
    0x21u8 => (and, IndirectX, 2, 6),
    0x31u8 => (and, IndirectY, 2, 5),

    0x0Au8 => (asl, NoneAddressing, 1, 2),
    0x06u8 => (asl, ZeroPage, 2, 5),
    0x16u8 => (asl, ZeroPageX, 2, 6),
    0x0Eu8 => (asl, Absolute, 3, 6),
    0x1Eu8 => (asl, AbsoluteX, 3, 7),

    0x90u8 => (bcc, Relative, 2, 2),
    0xB0u8 => (bcs, Relative, 2, 2),

    0xF0u8 => (beq, Relative, 2, 2),
    0xD0u8 => (bne, Relative, 2, 2),

    0x30u8 => (bmi, Relative, 2, 2),
    0x10u8 => (bpl, Relative, 2, 2),

    0x50u8 => (bvc, Relative, 2, 2),
    0x70u8 => (bvs, Relative, 2, 2),

    0x38u8 => (sec, NoneAddressing, 1, 2),
    0x18u8 => (clc, NoneAddressing, 1, 2),

    0xF8u8 => (sed, NoneAddressing, 1, 2),
    0xD8u8 => (cld, NoneAddressing, 1, 2),

    0x78u8 => (sei, NoneAddressing, 1, 2),
    0x58u8 => (cli, NoneAddressing, 1, 2),

    0xC9u8 => (cmp, Immediate, 2, 2),
    0xC5u8 => (cmp, ZeroPage, 2, 3),
    0xD5u8 => (cmp, ZeroPageX, 2, 4),
    0xCDu8 => (cmp, Absolute, 3, 4),
    0xDDu8 => (cmp, AbsoluteX, 3, 4),
    0xD9u8 => (cmp, AbsoluteY, 3, 4),
    0xC1u8 => (cmp, IndirectX, 2, 6),
    0xD1u8 => (cmp, IndirectY, 2, 5),

    0xE0u8 => (cpx, Immediate, 2, 2),
    0xE4u8 => (cpx, ZeroPage, 2, 3),
    0xECu8 => (cpx, Absolute, 3, 4),

    0xC0u8 => (cpy, Immediate, 2, 2),
    0xC4u8 => (cpy, ZeroPage, 2, 3),
    0xCCu8 => (cpy, Absolute, 3, 4),

    0x00u8 => (brk, NoneAddressing, 1, 7),
};
