use fete::{
    bus::Bus,
    cpu::{trace::TraceOp, Cpu, Status},
    rom::Rom,
};
use pretty_assertions::assert_eq;

static NESTEST_ROM: &[u8] = include_bytes!("../tests/nestest/nestest.nes");
static NESTICLE_LOG: &str = include_str!("../tests/nestest/nestest.log");

#[test]
fn cpu_test() {
    let rom = Rom::new(NESTEST_ROM).unwrap();
    let bus = Bus::new(rom);

    let mut cpu = Cpu::new(bus);

    cpu.status = Status::INTERRUPT_DISABLE | Status::BREAK2;
    cpu.pc = 0xC000;

    for line in NESTICLE_LOG.lines().map(|l| l.split_at(73).0) {
        // TODO: CPU cycles.

        let trace = TraceOp::new(&cpu).unwrap().to_string();

        assert_eq!(trace, line);
        if cpu.tick().unwrap() {
            break;
        }
    }
}
