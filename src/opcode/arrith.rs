use crate::cpu::{AddressingMode, Cpu, Status};

/// Adds a value in memory to the accumulator, and sets the zero, negative, carry, and overflow flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // SEC
/// // ADC #$05
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x38, 0x69, 0x05, 0x00])
///     .unwrap();
///
/// assert_eq!(cpu.reg_a, 0x0B);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn adc(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.bus.mem_read(addr);

    let (init_add, first_carry) = cpu.reg_a.overflowing_add(val);
    let (sum, second_carry) =
        init_add.overflowing_add(u8::from(cpu.status.contains(Status::CARRY)));
    cpu.set_reg_a(sum);

    cpu.status.set(Status::CARRY, first_carry || second_carry);
    cpu.status.set(
        Status::OVERFLOW,
        (val ^ cpu.reg_a) & (val ^ sum) & (1 << 7) != 0,
    );
}

/// Subtracts a value in memory to the accumulator, and sets the zero, negative, carry, and overflow flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // SBC #$05
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0xE9, 0x05, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_a, 0xFF);
/// assert_eq!(
///     cpu.status,
///     Status::CARRY | Status::NEGATIVE | Status::OVERFLOW | Status::BREAK
/// );
/// ```
pub fn sbc(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.bus.mem_read(addr);

    let (init_add, first_carry) = cpu.reg_a.overflowing_sub(val);
    let (diff, second_carry) =
        init_add.overflowing_sub(u8::from(!cpu.status.contains(Status::CARRY)));
    cpu.set_reg_a(diff);

    cpu.status.set(Status::CARRY, first_carry || second_carry);
    cpu.status.set(
        Status::OVERFLOW,
        (val ^ cpu.reg_a) & (val ^ diff) & (1 << 7) != 0,
    );
}

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
    let val = cpu.bus.mem_read(addr);

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
    let val = cpu.bus.mem_read(addr);

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
    let val = cpu.bus.mem_read(addr);

    cpu.status.set(Status::CARRY, cpu.reg_y >= val);
    cpu.zero_and_neg_flags(cpu.reg_y.wrapping_sub(val));
}
