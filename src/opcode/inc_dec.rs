use crate::cpu::{AddressingMode, Cpu};

/// Increments the X register, and sets the zero and negative flags.
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
/// // LDX #$05
/// // INX
/// // BRK
/// cpu.load_and_run(&[0xA2, 0x05, 0xE8, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_x, 0x06);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn inx(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.reg_x = cpu.reg_x.wrapping_add(1);
    cpu.zero_and_neg_flags(cpu.reg_x);
}

/// Increments the Y register, and sets the zero and negative flags.
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
/// // LDY #$05
/// // INY
/// // BRK
/// cpu.load_and_run(&[0xA0, 0x05, 0xC8, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_y, 0x06);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn iny(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.reg_y = cpu.reg_y.wrapping_add(1);
    cpu.zero_and_neg_flags(cpu.reg_y);
}

/// Increments a value in memory, and sets the zero and negative flags.
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
/// // STA $00FF
/// // INC $00FF
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x8D, 0xFF, 0x00, 0xEE, 0xFF, 0x00, 0x00])
///     .unwrap();
///
/// assert_eq!(cpu.bus.mem_read(0x00FF), 0x06);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn inc(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.bus.mem_read(addr).wrapping_add(1);

    cpu.bus.mem_write(addr, val);
    cpu.zero_and_neg_flags(val);
}

/// Decrements the X register, and sets the zero and negative flags.
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
/// // LDX #$05
/// // DEX
/// // BRK
/// cpu.load_and_run(&[0xA2, 0x05, 0xCA, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_x, 0x04);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn dex(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.reg_x = cpu.reg_x.wrapping_sub(1);
    cpu.zero_and_neg_flags(cpu.reg_x);
}

/// Decrements the Y register, and sets the zero and negative flags.
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
/// // LDY #$05
/// // DEY
/// // BRK
/// cpu.load_and_run(&[0xA0, 0x05, 0x88, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_y, 0x04);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn dey(cpu: &mut Cpu, _mode: AddressingMode) {
    cpu.reg_y = cpu.reg_y.wrapping_sub(1);
    cpu.zero_and_neg_flags(cpu.reg_y);
}

/// Decrements a value in memory, and sets the zero and negative flags.
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
/// // STA $00FF
/// // DEC $00FF
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x8D, 0xFF, 0x00, 0xCE, 0xFF, 0x00, 0x00])
///     .unwrap();
///
/// assert_eq!(cpu.bus.mem_read(0x00FF), 0x04);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn dec(cpu: &mut Cpu, mode: AddressingMode) {
    let addr = cpu.get_op_addr(mode);
    let val = cpu.bus.mem_read(addr).wrapping_sub(1);

    cpu.bus.mem_write(addr, val);
    cpu.zero_and_neg_flags(val);
}
