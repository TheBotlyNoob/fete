use crate::cpu::{AddressingMode, Cpu, Status};

/// Sets the carry flag.
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
/// // SEC
/// // BRK
/// cpu.load_and_run(&[0x38, 0x00]).unwrap();
///
/// assert_eq!(cpu.status, Status::CARRY | Status::BREAK);
/// ```
pub fn sec(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.status |= Status::CARRY;
}

/// Clears the carry flag.
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
/// // SEC
/// // CLC
/// // BRK
/// cpu.load_and_run(&[0x38, 0x18, 0x00]).unwrap();
///
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn clc(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.status &= !Status::CARRY;
}

/// Sets the decimal mode flag.
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
/// // SED
/// // BRK
/// cpu.load_and_run(&[0xF8, 0x00]).unwrap();
///
/// assert_eq!(cpu.status, Status::DECIMAL_MODE | Status::BREAK);
/// ```
pub fn sed(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.status |= Status::DECIMAL_MODE;
}

/// Clears the decimal mode flag.
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
/// // SED
/// // CLD
/// // BRK
/// cpu.load_and_run(&[0xF8, 0xD8, 0x00]).unwrap();
///
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn cld(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.status &= !Status::DECIMAL_MODE;
}

/// Sets the interrupt disable flag.
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
/// // SEI
/// // BRK
/// cpu.load_and_run(&[0x78, 0x00]).unwrap();
///
/// assert_eq!(cpu.status, Status::INTERRUPT_DISABLE | Status::BREAK);
/// ```
pub fn sei(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.status |= Status::INTERRUPT_DISABLE;
}

/// Clears the interrupt disable flag.
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
/// // SEI
/// // CLI
/// // BRK
/// cpu.load_and_run(&[0x78, 0x58, 0x00]).unwrap();
///
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn cli(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.status &= !Status::INTERRUPT_DISABLE;
}

/// Clears the overflow flag.
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
/// // LDA #$40
/// // STA $80
/// // LDA #$FF
/// // BIT $80 ; sets the overflow flag
/// // CLV
/// // BRK
/// cpu.load_and_run(&[
///     0xA9, 0x40, 0x8D, 0x80, 0x00, 0xA9, 0xFF, 0x24, 0x80, 0xB8, 0x00,
/// ])
/// .unwrap();
///
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn clv(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.status &= !Status::OVERFLOW;
}
