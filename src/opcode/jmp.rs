use crate::cpu::{AddressingMode, Cpu};

/// Sets the program counter to the address specified by a value in memory.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// # use fete::{bus::Bus, rom::Rom, testing::test_rom};
/// use fete::cpu::{Cpu, Status};
///
/// # let rom = test_rom();
/// # let bus = Bus::new(Rom::new(&rom).unwrap());
/// let mut cpu = Cpu::new(bus);
///
/// // JMP $0604
/// // BRK
/// // LDA #$05
/// // BRK
/// cpu.load_and_run(&[0x4C, 0x04, 0x06, 0x00, 0xA9, 0x05, 0x00])
///     .unwrap();
///
/// assert_eq!(cpu.reg_a, 0x05);
/// assert_eq!(cpu.pc, 0x0608);
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
/// # use fete::{bus::Bus, rom::Rom, testing::test_rom};
/// use fete::cpu::{Cpu, Status};
///
/// # let rom = test_rom();
/// # let bus = Bus::new(Rom::new(&rom).unwrap());
/// let mut cpu = Cpu::new(bus);
///
/// // JSR $0602
/// // BRK
/// // LDA #$05
/// // BRK
/// cpu.load_and_run(&[0x20, 0x02, 0x06, 0x00, 0xA9, 0x05, 0x00])
///     .unwrap();
///
/// assert_eq!(cpu.reg_a, 0x05);
/// assert_eq!(cpu.pc, 0x0608);
/// assert_eq!(cpu.pop_u16(), 0x0603);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn jsr(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    cpu.push_u16(cpu.pc);
    cpu.pc = addr;
}

/// Returns from a subroutine, and sets the program counter to the address stored on the stack.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// # use fete::{bus::Bus, rom::Rom, testing::test_rom};
/// use fete::cpu::{Cpu, Status};
///
/// # let rom = test_rom();
/// # let bus = Bus::new(Rom::new(&rom).unwrap());
/// let mut cpu = Cpu::new(bus);
///
/// // LDA #$05
/// // JSR $0608
/// // LDA #$01
/// // BRK
/// // SEI
/// // RTS
/// cpu.load_and_run(&[0xA9, 0x05, 0x20, 0x08, 0x06, 0xA9, 0x01, 0x00, 0x78, 0x60])
///     .unwrap();
///
/// assert_eq!(cpu.reg_a, 0x01);
/// assert_eq!(cpu.pc, 0x0609);
/// assert_eq!(cpu.bus.mem_read_u16(0x01FC), 0x0605); // not using pop() b/c already pop'd by RTS
/// assert_eq!(cpu.status, Status::INTERRUPT_DISABLE | Status::BREAK);
/// ```
pub fn rts(cpu: &mut Cpu, _mode: AddressingMode) {
    let addr = cpu.pop_u16();
    cpu.pc = addr;
}
