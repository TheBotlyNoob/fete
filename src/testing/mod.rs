mod logger;

use crate::rom::*;

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
    pub pgp_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

pub fn create_rom(rom: TestRom) -> Vec<u8> {
    let mut result = Vec::with_capacity(
        rom.header.len()
            + rom.trainer.as_ref().map_or(0, Vec::len)
            + rom.pgp_rom.len()
            + rom.chr_rom.len(),
    );

    result.extend(&rom.header);
    if let Some(t) = rom.trainer {
        result.extend(t);
    }
    result.extend(&rom.pgp_rom);
    result.extend(&rom.chr_rom);

    result
}

pub fn test_rom() -> Vec<u8> {
    create_rom(TestRom {
        header: vec![
            0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x31, 00, 00, 00, 00, 00, 00, 00, 00, 00,
        ],
        trainer: None,
        pgp_rom: vec![1; 2 * PRG_ROM_PAGE_SIZE],
        chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
    })
}
