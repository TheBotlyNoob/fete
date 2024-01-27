use bitflags::bitflags;
use snafu::prelude::*;

use crate::bus::Bus;

pub const STACK: u16 = 0x0100;
pub const STACK_RESET: u8 = 0xFD;

#[derive(Snafu)]
pub enum Error {
    #[snafu(display("invalid opcode: {:#02x}", opcode))]
    InvalidOpcode { opcode: u8, offset: u16 },
}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        struct DisplayAsDebug<T: core::fmt::Display>(pub T);
        impl<T: core::fmt::Display> core::fmt::Debug for DisplayAsDebug<T> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        match self {
            Self::InvalidOpcode { opcode, offset } => f
                .debug_struct("InvalidOpcode")
                .field("opcode", &DisplayAsDebug(format_args!("{opcode:#02x}")))
                .field("offset", &DisplayAsDebug(format_args!("{offset:#02x}")))
                .finish(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    /// Immediate addressing allows the programmer to directly specify an 8 bit constant within the instruction.
    /// It is indicated by a '#' symbol followed by an numeric expression. For example:
    /// ```x86asm
    /// LDA #10         ;Load 10 ($0A) into the accumulator
    /// LDX #LO LABEL   ;Load the LSB of a 16 bit address into X
    /// LDY #HI LABEL   ;Load the MSB of a 16 bit address into Y
    /// ```
    Immediate,
    /// An instruction using zero page addressing mode has only an 8 bit address operand.
    /// This limits it to addressing only the first 256 bytes of memory (e.g. $0000 to $00FF) where the most significant byte of the address is always zero.
    /// In zero page mode only the least significant byte of the address is held in the instruction making it shorter by one byte (important for space saving) and one less memory fetch during execution (important for speed).
    /// An assembler will automatically select zero page addressing mode if the operand evaluates to a zero page address and the instruction supports the mode (not all do).
    /// ```x86asm
    /// LDA $00         ;Load accumulator from $00
    /// ASL ANSWER      ;Shift labelled location ANSWER left
    /// ```
    ZeroPage,
    /// The address to be accessed by an instruction using indexed zero page addressing is calculated by taking the 8 bit zero page address from the instruction and adding the current value of the X register to it.
    /// For example if the X register contains $0F and the instruction LDA $80,X is executed then the accumulator will be loaded from $008F (e.g. $80 + $0F => $8F).
    ///
    /// NB: The address calculation wraps around if the sum of the base address and the register exceed $FF.
    /// If we repeat the last example but with $FF in the X register then the accumulator will be loaded from $007F (e.g. $80 + $FF => $7F) and not $017F.
    /// ```x86asm
    /// STY $10,X       ;Save the Y register at location on zero page
    /// AND TEMP,X      ;Logical AND accumulator with a zero page value
    /// ```
    ZeroPageX,
    /// The address to be accessed by an instruction using indexed zero page addressing is calculated by taking the 8 bit zero page address from the instruction and adding the current value of the Y register to it.
    /// This mode can only be used with the LDX and STX instructions.
    /// ```x86asm
    /// STX $10,Y       ;Save the X register at location on zero page
    /// AND TEMP,Y      ;Logical AND accumulator with a zero page value
    /// ```
    ZeroPageY,
    /// Instructions using absolute addressing contain a full 16 bit address to identify the target location.
    /// ```x86asm
    /// JMP $1234       ;Jump to location $1234
    /// JSR WIBBLE      ;Call subroutine WIBBLE
    /// ```
    Absolute,
    /// The address to be accessed by an instruction using X register indexed absolute addressing is computed by taking the 16 bit address from the instruction and added the contents of the X register.
    /// For example if X contains $92 then an STA $2000,X instruction will store the accumulator at $2092 (e.g. $2000 + $92).
    /// ```x86asm
    /// STA $3000,X     ;Store accumulator between $3000 and $30FF
    /// ROR CRC,X       ;Rotate right one bit
    /// ```
    AbsoluteX,
    /// The Y register indexed absolute addressing mode is the same as the previous mode only with the contents of the Y register added to the 16 bit address from the instruction.
    /// ```x86asm
    /// AND $4000,Y     ;Perform a logical AND with a byte of memory
    /// STA MEM,Y       ;Store accumulator in memory
    /// ```
    AbsoluteY,
    /// JMP is the only 6502 instruction to support indirection. The instruction contains a 16 bit address which identifies the location of the least significant byte of another 16 bit memory address which is the real target of the instruction.
    /// For example if location $0120 contains $FC and location $0121 contains $BA then the instruction JMP ($0120) will cause the next instruction execution to occur at $BAFC (e.g. the contents of $0120 and $0121).
    /// ```x86asm
    /// JMP ($FFFC)     ;Force a power on reset
    /// JMP (TARGET)    ;Jump via a labelled memory area
    /// ```
    Indirect,
    /// Indexed indirect addressing is normally used in conjunction with a table of address held on zero page.
    /// The address of the table is taken from the instruction and the X register added to it (with zero page wrap around) to give the location of the least significant byte of the target address.
    /// ```x86asm
    /// LDA ($40,X)     ;Load a byte indirectly from memory
    /// STA (MEM,X)     ;Store accumulator indirectly into memory
    /// ```
    IndirectX,
    /// Indirect indirect addressing is the most common indirection mode used on the 6502. In instruction contains the zero page location of the least significant byte of 16 bit address. The Y register is dynamically added to this value to generated the actual target address for operation.
    /// ```x86asm
    /// LDA ($40),Y     ;Load a byte indirectly from memory
    /// STA (DST),Y     ;Store accumulator indirectly into memory
    /// ```
    IndirectY,
    /// Relative addressing mode is used by branch instructions (e.g. BEQ, BNE, etc.) which contain a signed 8 bit relative offset (e.g. -128 to +127) which is added to program counter if the condition is true.
    /// As the program counter itself is incremented during instruction execution by two the effective address range for the target instruction must be with -126 to +129 bytes of the branch.
    /// ```x86asm
    /// BEQ LABEL       ;Branch if zero flag set to LABEL
    /// BNE *+4         ;Skip over the following 2 byte instruction
    /// ```
    Relative,
    /// For many 6502 instructions the source and destination of the information to be manipulated is implied directly by the function of the instruction itself and no further operand needs to be specified. Operations like 'Clear Carry Flag' (CLC) and 'Return from Subroutine' (RTS) are implicit.
    ///
    /// Additionally, some instructions have an option to operate directly upon the accumulator. The programmer specifies this by using a special operand value, 'A'. For example:
    /// ```x86asm
    /// LSR A           ;Logical shift right one bit
    /// ROR A           ;Rotate right one bit
    /// ```
    NoneAddressing,
}

bitflags! {
    /// Status register flags.
    ///
    /// ```ignore
    ///  7 6 5 4 3 2 1 0
    ///  N V _ B D I Z C
    ///  | |   | | | | +--- Carry Flag
    ///  | |   | | | +----- Zero Flag
    ///  | |   | | +------- Interrupt Disable
    ///  | |   | +--------- Decimal Mode (not used on NES)
    ///  | |   +----------- Break Command
    ///  | +--------------- Overflow Flag
    ///  +----------------- Negative Flag
    /// ```
    #[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
    pub struct Status: u8 {
        /// Carry flag.
        const CARRY = 1;
        /// Zero flag.
        const ZERO = 1 << 1;
        /// Interrupt disable flag.
        const INTERRUPT_DISABLE = 1 << 2;
        /// Decimal mode flag.
        const DECIMAL_MODE = 1 << 3;
        /// Break flag.
        const BREAK = 1 << 4;
        /// Unused flag.
        const UNUSED = 1 << 5;
        /// Overflow flag.
        const OVERFLOW = 1 << 6;
        /// Negative flag.
        const NEGATIVE = 1 << 7;
    }
}

pub struct Cpu<'rom> {
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    pub status: Status,
    pub sp: u8,
    pub pc: u16,
    pub bus: Bus<'rom>,
}

