use crate::cpu::{AddressingMode, Cpu, Status};

/// Performs a bitwise AND on the accumulator and a value in memory, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // AND #$05
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x29, 0x05, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_a, 0x05);
/// ```
pub fn and(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.mem_read(addr);

    cpu.set_reg_a(cpu.reg_a & val);
}

/// Performs a bitwise EOR on the accumulator and a value in memory, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // EOR #$05
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x49, 0x05, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_a, 0x00);
/// ```
pub fn eor(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.mem_read(addr);

    cpu.set_reg_a(cpu.reg_a ^ val);
}

/// Performs a bitwise OR on the accumulator and a value in memory, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // ORA #$0A
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x09, 0x0A, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_a, 0x0F);
/// ```
pub fn ora(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.mem_read(addr);

    cpu.set_reg_a(cpu.reg_a | val);
}

/// Tests bits in the accumulator with a value in memory, and sets the zero, negative, and overflow flags.
/// Bits 7 and 6 of the value in memory are copied into the negative and overflow flags, respectively.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$C0
/// // STA $80
/// // LDA #$3F
/// // BIT $80
/// // BRK
/// cpu.load_and_run(&[0xA9, 0xC0, 0x85, 0x80, 0xA9, 0xFF, 0x24, 0x80, 0x00])
///     .unwrap();
///
/// assert_eq!(
///     cpu.status,
///     Status::NEGATIVE | Status::OVERFLOW | Status::BREAK
/// );
/// ```
pub fn bit(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.mem_read(addr);

    cpu.status.set(Status::ZERO, cpu.reg_a & val == 0);
    cpu.status.set(Status::NEGATIVE, val & (1 << 7) != 0);
    cpu.status.set(Status::OVERFLOW, val & (1 << 6) != 0);
}
