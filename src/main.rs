use std::hint::black_box;

const EXAMPLE_PROGRAM: &[u8] = &[
    0x20, 0x06, 0x00, 0x20, 0x38, 0x00, 0x20, 0x0D, 0x00, 0x20, 0x2A, 0x00, 0x60, 0xA9, 0x02, 0x85,
    0x02, 0xA9, 0x04, 0x85, 0x03, 0xA9, 0x11, 0x85, 0x10, 0xA9, 0x10, 0x85, 0x12, 0xA9, 0x0F, 0x85,
    0x14, 0xA9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60, 0xA5, 0xFE, 0x85, 0x00, 0xA5, 0xFE,
    0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60, 0x20, 0x4D, 0x00, 0x20, 0x8D, 0x00, 0x20, 0xC3,
    0x00, 0x20, 0x19, 0x01, 0x20, 0x20, 0x01, 0x20, 0x2D, 0x01, 0x4C, 0x38, 0x00, 0xA5, 0xFF, 0xC9,
    0x77, 0xF0, 0x0D, 0xC9, 0x64, 0xF0, 0x14, 0xC9, 0x73, 0xF0, 0x1B, 0xC9, 0x61, 0xF0, 0x22, 0x60,
    0xA9, 0x04, 0x24, 0x02, 0xD0, 0x26, 0xA9, 0x01, 0x85, 0x02, 0x60, 0xA9, 0x08, 0x24, 0x02, 0xD0,
    0x1B, 0xA9, 0x02, 0x85, 0x02, 0x60, 0xA9, 0x01, 0x24, 0x02, 0xD0, 0x10, 0xA9, 0x04, 0x85, 0x02,
    0x60, 0xA9, 0x02, 0x24, 0x02, 0xD0, 0x05, 0xA9, 0x08, 0x85, 0x02, 0x60, 0x60, 0x20, 0x94, 0x00,
    0x20, 0xA8, 0x00, 0x60, 0xA5, 0x00, 0xC5, 0x10, 0xD0, 0x0D, 0xA5, 0x01, 0xC5, 0x11, 0xD0, 0x07,
    0xE6, 0x03, 0xE6, 0x03, 0x20, 0x2A, 0x00, 0x60, 0xA2, 0x02, 0xB5, 0x10, 0xC5, 0x10, 0xD0, 0x06,
    0xB5, 0x11, 0xC5, 0x11, 0xF0, 0x09, 0xE8, 0xE8, 0xE4, 0x03, 0xF0, 0x06, 0x4C, 0xAA, 0x00, 0x4C,
    0x35, 0x01, 0x60, 0xA6, 0x03, 0xCA, 0x8A, 0xB5, 0x10, 0x95, 0x12, 0xCA, 0x10, 0xF9, 0xA5, 0x02,
    0x4A, 0xB0, 0x09, 0x4A, 0xB0, 0x19, 0x4A, 0xB0, 0x1F, 0x4A, 0xB0, 0x2F, 0xA5, 0x10, 0x38, 0xE9,
    0x20, 0x85, 0x10, 0x90, 0x01, 0x60, 0xC6, 0x11, 0xA9, 0x01, 0xC5, 0x11, 0xF0, 0x28, 0x60, 0xE6,
    0x10, 0xA9, 0x1F, 0x24, 0x10, 0xF0, 0x1F, 0x60, 0xA5, 0x10, 0x18, 0x69, 0x20, 0x85, 0x10, 0xB0,
    0x01, 0x60, 0xE6, 0x11, 0xA9, 0x06, 0xC5, 0x11, 0xF0, 0x0C, 0x60, 0xC6, 0x10, 0xA5, 0x10, 0x29,
    0x1F, 0xC9, 0x1F, 0xF0, 0x01, 0x60, 0x4C, 0x35, 0x01, 0xA0, 0x00, 0xA5, 0xFE, 0x91, 0x00, 0x60,
    0xA2, 0x00, 0xA9, 0x01, 0x81, 0x10, 0xA6, 0x03, 0xA9, 0x00, 0x81, 0x10, 0x60, 0xA2, 0x00, 0xEA,
    0xEA, 0xCA, 0xD0, 0xFB, 0x60,
];

fn main() {
    black_box(
        fete::cpu::Cpu::new(fete::bus::Bus::new(todo!()))
            .load_and_run(EXAMPLE_PROGRAM)
            .unwrap(),
    );
}