impl<'rom> Cpu<'rom> {
    /// Creates a new CPU with the default state.
    #[must_use]
    pub fn new(bus: Bus<'rom>) -> Self {
        Self {
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            status: Status::default(),
            sp: STACK_RESET,
            pc: bus.mem_read_u16(0xFFFC),
            bus,
        }
    }

    /// Resets the CPU to its initial state. Keeps the memory intact.
    pub fn reset(&mut self) {
        replace_with::replace_with(self, || unreachable!(), |self_| Self::new(self_.bus));
    }

    /// Gets the address at the current program count, using the given [`AddressingMode`]. Increments the program count as needed.
    pub fn get_op_addr(&mut self, mode: AddressingMode) -> u16 {
        #[allow(clippy::match_wildcard_for_single_variants)] // that's the point
        match mode {
            AddressingMode::Immediate => {
                let prev_pc = self.pc;
                self.pc = self.pc.wrapping_add(1);
                prev_pc
            }
            AddressingMode::ZeroPage => u16::from(self.take()),
            AddressingMode::ZeroPageX => u16::from(self.take().wrapping_add(self.reg_x)),
            AddressingMode::ZeroPageY => u16::from(self.take().wrapping_add(self.reg_y)),
            AddressingMode::Absolute => self.take_u16(),
            AddressingMode::AbsoluteX => self.take_u16().wrapping_add(u16::from(self.reg_x)),
            AddressingMode::AbsoluteY => self.take_u16().wrapping_add(u16::from(self.reg_y)),
            AddressingMode::Indirect => {
                let real_addr = self.take_u16();
                self.bus.mem_read_u16(real_addr)
            }
            AddressingMode::IndirectX => {
                let real_addr = u16::from(self.take());
                self.bus.mem_read_u16(real_addr) + u16::from(self.reg_x)
            }
            AddressingMode::IndirectY => {
                let real_addr = u16::from(self.take());
                self.bus.mem_read_u16(real_addr) + u16::from(self.reg_y)
            }
            AddressingMode::Relative => {
                let offset = self.take(); // self.pc + 1
                self.pc.wrapping_add(u16::from(offset))
            }
            AddressingMode::NoneAddressing => {
                unreachable!("AddressingMode::NoneAddressing is not a valid addressing mode");
            }
        }
    }

