use fete::{
    bus::Bus,
    cpu::{Cpu, Status},
    rom::Rom,
    testing::trace_cpu,
};

static NESTEST_ROM: &[u8] = include_bytes!("../tests/nestest/nestest.nes");

#[test]
fn cpu_test() {
    let rom = Rom::new(NESTEST_ROM).unwrap();
    let bus = Bus::new(rom);

    let mut cpu = Cpu::new(bus);

    cpu.status = Status::INTERRUPT_DISABLE | Status::UNUSED;
    cpu.pc = 0xC000;

    loop {
        println!("{}", trace_cpu(&cpu).unwrap());
        if cpu.tick().unwrap() {
            break;
        }
    }
}
