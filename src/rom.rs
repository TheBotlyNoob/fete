pub const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
pub const PRG_ROM_PAGE_SIZE: usize = 16384;
pub const CHR_ROM_PAGE_SIZE: usize = 8192;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, snafu::Snafu)]
pub enum Error {
    #[snafu(display("invalid magic bytes"))]
    InvalidMagicBytes,
    #[snafu(display("unsupported NES format"))]
    UnsupportedFormat,
    #[snafu(display("unexpected end of input"))]
    UnexpectedEOI,
}
impl From<untrusted::EndOfInput> for Error {
    fn from(_: untrusted::EndOfInput) -> Self {
        Self::UnexpectedEOI
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}
#[derive(Debug, Clone)]
pub struct Rom<'rom> {
    pub prg_rom: &'rom [u8],
    pub chr_rom: &'rom [u8],
    pub mapper: u8,
    pub mirroring: Mirroring,
}

impl<'a> Rom<'a> {
    pub fn new(raw: &'a [u8]) -> Result<Self, Error> {
        let mut reader = untrusted::Reader::new(untrusted::Input::from(raw));

        if reader.read_bytes(4)?.as_slice_less_safe() != NES_TAG {
            return Err(Error::InvalidMagicBytes);
        };

        let prg_rom_size = usize::from(reader.read_byte()?) * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = usize::from(reader.read_byte()?) * CHR_ROM_PAGE_SIZE;

        let flags_6 = reader.read_byte()?;
        let flags_7 = reader.read_byte()?;
        let _flags_8 = reader.read_byte()?;
        let _flags_9 = reader.read_byte()?;
        let _flags_10 = reader.read_byte()?;

        if (flags_7 >> 2) & 0b11 != 0 {
            return Err(Error::UnsupportedFormat);
        }

        let mapper = (flags_6 >> 4) | (flags_7 & 0xF0);

        let four_screen = flags_6 & 0b1000 != 0;
        let vert_mirroring = flags_6 & 0x0001 != 0;
        let mirroring = match (four_screen, vert_mirroring) {
            (true, _) => Mirroring::FourScreen,
            (_, true) => Mirroring::Vertical,
            (_, false) => Mirroring::Horizontal,
        };

        let trainer = flags_6 & 0b0100 != 0;
        if trainer {
            reader.read_bytes(512)?;
        }

        let prg_rom = reader.read_bytes(prg_rom_size)?.as_slice_less_safe();
        let chr_rom = reader.read_bytes(chr_rom_size)?.as_slice_less_safe();

        Ok(Self {
            prg_rom,
            chr_rom,
            mapper,
            mirroring,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::{create_rom, TestRom};
    use pretty_assertions::assert_eq;

    #[test]
    fn test() {
        let test_rom = create_rom(TestRom {
            header: vec![
                0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x31, 00, 00, 00, 00, 00, 00, 00, 00, 00,
            ],
            trainer: None,
            pgp_rom: vec![1; 2 * PRG_ROM_PAGE_SIZE],
            chr_rom: vec![2; CHR_ROM_PAGE_SIZE],
        });

        let rom = Rom::new(&test_rom).unwrap();

        // assert_eq!(rom.chr_rom, vec![2; CHR_ROM_PAGE_SIZE]);
        // assert_eq!(rom.prg_rom, vec![1; 2 * PRG_ROM_PAGE_SIZE]);
        assert_eq!(rom.mapper, 3);
        assert_eq!(rom.mirroring, Mirroring::Vertical);
    }

    #[test]
    fn test_with_trainer() {
        let test_rom = create_rom(TestRom {
            header: vec![
                0x4E,
                0x45,
                0x53,
                0x1A,
                0x02,
                0x01,
                0x31 | 0b100,
                00,
                00,
                00,
                00,
                00,
                00,
                00,
                00,
                00,
            ],
            trainer: Some(vec![0; 512]),
            pgp_rom: vec![1; 2 * PRG_ROM_PAGE_SIZE],
            chr_rom: vec![2; CHR_ROM_PAGE_SIZE],
        });

        let rom: Rom = Rom::new(&test_rom).unwrap();

        // assert_eq!(rom.chr_rom, vec![2; CHR_ROM_PAGE_SIZE]);
        // assert_eq!(rom.prg_rom, vec![1; 2 * PRG_ROM_PAGE_SIZE]);
        assert_eq!(rom.mapper, 3);
        assert_eq!(rom.mirroring, Mirroring::Vertical);
    }

    #[test]
    fn test_nes2_is_not_supported() {
        let test_rom = create_rom(TestRom {
            header: vec![
                0x4E, 0x45, 0x53, 0x1A, 0x01, 0x01, 0x31, 0x8, 00, 00, 00, 00, 00, 00, 00, 00,
            ],
            trainer: None,
            pgp_rom: vec![1; PRG_ROM_PAGE_SIZE],
            chr_rom: vec![2; CHR_ROM_PAGE_SIZE],
        });
        let rom = Rom::new(&test_rom);
        match rom {
            Result::Ok(_) => unreachable!(),
            Result::Err(err) => assert_eq!(err, Error::UnsupportedFormat),
        }
    }
}
