use crate::cpu::{AddressingMode, Cpu, Status};

/// Breaks the program, and sets the break flag.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
/// cpu.load_and_run(&[0x00]).unwrap();
///
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn brk(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.pc += 1;
    cpu.status |= Status::BREAK;
    // TODO: impl. stack + interrupts
}

/// Performs no operation.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // NOP
/// // BRK
/// cpu.load_and_run(&[0xEA, 0x00]).unwrap(); // there's really nothing to test here...
/// ```
pub fn nop(_cpu: &mut Cpu, _mode: AddressingMode) {
    // do nothing
}

/// Returns from an interrupt processing routine. Pops the value on the stack into the status register, followed by the program counter.
///
/// # Examples
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// todo!();
/// ```
pub fn rti(_cpu: &mut Cpu, _mode: AddressingMode) {
    todo!("interrupts");
}
