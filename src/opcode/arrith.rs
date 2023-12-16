use crate::cpu::{AddressingMode, Cpu};

/// Adds a value in memory to the accumulator, and sets the zero, negative, carry, and overflow flags.
///
/// # Examples
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // ADC #$05
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x69, 0x05, 0x00]);
///
/// assert_eq!(cpu.reg_a, 0x0A);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn adc(_cpu: &mut Cpu, _mode: AddressingMode) {
    // TODO: impl.
}

/// Subtracts a value in memory to the accumulator, and sets the zero, negative, carry, and overflow flags.
///
/// # Examples
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // ADC #$05
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x69, 0x05, 0x00]);
///
/// assert_eq!(cpu.reg_a, 0x00);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn sbc(_cpu: &mut Cpu, _mode: AddressingMode) {
    // TODO: impl.
}
