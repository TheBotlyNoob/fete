#![allow(clippy::module_name_repetitions)]

use super::Cpu;
use crate::{
    cpu::AddressingMode,
    opcode::{OpCode, OPCODES},
};
use core::fmt::{Display, Write};

// matches the log format of NESticle

pub struct TraceAddrMode<'cpu> {
    cpu: &'cpu Cpu<'cpu>,
    op: &'cpu OpCode,
}
impl<'a> Display for TraceAddrMode<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // size outputted- used for padding later
        let mut out_size = 0_usize;

        let addr_mode = self.op.mode;
        if addr_mode != AddressingMode::NoneAddressing {
            let pc = self.cpu.pc + 1;

            let real_addr = {
                let mut cloned = self.cpu.clone();
                cloned.pc = pc;
                cloned.get_op_addr(addr_mode)
            };

            let got_addr = match addr_mode {
                AddressingMode::Immediate => {
                    let val = self.cpu.bus.mem_read(pc);

                    write!(f, "#${val:02X}")?;
                    out_size += "#$xx".len();

                    pc
                }
                AddressingMode::ZeroPage => {
                    let addr = self.cpu.bus.mem_read(pc);
                    let val = self.cpu.bus.mem_read(u16::from(addr));

                    write!(f, "${addr:02X} = {val:02X}")?;
                    out_size += "#xx = xx".len();

                    u16::from(addr)
                }
                AddressingMode::ZeroPageX => {
                    let addr = self.cpu.bus.mem_read(pc);
                    let with_x = addr.wrapping_add(self.cpu.reg_x);
                    let val = self.cpu.bus.mem_read(u16::from(with_x));

                    write!(f, "${addr:02X},X @ {with_x:02X} = {val:02X}")?;
                    out_size += "$xx,X @ xx = xx".len();

                    u16::from(with_x)
                }
                AddressingMode::IndirectX => {
                    let addr = self.cpu.bus.mem_read(pc);
                    let with_x = addr.wrapping_add(self.cpu.reg_x);
                    let real_addr = self.cpu.bus.mem_read_u16(u16::from(with_x));
                    let val = self.cpu.bus.mem_read(real_addr);

                    write!(
                        f,
                        "(${addr:02X},X) @ {with_x:02X} = {real_addr:04X} = {val:02X}"
                    )?;
                    out_size += "($xx,X) @ xx = xxxx = xx".len();

                    real_addr
                }
                AddressingMode::Relative => {
                    let addend = self.cpu.bus.mem_read(pc) + 1;
                    let addr = pc + u16::from(addend);

                    write!(f, "${addr:04X}")?;
                    out_size += "$xxxx".len();

                    addr
                }
                AddressingMode::Absolute => {
                    let addr = self.cpu.bus.mem_read_u16(pc);
                    if self.op.code == 0x4C || self.op.code == 0x20
                    /* JMP & JSR absolute */
                    {
                        write!(f, "${addr:04X}")?;
                        out_size += "$xxxx".len();
                    } else {
                        let val = self.cpu.bus.mem_read(addr);
                        write!(f, "${addr:04X} = {val:02X}")?;
                        out_size += "$xxxx = xx".len();
                    }
                    addr
                }
                mode => todo!("{mode:#?}"),
            };

            assert_eq!(got_addr, real_addr);
        }

        for _ in 0..(f.width().unwrap_or(0).saturating_sub(out_size)) {
            f.write_char(' ')?;
        }

        Ok(())
    }
}

pub struct TraceOp<'cpu> {
    pub cpu: &'cpu Cpu<'cpu>,
    pub op: &'cpu OpCode,
}
impl<'a> TraceOp<'a> {
    #[must_use]
    pub fn new(cpu: &'a Cpu<'a>) -> Option<Self> {
        Some(Self {
            cpu,
            op: OPCODES.get(&cpu.bus.mem_read(cpu.pc))?,
        })
    }
}
impl<'a> Display for TraceOp<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:04X} {:10} {} {:27} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.cpu.pc,
            TraceBytes {
                cpu: self.cpu,
                op: self.op,
            },
            DisplayUppercase(self.op.name),
            TraceAddrMode {
                cpu: self.cpu,
                op: self.op
            },
            self.cpu.reg_a,
            self.cpu.reg_x,
            self.cpu.reg_y,
            self.cpu.status.bits(),
            self.cpu.sp
        )
    }
}

struct TraceBytes<'cpu> {
    cpu: &'cpu Cpu<'cpu>,
    op: &'cpu OpCode,
}
impl<'a> Display for TraceBytes<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let op_size = self.op.mode.size() + 1;
        for i in 0..op_size {
            write!(
                f,
                " {:02X}",
                self.cpu.bus.mem_read(self.cpu.pc + u16::from(i))
            )?;
        }
        for _ in 0..(f
            .width()
            .unwrap_or(0)
            .saturating_sub(usize::from(op_size * 3)))
        {
            f.write_char(' ')?;
        }
        Ok(())
    }
}

struct DisplayUppercase<'a>(&'a str);
impl<'a> Display for DisplayUppercase<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for c in self.0.chars() {
            f.write_char(c.to_ascii_uppercase())?;
        }
        Ok(())
    }
}
