use crate::cpu::{AddressingMode, Cpu, Status};

/// Sets the carry flag.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
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
/// use fete::cpu::{Cpu, Status};
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
/// use fete::cpu::{Cpu, Status};
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
/// use fete::cpu::{Cpu, Status};
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
/// use fete::cpu::{Cpu, Status};
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
/// use fete::cpu::{Cpu, Status};
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

/// Clears the overflow flag.
///
/// # Examples
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // TODO: set overflow flag
/// // CLV
/// // BRK
/// cpu.load_and_run(&[0xB8, 0x00]);
///
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn clv(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.status &= !Status::OVERFLOW;
}
