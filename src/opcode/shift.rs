use crate::cpu::{AddressingMode, Cpu, Status};

/// Shifts the bits of the accumulator or a value in memory left by one, and sets the zero, negative, and carry flags.
/// The bit that is shifted out is stored in the carry flag.
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
/// // ASL A
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x0A, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_a, 0x05 << 1);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn asl(cpu: &mut Cpu, mode: AddressingMode) {
    let accum = mode == AddressingMode::NoneAddressing;
    let (addr, val) = if accum {
        (0x0000, cpu.reg_a)
    } else {
        let addr = cpu.get_op_addr(mode);
        (addr, cpu.bus.mem_read(addr))
    };

    let new_val = val << 1;
    cpu.status.set(Status::CARRY, val & (1 << 7) != 0);
    if accum {
        cpu.set_reg_a(new_val);
    } else {
        cpu.zero_and_neg_flags(new_val);
        cpu.bus.mem_write(addr, new_val);
    }
}

/// Shifts the bits of the accumulator or a value in memory right by one, and sets the zero, negative, and carry flags.
/// The bit that is shifted out is stored in the carry flag, and bit 7 is set to zero.
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
/// // LSR A
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x4A, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_a, 0x05 >> 1);
/// assert_eq!(cpu.status, Status::CARRY | Status::BREAK);
/// ```
pub fn lsr(cpu: &mut Cpu, mode: AddressingMode) {
    let accum = mode == AddressingMode::NoneAddressing;
    let (addr, val) = if accum {
        (0x0000, cpu.reg_a)
    } else {
        let addr = cpu.get_op_addr(mode);
        (addr, cpu.bus.mem_read(addr))
    };

    let new_val = val >> 1;
    cpu.status.set(Status::CARRY, val & 1 != 0);
    if accum {
        cpu.set_reg_a(new_val);
    } else {
        cpu.zero_and_neg_flags(new_val);
        cpu.bus.mem_write(addr, new_val);
    }
}

/// Shifts the bits of the accumulator or a value in memory left by one, and sets the zero, negative, and carry flags.
/// Bit 0 is filled with the current value of the carry flag whilst the old bit 7 becomes the new carry flag value.
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
/// // SEC
/// // ROL A
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x38, 0x2A, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_a, 0x05 << 1 | 1);
/// assert_eq!(cpu.status, Status::BREAK);
/// ```
pub fn rol(cpu: &mut Cpu, mode: AddressingMode) {
    let accum = mode == AddressingMode::NoneAddressing;
    let (addr, val) = if accum {
        (0x0000, cpu.reg_a)
    } else {
        let addr = cpu.get_op_addr(mode);
        (addr, cpu.bus.mem_read(addr))
    };

    let new_val = val << 1 | u8::from(cpu.status.contains(Status::CARRY));
    cpu.status.set(Status::CARRY, val & (1 << 7) != 0);
    if accum {
        cpu.set_reg_a(new_val);
    } else {
        cpu.zero_and_neg_flags(new_val);
        cpu.bus.mem_write(addr, new_val);
    }
}

/// Shifts the bits of the accumulator or a value in memory right by one, and sets the zero, negative, and carry flags.
/// Bit 7 is filled with the current value of the carry flag whilst the old bit 0 becomes the new carry flag value.
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
/// // SEC
/// // ROR A
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x38, 0x6A, 0x00]).unwrap();
///
/// assert_eq!(cpu.reg_a, 0x05 >> 1 | 1 << 7);
/// assert_eq!(cpu.status, Status::CARRY | Status::NEGATIVE | Status::BREAK);
/// ```
pub fn ror(cpu: &mut Cpu, mode: AddressingMode) {
    let accum = mode == AddressingMode::NoneAddressing;
    let (addr, val) = if accum {
        (0x0000, cpu.reg_a)
    } else {
        let addr = cpu.get_op_addr(mode);
        (addr, cpu.bus.mem_read(addr))
    };

    let new_val = val >> 1 | u8::from(cpu.status.contains(Status::CARRY)) << 7;
    cpu.status.set(Status::CARRY, val & 1 != 0);

    if accum {
        cpu.set_reg_a(new_val);
    } else {
        cpu.zero_and_neg_flags(new_val);
        cpu.bus.mem_write(addr, new_val);
    }
}
