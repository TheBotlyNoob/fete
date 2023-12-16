use crate::cpu::{AddressingMode, Cpu};

/// Transfers the value in the accumulator to the X register, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
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
/// use fete::cpu::{Cpu, Status};
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
/// use fete::cpu::{Cpu, Status};
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
/// use fete::cpu::{Cpu, Status};
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
/// use fete::cpu::{Cpu, Status};
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
/// use fete::cpu::{Cpu, Status};
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
