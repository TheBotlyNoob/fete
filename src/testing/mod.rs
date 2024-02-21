mod logger;

use crate::{
    cpu::{AddressingMode, Cpu},
    opcode::OPCODES,
    rom::*,
};
use std::fmt::Write;

#[used]
#[doc(hidden)]
#[cfg_attr(
    any(target_os = "linux", target_os = "android"),
    link_section = ".init_array"
)]
#[cfg_attr(target_os = "freebsd", link_section = ".init_array")]
#[cfg_attr(target_os = "netbsd", link_section = ".init_array")]
#[cfg_attr(target_os = "openbsd", link_section = ".init_array")]
#[cfg_attr(target_os = "illumos", link_section = ".init_array")]
#[cfg_attr(
    any(target_os = "macos", target_os = "ios"),
    link_section = "__DATA_CONST,__mod_init_func"
)]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
static INIT_LOGGER_STARTUP: unsafe extern "C" fn() -> usize = {
    #[cfg_attr(
        any(target_os = "linux", target_os = "android"),
        link_section = ".text.startup"
    )]
    unsafe extern "C" fn init_logger_startup() -> usize {
        logger::init_with_env().unwrap();
        0
    }
    init_logger_startup
};

pub struct TestRom {
    pub header: Vec<u8>,
    pub trainer: Option<Vec<u8>>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

pub fn create_rom(rom: TestRom) -> Vec<u8> {
    let mut result = Vec::with_capacity(
        rom.header.len()
            + rom.trainer.as_ref().map_or(0, Vec::len)
            + rom.prg_rom.len()
            + rom.chr_rom.len(),
    );

    result.extend(&rom.header);
    if let Some(t) = rom.trainer {
        result.extend(t);
    }
    result.extend(&rom.prg_rom);
    result.extend(&rom.chr_rom);

    result
}

#[must_use = "does nothing without parsing it into a Rom"]
pub fn test_rom() -> Vec<u8> {
    create_rom(TestRom {
        header: vec![
            0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x31, 00, 00, 00, 00, 00, 00, 00, 00, 00,
        ],
        trainer: None,
        prg_rom: vec![0; 2 * PRG_ROM_PAGE_SIZE],
        chr_rom: vec![0; CHR_ROM_PAGE_SIZE],
    })
}

fn trace_addr_mode(cpu: &Cpu, addr_mode: AddressingMode) -> String {
    if addr_mode == AddressingMode::NoneAddressing {
        return String::new();
    }

    let pc = cpu.pc + 1;

    let real_addr = {
        let mut cloned = cpu.clone();
        cloned.pc = pc;
        cloned.get_op_addr(addr_mode)
    };

    let (got_addr, output) = match addr_mode {
        AddressingMode::Immediate => {
            let val = cpu.bus.mem_read(pc);
            (pc, format!("#${val:02X}"))
        }
        AddressingMode::ZeroPage => {
            let addr = cpu.bus.mem_read(pc);
            let val = cpu.bus.mem_read(u16::from(addr));
            (u16::from(addr), format!("${addr:02X} = {val:02X}"))
        }
        AddressingMode::ZeroPageX => {
            let addr = cpu.bus.mem_read(pc);
            let with_x = addr.wrapping_add(cpu.reg_x);
            let val = cpu.bus.mem_read(u16::from(with_x));
            (
                u16::from(with_x),
                format!("${addr:02X},X @ {with_x:02X} = {val}"),
            )
        }
        AddressingMode::IndirectX => {
            let addr = cpu.bus.mem_read(pc);
            let with_x = addr.wrapping_add(cpu.reg_x);
            let real_addr = cpu.bus.mem_read_u16(u16::from(with_x));
            let val = cpu.bus.mem_read(real_addr);
            (
                real_addr,
                format!("(${addr:02X},X) @ {with_x:02X} = {real_addr:04X} = {val:02X}"),
            )
        }
        AddressingMode::Relative => {
            let addend = cpu.bus.mem_read(pc) + 1;
            let addr = pc + u16::from(addend);
            (addr, format!("${addr:02X}"))
        }
        AddressingMode::Absolute => {
            let addr = cpu.bus.mem_read_u16(pc);
            (addr, format!("${addr:04X}"))
        }
        mode => todo!("{mode:#?}"),
    };

    assert_eq!(got_addr, real_addr);

    output
}
#[must_use]
pub fn trace_cpu(cpu: &Cpu) -> Option<String> {
    let opcode = cpu.bus.mem_read(cpu.pc);
    let Some(opcode) = OPCODES.get(&opcode) else {
        log::error!("OPCODE NOT FOUND: {opcode:#02X}");
        return None;
    };

    let bytes = (0..=opcode.mode.size()).fold(String::new(), |mut output, b| {
        let _ = write!(output, " {:02X}", cpu.bus.mem_read(cpu.pc + u16::from(b)));
        output
    });

    Some(format!(
        "{:04X} {:<10} {} {:<27} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
        cpu.pc,
        bytes,
        opcode.name.to_uppercase(),
        trace_addr_mode(cpu, opcode.mode),
        cpu.reg_a,
        cpu.reg_x,
        cpu.reg_y,
        cpu.status.bits(),
        cpu.sp
    ))
}
