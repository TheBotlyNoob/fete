use crate::cpu::{AddressingMode, Cpu, Status};

/// Shifts the bits of the accumulator or a value in memory left by one, and sets the zero, negative, and carry flags.
/// The bit that is shifted out is stored in the carry flag.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::{Cpu, Status};
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$05
/// // ASL A
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x05, 0x0A, 0x00]);
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
        (addr, cpu.mem_read(addr))
    };

    let new_val = val << 1;
    cpu.status.set(Status::CARRY, val & 0x80 != 0);
    if accum {
        cpu.set_reg_a(new_val);
    } else {
        cpu.mem_write(addr, new_val);
    }
}