    /// Loads the given program into memory, and sets the program counter to the start of the program.
    pub fn load(&mut self, prog: &[u8]) -> Result<(), Error> {
        for (i, &b) in prog.iter().enumerate().take(usize::from(u16::MAX)) {
            self.bus.mem_write(0x0600 + i as u16, b);
        }
        self.pc = self.bus.mem_read_u16(0xFFFC);
        Ok(())
    }

    /// Loads the given program into memory, resets the CPU, and runs the program.
    ///
    /// # Errors
    /// Returns an [`Error::InvalidOpcode`] if an invalid opcode is encountered.
    pub fn load_and_run(&mut self, prog: &[u8]) -> Result<(), Error> {
        self.load(prog)?;
        self.reset();
        self.run()
    }

    /// Sets the accumulator register, and sets the zero and negative flags.
    pub fn set_reg_a(&mut self, val: u8) {
        self.reg_a = val;
        self.zero_and_neg_flags(val);
    }

    /// Sets the zero and negative flags based on the given value.
    ///
    /// # Examples
    /// ```
    /// # use fete::{bus::Bus, rom::{Rom, common_test::test_rom}};
/// # use pretty_assertions::assert_eq;
/// # use fete::{bus::Bus, rom::{Rom, common_test::test_rom}};
/// # use fete::{bus::Bus, rom::{Rom, common_test::test_rom}};
    /// use fete::cpu::{Cpu, Status};
    ///
    /// # let rom = test_rom();
/// # let bus = Bus::new(&rom);
/// let mut cpu = Cpu::new(bus);
    ///
    /// cpu.zero_and_neg_flags(0);
    /// assert_eq!(cpu.status, Status::ZERO);
    ///
    /// cpu.zero_and_neg_flags(1);
    /// assert_eq!(cpu.status, Status::empty());
    ///
    /// cpu.zero_and_neg_flags(1 << 7);
    /// assert_eq!(cpu.status, Status::NEGATIVE);
    /// ```
    pub fn zero_and_neg_flags(&mut self, val: u8) {
        if val == 0 {
            self.status |= Status::ZERO;
        } else {
            self.status &= !Status::ZERO;
        }

        if val & (1 << 7) != 0 {
            self.status |= Status::NEGATIVE;
        } else {
            self.status &= !Status::NEGATIVE;
        }
    }

