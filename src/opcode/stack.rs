use crate::cpu::{AddressingMode, Cpu, Status};

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
/// cpu.load_and_run(&[0xA2, 0x05, 0x9A, 0xBA, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_x, 0x05);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn tsx(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.reg_x = cpu.sp;
    cpu.zero_and_neg_flags(cpu.reg_x);
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
/// cpu.load_and_run(&[0xA2, 0x05, 0x9A, 0x00]).unwrap();
///
/// assert_eq!(cpu.sp, 0x05);
/// ```
pub fn txs(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.sp = cpu.reg_x;
}

/// Pushes the value in the accumulator onto the stack.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // PHA
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x48, 0x00]).unwrap();
///
/// assert_eq!(cpu.mem_read(0x01FF), 0x05);
/// assert_eq!(cpu.sp, 0xFE);
/// ```
pub fn pha(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.push(cpu.reg_a);
}

/// Pops the value on the stack into the accumulator, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // PHA
/// // LDA #$06
/// // PLA
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x48, 0xA9, 0x06, 0x68, 0x00])
///     .unwrap();
///
/// assert_eq!(cpu.reg_a, 0x05);
/// assert_eq!(cpu.sp, 0xFF);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn pla(cpu: &mut Cpu, _mode: AddressingMode) {
    let val = cpu.pop();
    cpu.set_reg_a(val);
}

/// Pushes the value in the status register onto the stack.
/// The [`Status::BREAK`] and [`Status::UNUSED`] flags will be added to the status on the stack.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // SEI
/// // PHP
/// // BRK
/// cpu.load_and_run(&[0x78, 0x08, 0x00]).unwrap();
///
/// assert_eq!(
///     cpu.mem_read(0x01FF),
///     (Status::INTERRUPT_DISABLE | Status::BREAK | Status::UNUSED).bits()
/// );
/// assert_eq!(cpu.sp, 0xFE);
/// ```
pub fn php(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.push((cpu.status | Status::BREAK | Status::UNUSED).bits());
}

/// Pops the value on the stack into the status register.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // SEI
/// // PHP
/// // CLI
/// // PLP
/// // BRK
/// cpu.load_and_run(&[0x78, 0x08, 0x58, 0x28, 0x00]).unwrap();
///
/// assert_eq!(
///     cpu.status,
///     Status::INTERRUPT_DISABLE | Status::BREAK | Status::UNUSED
/// );
/// assert_eq!(cpu.sp, 0xFF);
/// ```
pub fn plp(cpu: &mut Cpu, _mode: AddressingMode) {
    let val = cpu.pop();
    cpu.status = Status::from_bits_truncate(val);
}
