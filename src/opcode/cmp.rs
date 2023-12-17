use crate::cpu::{AddressingMode, Cpu, Status};

/// Compares the value in the accumulator with a value in memory, and sets the zero, negative, and carry flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // CMP #$05
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0xC9, 0x05, 0x00]).unwrap();
///
/// assert_eq!(cpu.status, Status::CARRY | Status::ZERO | Status::BREAK);
/// ```
pub fn cmp(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.mem_read(addr);

    cpu.status.set(Status::CARRY, cpu.reg_a >= val);
    cpu.zero_and_neg_flags(cpu.reg_a.wrapping_sub(val));
}

/// Compares the value in the X register with a value in memory, and sets the zero, negative, and carry flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDX #$05
/// // CPX #$05
/// // BRK
/// cpu.load_and_run(&[0xA2, 0x05, 0xE0, 0x05, 0x00]).unwrap();
///
/// assert_eq!(cpu.status, Status::CARRY | Status::ZERO | Status::BREAK);
/// ```
pub fn cpx(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.mem_read(addr);

    cpu.status.set(Status::CARRY, cpu.reg_x >= val);
    cpu.zero_and_neg_flags(cpu.reg_x.wrapping_sub(val));
}

/// Compares the value in the Y register with a value in memory, and sets the zero, negative, and carry flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDY #$05
/// // CPY #$05
/// // BRK
/// cpu.load_and_run(&[0xA0, 0x05, 0xC0, 0x05, 0x00]).unwrap();
///
/// assert_eq!(cpu.status, Status::CARRY | Status::ZERO | Status::BREAK);
/// ```
pub fn cpy(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.mem_read(addr);

    cpu.status.set(Status::CARRY, cpu.reg_y >= val);
    cpu.zero_and_neg_flags(cpu.reg_y.wrapping_sub(val));
}
