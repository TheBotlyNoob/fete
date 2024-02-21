use fete::{
    bus::Bus,
    cpu::{AddressingMode, Cpu, Status},
    opcode::OPCODES,
    rom::Rom,
};
use pretty_assertions::assert_eq;
use std::fmt::Write;

static NESTEST_ROM: &[u8] = include_bytes!("../tests/nestest/nestest.nes");
static NESTICLE_LOG: &str = include_str!("../tests/nestest/nestest.log");

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

#[test]
fn cpu_test() {
    let rom = Rom::new(NESTEST_ROM).unwrap();
    let bus = Bus::new(rom);

    let mut cpu = Cpu::new(bus);

    cpu.status = Status::INTERRUPT_DISABLE | Status::UNUSED;
    cpu.pc = 0xC000;

    for line in NESTICLE_LOG.lines().map(|l| l.split_at(73).0) {
        // TODO: CPU cycles.

        let trace = trace_cpu(&cpu).unwrap();
        eprintln!("{trace}");
        assert_eq!(trace, line);
        if cpu.tick().unwrap() {
            break;
        }
    }
}
