use crate::cpu::{AddressingMode, Cpu};

/// Sets the program counter to the address specified by a value in memory.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // JMP $C000
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x4C, 0x00, 0xC0, 0x00])
///     .unwrap();
///
/// assert_eq!(cpu.reg_a, 0x05);
/// assert_eq!(cpu.pc, 0xC002);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn jmp(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    cpu.pc = addr;
}

/// Sets the program counter to the address specified by a value in memory, and stores the old program counter on the stack.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // JSR $C000
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x20, 0x00, 0xC0, 0x00])
///     .unwrap();
///
/// assert_eq!(cpu.reg_a, 0x05);
/// assert_eq!(cpu.pc, 0xC002);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn jsr(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let pc = cpu.pc - 1;
    cpu.push_u16(pc);
    cpu.pc = addr;
}

// /// Returns from a subroutine, and sets the program counter to the address stored on the stack.
// ///
// /// # Examples
// /// ```
// /// # use pretty_assertions::assert_eq;
// /// use fete::cpu::{Cpu, Status};
// ///
// /// let mut cpu = Cpu::new();
// ///
// /// // LDA #$05
// /// // JSR $0007
// /// // SEI
// /// // RTS
// /// // BRK
// /// cpu.load_and_run(&[0xA9, 0x05, 0x20, 0x00, 0xC0, 0x78, 0x60, 0x00])
// ///     .unwrap();
// ///
// /// assert_eq!(cpu.reg_a, 0x05);
// /// assert_eq!(cpu.pc, 0x8005);
// /// assert_eq!(cpu.mem_read(0x01FF), 0x02);
// /// assert_eq!(cpu.status, Status::BREAK);
// /// ```
// pub fn rts(cpu: &mut Cpu, _mode: AddressingMode) {
//     let addr = cpu.pop_u16();
//     cpu.pc = addr + 1;
// }
