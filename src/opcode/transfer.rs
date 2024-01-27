use crate::cpu::{AddressingMode, Cpu};

/// Transfers the value in the accumulator to the X register, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// # use fete::{bus::Bus, rom::{Rom, common_test::test_rom}};
/// use fete::cpu::{Cpu, Status};
///
/// # let rom = test_rom();
/// # let bus = Bus::new(Rom::new(&rom).unwrap());
/// let mut cpu = Cpu::new(bus);
///
/// // LDA #$05
/// // TAX
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0xAA, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_x, 0x05);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn tax(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.reg_x = cpu.reg_a;
    cpu.zero_and_neg_flags(cpu.reg_x);
}

/// Transfers the value in the accumulator to the Y register, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// # use fete::{bus::Bus, rom::{Rom, common_test::test_rom}};
/// use fete::cpu::{Cpu, Status};
///
/// # let rom = test_rom();
/// # let bus = Bus::new(Rom::new(&rom).unwrap());
/// let mut cpu = Cpu::new(bus);
///
/// // LDA #$05
/// // TAY
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0xA8, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_y, 0x05);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn tay(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.reg_y = cpu.reg_a;
    cpu.zero_and_neg_flags(cpu.reg_y);
}

/// Transfers the value in the X register to the accumulator, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// # use fete::{bus::Bus, rom::{Rom, common_test::test_rom}};
/// use fete::cpu::{Cpu, Status};
///
/// # let rom = test_rom();
/// # let bus = Bus::new(Rom::new(&rom).unwrap());
/// let mut cpu = Cpu::new(bus);
///
/// // LDX #$05
/// // TXA
/// // BRK
/// cpu.load_and_run(&[0xA2, 0x05, 0x8A, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_a, 0x05);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn txa(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.set_reg_a(cpu.reg_x);
}

/// Transfers the value in the Y register to the accumulator, and sets the zero and negative flags.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// # use fete::{bus::Bus, rom::{Rom, common_test::test_rom}};
/// use fete::cpu::{Cpu, Status};
///
/// # let rom = test_rom();
/// # let bus = Bus::new(Rom::new(&rom).unwrap());
/// let mut cpu = Cpu::new(bus);
///
/// // LDY #$05
/// // TYA
/// // BRK
/// cpu.load_and_run(&[0xA0, 0x05, 0x98, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_a, 0x05);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn tya(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.set_reg_a(cpu.reg_y);
}
