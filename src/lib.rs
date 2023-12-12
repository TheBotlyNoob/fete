#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

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

pub struct Cpu {
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    // not using the bitflags crate because I want to get better at bit manipulation
    // probably could, tho.
    pub status: u8,
    pub pc: u16,
    pub mem: [u8; 0xffff],
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            status: 0,
            pc: 0,
            mem: [0; 0xffff],
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
            AddressingMode::Immediate => self.pc,
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
            _ => todo!("addressing mode {mode:#?} not supported"),
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

    pub fn lda(&mut self, val: u8) {
        self.reg_a = val;
        self.zero_and_neg_flags(val);
    }

    pub fn ldx(&mut self, val: u8) {
        self.reg_x = val;
        self.zero_and_neg_flags(val);
    }

    pub fn ldy(&mut self, val: u8) {
        self.reg_y = val;
        self.zero_and_neg_flags(val);
    }

    pub fn tax(&mut self) {
        self.reg_x = self.reg_a;
        self.zero_and_neg_flags(self.reg_x);
    }

    pub fn inx(&mut self) {
        self.reg_x += 1;
        self.zero_and_neg_flags(self.reg_x);
    }

    pub fn zero_and_neg_flags(&mut self, val: u8) {
        if val == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }

        if val & 0b1000_0000 == 0b1000_0000 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.take();

            match opcode {
                // LDA {val}
                0xa9 => {
                    let val = self.take();
                    self.lda(val);
                }
                // LDX {val}
                0xa2 => {
                    let val = self.take();
                    self.ldx(val);
                }
                // LDY {val}
                0xa0 => {
                    let val = self.take();
                    self.ldy(val);
                }
                // TAX
                0xaa => self.tax(),
                // INX
                0xe8 => self.inx(),
                0x00 => {
                    // BRK
                    self.pc += 1;
                    break;
                }
                _ => todo!("opcode {:02x}", opcode),
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

    fn mem_read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, val: u8) {
        self.mem[addr as usize] = val;
    }

    fn mem_read_u16(&self, addr: u16) -> u16 {
        let lo = self.mem_read(addr);
        let hi = self.mem_read(addr + 1);
        u16::from_le_bytes([lo, hi])
    }

    fn mem_write_u16(&mut self, addr: u16, val: u16) {
        let [lo, hi] = val.to_le_bytes();
        self.mem_write(addr, lo);
        self.mem_write(addr + 1, hi);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! test_load {
        ($name:ident, $hex:literal, $reg:ident) => {
            paste::paste! {
                #[test]
                fn [<$name _immediate_load_data>]() {
                    let mut cpu = Cpu::new();
                    cpu.load_and_run(&[$hex, 0x05, 0x00]);
                    assert_eq!(cpu.$reg, 0x05);
                    assert_eq!(cpu.status & 0b0000_0010, 0b00);
                    assert_eq!(cpu.status & 0b1000_0000, 0);
                }
                #[test]
                fn [<$name _zero_flag>]() {
                    let mut cpu = Cpu::new();
                    cpu.load_and_run(&[$hex, 0x00, 0x00]);
                    assert_eq!(cpu.$reg, 0x00);
                    assert_eq!(cpu.status & 0b0000_0010, 0b10);
                    assert_eq!(cpu.status & 0b1000_0000, 0);
                }
            }
        };
    }

    test_load!(lda_0xa9, 0xa9, reg_a);
    test_load!(ldx_0xa2, 0xa2, reg_x);
    test_load!(ldy_0xa0, 0xa0, reg_y);

    #[test]
    fn tax_0xaa_immediate_load_data() {
        let mut cpu = Cpu::new();
        // LDA #$05
        // TAX
        // BRK
        cpu.load_and_run(&[0xa9, 0x05, 0xaa, 0x00]);
        assert!(cpu.reg_x == 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn tax_0xaa_zero_flag() {
        let mut cpu = Cpu::new();
        // LDA #$00
        // TAX
        // BRK
        cpu.load_and_run(&[0xa9, 0x00, 0xaa, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn inx_0xe8_zero_flag() {
        let mut cpu = Cpu::new();
        // LD
    }
}
