use crate::cpu::{AddressingMode, Cpu};

/// Increments the X register, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDX #$05
/// // INX
/// // BRK
/// cpu.load_and_run(&[0xA2, 0x05, 0xE8, 0x00]).unwrap();
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
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDY #$05
/// // INY
/// // BRK
/// cpu.load_and_run(&[0xA0, 0x05, 0xC8, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_y, 0x06);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn iny(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.reg_y += 1;
    cpu.zero_and_neg_flags(cpu.reg_y);
}

/// Increments a value in memory, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // STA $8000
/// // INC $8000
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x8D, 0x00, 0x80, 0xEE, 0x00, 0x80, 0x00])
///     .unwrap();
///
/// assert_eq!(cpu.mem_read(0x8000), 0x06);
/// ```
pub fn inc(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.mem_read(addr).wrapping_add(1);

    cpu.mem_write(addr, val);
    cpu.zero_and_neg_flags(val);
}