    /// Runs the program currently loaded into memory.
    ///
    /// # Errors
    /// Returns an [`Error::InvalidOpcode`] if an invalid opcode is encountered.
    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            if self.tick()? {
                break Ok(());
            }
        }
    }

    /// Ticks the current cpu cycle, executing the current instruction loaded into memory.
    ///
    /// # Errors
    /// Returns an [`Error::InvalidOpcode`] if an invalid opcode is encountered.
    pub fn tick(&mut self) -> Result<bool, Error> {
        let opcode = self.take();
        let opcode_info = crate::opcode::OPCODES.get(&opcode);

        if let Some(opcode) = opcode_info {
            println!("{:#02x} {} ({:#?})", self.pc, opcode.name, opcode.mode);
            (opcode.op)(self, opcode.mode);
        } else {
            return Err(Error::InvalidOpcode {
                opcode,
                offset: self.pc.saturating_sub(1),
            });
        }

        Ok(opcode == 0x00)
    }

    /// Pushes a byte onto the stack.
    pub fn push(&mut self, val: u8) {
        self.bus
            .mem_write(STACK.saturating_add(u16::from(self.sp)), val);
        self.sp = self.sp.wrapping_sub(1);
    }

    /// Pushes a little-endian, 16-bit number onto the stack.
    pub fn push_u16(&mut self, val: u16) {
        let [lo, hi] = val.to_le_bytes();
        self.push(hi);
        self.push(lo);
    }

    /// Pops a value from the stack.
    pub fn pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.bus.mem_read(STACK.saturating_add(u16::from(self.sp)))
    }

    /// Pops a little-endian, 16-bit number from the stack.
    pub fn pop_u16(&mut self) -> u16 {
        let lo = self.pop();
        let hi = self.pop();
        u16::from_le_bytes([lo, hi])
    }

    /// Takes the next byte from memory, and increments the program counter.
    fn take(&mut self) -> u8 {
        let byte = self.bus.mem_read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }
    /// Takes the next little-endian, 16-bit number from memory, and increments the program counter.
    fn take_u16(&mut self) -> u16 {
        let num = self.bus.mem_read_u16(self.pc);
        self.pc = self.pc.wrapping_add(2);
        num
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rom::{common_test::test_rom, Rom};
    use pretty_assertions::assert_eq;
    use test_log::test;

    #[test]
    fn op_addr_immediate() {
        let rom = test_rom();
        let bus = Bus::new(Rom::new(&rom).unwrap());
        let mut cpu = Cpu::new(bus);
        cpu.bus.mem_write(0x0000, 0x05);

        assert_eq!(cpu.get_op_addr(AddressingMode::Immediate), 0x0000);
        assert_eq!(cpu.pc, 0x0001);
    }

    #[test]
    fn op_addr_zero_page() {
        let rom = test_rom();
        let bus = Bus::new(Rom::new(&rom).unwrap());
        let mut cpu = Cpu::new(bus);
        cpu.bus.mem_write(0x0000, 0x05);

        assert_eq!(cpu.get_op_addr(AddressingMode::ZeroPage), 0x05);
        assert_eq!(cpu.pc, 0x0001);

        assert_eq!(cpu.get_op_addr(AddressingMode::ZeroPage), 0x00);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn op_addr_zero_page_x() {
        let rom = test_rom();
        let bus = Bus::new(Rom::new(&rom).unwrap());
        let mut cpu = Cpu::new(bus);
        cpu.bus.mem_write(0x0000, 0x05);
        cpu.reg_x = 0x05;

        assert_eq!(cpu.get_op_addr(AddressingMode::ZeroPageX), 0x0A);
        assert_eq!(cpu.pc, 0x0001);

        assert_eq!(cpu.get_op_addr(AddressingMode::ZeroPageX), 0x05);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn op_addr_zero_page_y() {
        let rom = test_rom();
        let bus = Bus::new(Rom::new(&rom).unwrap());
        let mut cpu = Cpu::new(bus);
        cpu.bus.mem_write(0x0000, 0x05);
        cpu.reg_y = 0x05;

        assert_eq!(cpu.get_op_addr(AddressingMode::ZeroPageY), 0x0A);
        assert_eq!(cpu.pc, 0x0001);

        assert_eq!(cpu.get_op_addr(AddressingMode::ZeroPageY), 0x05);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn op_addr_absolute() {
        let rom = test_rom();
        let bus = Bus::new(Rom::new(&rom).unwrap());
        let mut cpu = Cpu::new(bus);
        cpu.bus.mem_write_u16(0x0000, 0x1234);

        assert_eq!(cpu.get_op_addr(AddressingMode::Absolute), 0x1234);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn op_addr_absolute_x() {
        let rom = test_rom();
        let bus = Bus::new(Rom::new(&rom).unwrap());
        let mut cpu = Cpu::new(bus);
        cpu.bus.mem_write_u16(0x0000, 0x1234);
        cpu.reg_x = 0x05;

        assert_eq!(cpu.get_op_addr(AddressingMode::AbsoluteX), 0x1239);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn op_addr_absolute_y() {
        let rom = test_rom();
        let bus = Bus::new(Rom::new(&rom).unwrap());
        let mut cpu = Cpu::new(bus);
        cpu.bus.mem_write_u16(0x0000, 0x1234);
        cpu.reg_y = 0x05;

        assert_eq!(cpu.get_op_addr(AddressingMode::AbsoluteY), 0x1239);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn op_addr_indirect() {
        let rom = test_rom();
        let bus = Bus::new(Rom::new(&rom).unwrap());
        let mut cpu = Cpu::new(bus);
        cpu.bus.mem_write_u16(0x0000, 0x1234);
        cpu.bus.mem_write_u16(0x1234, 0x5678);

        assert_eq!(cpu.get_op_addr(AddressingMode::Indirect), 0x5678);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn op_addr_indirect_x() {
        let rom = test_rom();
        let bus = Bus::new(Rom::new(&rom).unwrap());
        let mut cpu = Cpu::new(bus);
        cpu.bus.mem_write(0x0000, 0x12);
        cpu.bus.mem_write_u16(0x0012, 0x1234);
        cpu.reg_x = 0x05;

        assert_eq!(cpu.get_op_addr(AddressingMode::IndirectX), 0x1239);
        assert_eq!(cpu.pc, 0x0001);
    }

    #[test]
    fn op_addr_indirect_y() {
        let rom = test_rom();
        let bus = Bus::new(Rom::new(&rom).unwrap());
        let mut cpu = Cpu::new(bus);
        cpu.bus.mem_write(0x0000, 0x12);
        cpu.bus.mem_write_u16(0x0012, 0x1234);
        cpu.reg_y = 0x05;

        assert_eq!(cpu.get_op_addr(AddressingMode::IndirectY), 0x1239);
        assert_eq!(cpu.pc, 0x0001);
    }

    #[test]
    #[should_panic = "NoneAddressing is not a valid addressing mode"]
    fn op_addr_none_addressing() {
        let rom = test_rom();
        let bus = Bus::new(Rom::new(&rom).unwrap());
        let mut cpu = Cpu::new(bus);

        cpu.get_op_addr(AddressingMode::NoneAddressing);
    }
}
