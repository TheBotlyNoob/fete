use crate::cpu::{AddressingMode, Cpu, Status};

fn branch_if(cpu: &mut Cpu, mode: AddressingMode, cond: bool) {
    let addr = cpu.get_op_addr(mode);
    if cond {
        cpu.pc = addr;
    }
}

/// Increases the program counter by the given number of bytes if the carry flag is clear.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // CLC
/// // BCC
/// // BRK
/// cpu.load_and_run(&[0x18, 0x90, 0x02, 0x00]);
///
/// assert_eq!(cpu.pc, 0x8007);
/// ```
pub fn bcc(cpu: &mut Cpu, mode: AddressingMode) {
    branch_if(cpu, mode, !cpu.status.contains(Status::CARRY));
}

/// Increases the program counter by the given number of bytes if the carry flag is set.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // SEC
/// // BCS $02
/// // BRK
/// cpu.load_and_run(&[0x38, 0xB0, 0x02, 0x00]);
///
/// assert_eq!(cpu.pc, 0x8007);
/// ```
pub fn bcs(cpu: &mut Cpu, mode: AddressingMode) {
    branch_if(cpu, mode, cpu.status.contains(Status::CARRY));
}

/// Increases the program counter by the given number of bytes if equal (zero flag is set).
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$00
/// // BEQ $02
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x00, 0xF0, 0x02, 0x00]);
///
/// assert_eq!(cpu.pc, 0x8008);
/// ```
pub fn beq(cpu: &mut Cpu, mode: AddressingMode) {
    branch_if(cpu, mode, cpu.status.contains(Status::ZERO));
}

/// Increases the program counter by the given number of bytes if not equal (zero flag is clear).
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$01
/// // BNE $02
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x01, 0xD0, 0x02, 0x00]);
///
/// assert_eq!(cpu.pc, 0x8008);
/// ```
pub fn bne(cpu: &mut Cpu, mode: AddressingMode) {
    branch_if(cpu, mode, !cpu.status.contains(Status::ZERO));
}

/// Increases the program counter by the given number of bytes if minus (negative flag set).
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$80 ; 0x80 is -128
/// // BMI $02
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x80, 0x30, 0x02, 0x00]);
///
/// assert_eq!(cpu.pc, 0x8008);
/// ```
pub fn bmi(cpu: &mut Cpu, mode: AddressingMode) {
    branch_if(cpu, mode, cpu.status.contains(Status::NEGATIVE));
}

/// Increases the program counter by the given number of bytes if positive (negative flag clear).
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // LDA #$01
/// // BMI $02
/// // BRK
/// cpu.load_and_run(&[0xA9, 0x01, 0x10, 0x02, 0x00]);
///
/// assert_eq!(cpu.pc, 0x8008);
/// ```
pub fn bpl(cpu: &mut Cpu, mode: AddressingMode) {
    branch_if(cpu, mode, !cpu.status.contains(Status::NEGATIVE));
}

/// Increases the program counter by the given number of bytes if the overflow flag is set.
///
/// # Examples
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // set overflow flag
/// // BMI $02
/// // BRK
/// cpu.load_and_run(&[0x70, 0x02, 0x00]);
///
/// assert_eq!(cpu.pc, 0x8006);
/// ```
pub fn bvs(cpu: &mut Cpu, mode: AddressingMode) {
    branch_if(cpu, mode, cpu.status.contains(Status::OVERFLOW));
}

/// Increases the program counter by the given number of bytes if the overflow flag is clear.
///
/// # Examples
/// ```
/// # use pretty_assertions::assert_eq;
/// use fete::cpu::Cpu;
///
/// let mut cpu = Cpu::new();
///
/// // CLV
/// // BMI $02
/// // BRK
/// cpu.load_and_run(&[0xB8, 0x50, 0x02, 0x00]);
///
/// assert_eq!(cpu.pc, 0x8007);
/// ```
pub fn bvc(cpu: &mut Cpu, mode: AddressingMode) {
    branch_if(cpu, mode, !cpu.status.contains(Status::OVERFLOW));
}
