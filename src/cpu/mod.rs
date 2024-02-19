use crate::bus::Bus;
use snafu::prelude::*;

pub mod status;
pub use status::Status;

pub mod addr_mode;
pub use addr_mode::AddressingMode;

#[derive(Snafu)]
pub enum Error {
    #[snafu(display("invalid opcode: {:#02x}", opcode))]
    InvalidOpcode { opcode: u8, offset: u16 },
}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidOpcode { opcode, offset } => f
                .debug_struct("InvalidOpcode")
                .field("opcode", &format_args!("{opcode:#02x}"))
                .field("offset", &format_args!("{offset:#02x}"))
                .finish(),
        }
    }
}

#[derive(Clone)]
pub struct Cpu<'rom> {
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    pub status: Status,
    pub sp: u8,
    pub pc: u16,
    pub bus: Bus<'rom>,
}

impl<'rom> core::fmt::Debug for Cpu<'rom> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Cpu")
            .field("reg_a", &format_args!("{:#02X}", self.reg_a))
            .field("reg_x", &format_args!("{:#02X}", self.reg_x))
            .field("reg_y", &format_args!("{:#02X}", self.reg_y))
            .field("status", &self.status)
            .field("sp", &format_args!("{:#02X}", self.sp))
            .field("pc", &format_args!("{:#04X}", self.pc))
            .field("bus", &"Bus { .. }")
            .finish()
    }
}

impl<'rom> Cpu<'rom> {
    pub const STACK: u16 = 0x0100;
    pub const STACK_RESET: u8 = 0xFD;

    /// Creates a new CPU with the default state.
    #[must_use]
    pub fn new(bus: Bus<'rom>) -> Self {
        Self {
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            status: Status::default(),
            sp: Self::STACK_RESET,
            pc: bus.mem_read_u16(0xFFFC),
            bus,
        }
    }

    /// Resets the CPU to its initial state. Keeps the memory intact.
    pub fn reset(&mut self) {
        replace_with::replace_with(self, || unreachable!(), |self_| Self::new(self_.bus));
    }

    // tests are located in `addr_mode.rs`
    /// Gets the address at the current program count, using the given [`AddressingMode`]. Increments the program count as needed.
    pub fn get_op_addr(&mut self, mode: AddressingMode) -> u16 {
        let prev_pc = self.pc; // does nothing outside of debug mode
        let addr = match mode {
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
                self.bus
                    .mem_read_u16(real_addr)
                    .wrapping_add(u16::from(self.reg_x))
                    % 0xFF
            }
            AddressingMode::IndirectY => {
                let real_addr = u16::from(self.take());
                self.bus
                    .mem_read_u16(real_addr)
                    .wrapping_add(u16::from(self.reg_y))
            }
            AddressingMode::Relative => {
                let offset = self.take(); // self.pc + 1
                self.pc.wrapping_add(u16::from(offset))
            }
            AddressingMode::NoneAddressing => {
                unreachable!("AddressingMode::NoneAddressing is not a valid addressing mode");
            }
        };
        debug_assert_eq!(prev_pc + u16::from(mode.size()), self.pc);
        addr
    }

    /// Loads the given program into memory, and sets the program counter to the start of the program.
    ///
    /// The program is truncated to `u16::MAX`.
    pub fn load(&mut self, prog: &[u8]) {
        for (i, &b) in prog.iter().enumerate().take(usize::from(u16::MAX)) {
            log::info!("addr: {:#X?}: {b:#X?}", 0x0600 + i);
            #[allow(clippy::cast_possible_truncation)] // already truncated to u16::MAX
            self.bus.mem_write(0x0600 + i as u16, b);
        }
        self.pc = match self.bus.mem_read_u16(0xFFFC) {
            0 => 0x0600,
            pc => pc,
        };
    }

    /// Loads the given program into memory, resets the CPU, and runs the program.
    ///
    /// # Errors
    /// Returns an [`Error::InvalidOpcode`] if an invalid opcode is encountered.
    pub fn load_and_run(&mut self, prog: &[u8]) -> Result<(), Error> {
        self.reset();
        self.load(prog);
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
    /// # use fete::{bus::Bus, rom::Rom, testing::test_rom};
    /// # use pretty_assertions::assert_eq;
    /// use fete::cpu::{Cpu, Status};
    ///
    /// # let rom = test_rom();
    /// # let bus = Bus::new(Rom::new(&rom).unwrap());
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
            log::info!(
                "{:#02X} {:#X} ({}) ({:#?})",
                self.pc - 1,
                opcode.code,
                opcode.name,
                opcode.mode
            );
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
            .mem_write(Self::STACK.saturating_add(u16::from(self.sp)), val);
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
        self.bus
            .mem_read(Self::STACK.saturating_add(u16::from(self.sp)))
    }

    /// Pops a little-endian, 16-bit number from the stack.
    pub fn pop_u16(&mut self) -> u16 {
        let lo = self.pop();
        let hi = self.pop();
        u16::from_le_bytes([lo, hi])
    }

    /// Takes the next byte from memory, and increments the program counter.
    pub fn take(&mut self) -> u8 {
        let byte = self.bus.mem_read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }
    /// Takes the next little-endian, 16-bit number from memory, and increments the program counter.
    pub fn take_u16(&mut self) -> u16 {
        let num = self.bus.mem_read_u16(self.pc);
        self.pc = self.pc.wrapping_add(2);
        num
    }
}
