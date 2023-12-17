use crate::cpu::{AddressingMode, Cpu};

/// Loads the given value into the accumulator, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x00]).unwrap();
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
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDX #$05
/// // BRK
/// cpu.load_and_run(&[0xA2, 0x05, 0x00]).unwrap();
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
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDY #$05
/// // BRK
/// cpu.load_and_run(&[0xA0, 0x05, 0x00]).unwrap();
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
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
/// cpu.reg_a = 0x05;
///
/// // LDA #$05
/// // STA $8000
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x8D, 0x00, 0x80, 0x00])
///     .unwrap(); // keep in mind that the 16-bit address is stored in little-endian
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
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDX #$05
/// // STX $8000
/// // BRK
/// cpu.load_and_run(&[0xA2, 0x05, 0x8E, 0x00, 0x80, 0x00])
///     .unwrap(); // keep in mind that the 16-bit address is stored in little-endian
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
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDY #$05
/// // STY $8000
/// // BRK
/// cpu.load_and_run(&[0xA0, 0x05, 0x8C, 0x00, 0x80, 0x00])
///     .unwrap(); // keep in mind that the 16-bit address is stored in little-endian
///
/// assert_eq!(cpu.mem_read(0x8000), 0x05);
/// ```
pub fn sty(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    cpu.mem_write(addr, cpu.reg_y);
}
