use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    NoneAddressing,
}

bitflags! {
    /// Status register flags.
    #[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
    pub struct Status: u8 {
        /// Carry flag.
        const CARRY = 0b0000_0001;
        /// Zero flag.
        const ZERO = 0b0000_0010;
        /// Interrupt disable flag.
        const INTERRUPT_DISABLE = 0b0000_0100;
        /// Decimal mode flag.
        const DECIMAL_MODE = 0b0000_1000;
        /// Break flag.
        const BREAK = 0b0001_0000;
        /// Unused flag.
        const UNUSED = 0b0010_0000;
        /// Overflow flag.
        const OVERFLOW = 0b0100_0000;
        /// Negative flag.
        const NEGATIVE = 0b1000_0000;
    }
}

pub struct Cpu {
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    pub status: Status,
    pub sp: u8,
    pub pc: u16,
    pub mem: [u8; 0xFFFF],
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            status: Status::default(),
            sp: 0xFD,
            pc: 0,
            mem: [0; 0xFFFF],
        }
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self {
            pc: self.mem_read_u16(0xFFFC),
            mem: self.mem,
            ..Self::default()
        }
    }

    /// Gets the address at the current program count, using the given [`AddressingMode`]. Increments the program count.
    pub fn get_op_addr(&mut self, mode: AddressingMode) -> u16 {
        #[allow(clippy::match_wildcard_for_single_variants)] // that's the point
        match mode {
            AddressingMode::Immediate => {
                let prev_pc = self.pc;
                self.pc += 1;
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
                self.mem_read_u16(real_addr)
            }
            AddressingMode::IndirectX => {
                let real_addr = u16::from(self.take());
                self.mem_read_u16(real_addr) + u16::from(self.reg_x)
            }
            AddressingMode::IndirectY => {
                self.mem_read_u16(u16::from(self.mem_read(self.pc))) + u16::from(self.reg_y)
            }
            AddressingMode::NoneAddressing => {
                unreachable!("AddressingMode::NoneAddressing is not a valid addressing mode")
            }
        }
    }

    pub fn load(&mut self, prog: &[u8]) {
        self.mem[0x8000..0x8000 + prog.len()].copy_from_slice(prog);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn load_and_run(&mut self, prog: &[u8]) {
        self.load(prog);
        self.reset();
        self.run();
    }

    /// Loads the given value into the accumulator, and sets the zero and negative flags.
    ///
    /// # Examples
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use fete::{cpu::Status, Cpu};
    ///
    /// let mut cpu = Cpu::new();
    ///
    /// // LDA #$05
    /// // BRK
    /// cpu.load_and_run(&[0xA9, 0x05, 0x00]);
    ///
    /// assert_eq!(cpu.reg_a, 0x05);
    /// assert_eq!(cpu.status, Status::BREAK);
    /// ```
    pub fn lda(&mut self, mode: AddressingMode) {
        let addr = self.get_op_addr(mode);
        let val = self.mem_read(addr);

        self.reg_a = val;
        self.zero_and_neg_flags(val);
    }

    /// Loads the given value into the X register, and sets the zero and negative flags.
    ///
    /// # Examples
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use fete::{cpu::Status, Cpu};
    ///
    /// let mut cpu = Cpu::new();
    ///
    /// // LDX #$05
    /// // BRK
    /// cpu.load_and_run(&[0xA2, 0x05, 0x00]);
    ///
    /// assert_eq!(cpu.reg_x, 0x05);
    /// assert_eq!(cpu.status, Status::BREAK);
    /// ```
    pub fn ldx(&mut self, mode: AddressingMode) {
        let addr = self.get_op_addr(mode);
        let val = self.mem_read(addr);

        self.reg_x = val;
        self.zero_and_neg_flags(val);
    }

    /// Loads the given value into the Y register, and sets the zero and negative flags.
    ///
    /// # Examples
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use fete::{cpu::Status, Cpu};
    ///
    /// let mut cpu = Cpu::new();
    ///
    /// // LDY #$05
    /// // BRK
    /// cpu.load_and_run(&[0xA0, 0x05, 0x00]);
    ///
    /// assert_eq!(cpu.reg_y, 0x05);
    /// assert_eq!(cpu.status, Status::BREAK);
    /// ```
    pub fn ldy(&mut self, mode: AddressingMode) {
        let addr = self.get_op_addr(mode);
        let val = self.mem_read(addr);

        self.reg_y = val;
        self.zero_and_neg_flags(val);
    }

    /// Stores the value in the accumulator into memory.
    ///
    /// # Examples
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use fete::Cpu;
    ///
    /// let mut cpu = Cpu::new();
    /// cpu.reg_a = 0x05;
    ///
    /// // STA $8000
    /// // BRK
    /// cpu.load_and_run(&[0x8D, 0x00, 0x80, 0x00]); // keep in mind that the 16-bit address is stored in little-endian
    ///
    /// assert_eq!(cpu.mem_read(0x8000), 0x05);
    /// ```
    pub fn sta(&mut self, mode: AddressingMode) {
        let addr = self.get_op_addr(mode);
        self.mem_write(addr, self.reg_a);
    }

    /// Stores the value in the X register into memory.
    ///
    /// # Examples
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use fete::Cpu;
    ///
    /// let mut cpu = Cpu::new();
    /// cpu.reg_x = 0x05;
    ///
    /// // STX $8000
    /// // BRK
    /// cpu.load_and_run(&[0x8E, 0x00, 0x80, 0x00]); // keep in mind that the 16-bit address is stored in little-endian
    ///
    /// assert_eq!(cpu.mem_read(0x8000), 0x05);
    /// ```
    pub fn stx(&mut self, mode: AddressingMode) {
        let addr = self.get_op_addr(mode);
        self.mem_write(addr, self.reg_x);
    }

    /// Stores the value in the Y register into memory.
    ///
    /// # Examples
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use fete::Cpu;
    ///
    /// let mut cpu = Cpu::new();
    /// cpu.reg_y = 0x05;
    ///
    /// // STX $8000
    /// // BRK
    /// cpu.load_and_run(&[0x8C, 0x00, 0x80, 0x00]); // keep in mind that the 16-bit address is stored in little-endian
    ///
    /// assert_eq!(cpu.mem_read(0x8000), 0x05);
    /// ```
    pub fn sty(&mut self, mode: AddressingMode) {
        let addr = self.get_op_addr(mode);
        self.mem_write(addr, self.reg_x);
    }

    /// Transfers the value in the accumulator to the X register, and sets the zero and negative flags.
    ///
    /// # Examples
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use fete::{cpu::Status, Cpu};
    ///
    /// let mut cpu = Cpu::new();
    /// cpu.reg_a = 0x05;
    ///
    /// // TAX
    /// // BRK
    /// cpu.load_and_run(&[0xAA, 0x00]);
    ///
    /// assert_eq!(cpu.reg_x, 0x05);
    /// assert_eq!(cpu.status, Status::BREAK);
    /// ```
    pub fn tax(&mut self, _mode: AddressingMode) {
        self.reg_x = self.reg_a;
        self.zero_and_neg_flags(self.reg_x);
    }

    /// Transfers the value in the accumulator to the Y register, and sets the zero and negative flags.
    ///
    /// # Examples
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use fete::{cpu::Status, Cpu};
    ///
    /// let mut cpu = Cpu::new();
    /// cpu.reg_a = 0x05;
    ///
    /// // TAY
    /// // BRK
    /// cpu.load_and_run(&[0xA8, 0x00]);
    ///
    /// assert_eq!(cpu.reg_y, 0x05);
    /// assert_eq!(cpu.status, Status::BREAK);
    /// ```
    pub fn tay(&mut self, _mode: AddressingMode) {
        self.reg_y = self.reg_a;
        self.zero_and_neg_flags(self.reg_y);
    }

    /// Transfers the value in the stack pointer to the X register, and sets the zero and negative flags.
    ///
    /// # Examples
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use fete::{cpu::Status, Cpu};
    ///
    /// let mut cpu = Cpu::new();
    /// cpu.sp = 0x05;
    ///
    /// // TSX
    /// // BRK
    /// cpu.load_and_run(&[0xBA, 0x00]);
    ///
    /// assert_eq!(cpu.reg_x, 0x05);
    /// assert_eq!(cpu.status, Status::BREAK);
    /// ```
    pub fn tsx(&mut self, _mode: AddressingMode) {
        self.reg_x = self.sp;
        self.zero_and_neg_flags(self.reg_x);
    }

    /// Transfers the value in the X register to the accumulator, and sets the zero and negative flags.
    ///
    /// # Examples
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use fete::{cpu::Status, Cpu};
    ///
    /// let mut cpu = Cpu::new();
    /// cpu.reg_x = 0x05;
    ///
    /// // TXA
    /// // BRK
    /// cpu.load_and_run(&[0x8A, 0x00]);
    ///
    /// assert_eq!(cpu.reg_a, 0x05);
    /// assert_eq!(cpu.status, Status::BREAK);
    /// ```
    pub fn txa(&mut self, _mode: AddressingMode) {
        self.reg_a = self.reg_x;
        self.zero_and_neg_flags(self.reg_a);
    }

    /// Transfers the value in the X register to the stack pointer.
    ///
    /// # Examples
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use fete::{cpu::Status, Cpu};
    ///
    /// let mut cpu = Cpu::new();
    /// cpu.reg_x = 0x05;
    ///
    /// // TXS
    /// // BRK
    /// cpu.load_and_run(&[0x9A, 0x00]);
    ///
    /// assert_eq!(cpu.sp, 0x05);
    /// ```
    pub fn txs(&mut self, _mode: AddressingMode) {
        self.sp = self.reg_x;
    }

    /// Transfers the value in the Y register to the accumulator, and sets the zero and negative flags.
    ///
    /// # Examples
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use fete::{cpu::Status, Cpu};
    ///
    /// let mut cpu = Cpu::new();
    /// cpu.reg_y = 0x05;
    ///
    /// // TYA
    /// // BRK
    /// cpu.load_and_run(&[0x98, 0x00]);
    ///
    /// assert_eq!(cpu.reg_a, 0x05);
    /// assert_eq!(cpu.status, Status::BREAK);
    /// ```
    pub fn tya(&mut self, _mode: AddressingMode) {
        self.reg_a = self.reg_y;
        self.zero_and_neg_flags(self.reg_a);
    }

    /// Increments the X register, and sets the zero and negative flags.
    ///
    /// # Examples
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use fete::{cpu::Status, Cpu};
    ///
    /// let mut cpu = Cpu::new();
    /// cpu.reg_x = 0x05;
    ///
    /// // INX
    /// // BRK
    /// cpu.load_and_run(&[0xE8, 0x00]);
    ///
    /// assert_eq!(cpu.reg_x, 0x06);
    /// assert_eq!(cpu.status, Status::BREAK);
    /// ```
    pub fn inx(&mut self, _mode: AddressingMode) {
        self.reg_x += 1;
        self.zero_and_neg_flags(self.reg_x);
    }

    /// Increments the Y register, and sets the zero and negative flags.
    ///
    /// # Examples
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use fete::{cpu::Status, Cpu};
    ///
    /// let mut cpu = Cpu::new();
    /// cpu.reg_y = 0x05;
    ///
    /// // INY
    /// // BRK
    /// cpu.load_and_run(&[0xC8, 0x00]);
    ///
    /// assert_eq!(cpu.reg_y, 0x06);
    /// assert_eq!(cpu.status, Status::BREAK);
    /// ```
    pub fn iny(&mut self, _mode: AddressingMode) {
        self.reg_y += 1;
        self.zero_and_neg_flags(self.reg_y);
    }

    /// Breaks the program, and sets the break flag.
    ///
    /// # Examples
    /// ```no_run
    /// # use pretty_assertions::assert_eq;
    /// use fete::{cpu::Status, Cpu};
    ///
    /// let mut cpu = Cpu::new();
    /// cpu.load_and_run(&[0x00]);
    ///
    /// assert_eq!(cpu.status, Status::BREAK);
    /// ```
    pub fn brk(&mut self, _mode: AddressingMode) {
        self.pc += 1;
        self.status |= Status::BREAK;
        // TODO: impl. stack + interrupts
    }

    pub fn zero_and_neg_flags(&mut self, val: u8) {
        if val == 0 {
            self.status |= Status::ZERO;
        } else {
            self.status = self.status.difference(Status::ZERO);
        }

        if val & 0b1000_0000 == 0b1000_0000 {
            self.status |= Status::NEGATIVE;
        } else {
            self.status = self.status.difference(Status::NEGATIVE);
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.take();
            let opcode_info = crate::opcode::OPCODES.get(&opcode);

            if let Some(opcode) = opcode_info {
                (opcode.op)(self, opcode.mode);
            } else {
                todo!("opcode {opcode:#02x} not found")
            }

            if self.status.contains(Status::BREAK) {
                break;
            }
        }
    }

    fn take(&mut self) -> u8 {
        let byte = self.mem_read(self.pc);
        self.pc += 1;
        byte
    }
    fn take_u16(&mut self) -> u16 {
        let num = self.mem_read_u16(self.pc);
        self.pc += 2;
        num
    }

    pub const fn mem_read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn mem_write(&mut self, addr: u16, val: u8) {
        self.mem[addr as usize] = val;
    }

    pub const fn mem_read_u16(&self, addr: u16) -> u16 {
        let lo = self.mem_read(addr);
        let hi = self.mem_read(addr + 1);
        u16::from_le_bytes([lo, hi])
    }

    pub fn mem_write_u16(&mut self, addr: u16, val: u16) {
        let [lo, hi] = val.to_le_bytes();
        self.mem_write(addr, lo);
        self.mem_write(addr + 1, hi);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn zero_and_neg_flags() {
        let mut cpu = Cpu::new();
        cpu.zero_and_neg_flags(0);
        assert_eq!(cpu.status, Status::ZERO);
        cpu.zero_and_neg_flags(1);
        assert_eq!(cpu.status, Status::empty());
        cpu.zero_and_neg_flags(0b1000_0000);
        assert_eq!(cpu.status, Status::NEGATIVE);
        cpu.zero_and_neg_flags(0b0111_1111);
        assert_eq!(cpu.status, Status::empty());
    }
}
