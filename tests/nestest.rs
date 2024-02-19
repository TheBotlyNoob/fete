use fete::{bus::Bus, cpu::Cpu, rom::Rom, testing::trace_cpu};

static NESTEST_ROM: &[u8] = include_bytes!("../tests/nestest/nestest.nes");

#[test]
fn cpu_test() {
    let rom = Rom::new(NESTEST_ROM).unwrap();
    let bus = Bus::new(rom);

    let mut cpu = Cpu::new(bus);

    cpu.pc = 0xC000;

    cpu.run();

    println!("{}", trace_cpu(&cpu).unwrap());
}
