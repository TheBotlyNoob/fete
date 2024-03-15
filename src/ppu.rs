use crate::rom::Mirroring;

pub struct Ppu<'rom> {
    pub chr_rom: &'rom [u8],
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],

    pub mirroring: Mirroring,
    pub addr: AddrRegister,
}

impl<'rom> Ppu<'rom> {
    pub const fn new(chr_rom: &'rom [u8], mirroring: Mirroring) -> Self {
        Self {
            chr_rom,
            vram: [0; 2048],
            oam_data: [0; 256],
            palette_table: [0; 32],
            mirroring,
            addr: AddrRegister::new(),
        }
    }
    pub fn write_ppu_addr(&mut self, val: u8) {
        self.addr.update(val);
    }
}

pub struct AddrRegister {
    val: u16,
    hi: bool,
}

impl AddrRegister {
    pub const fn new() -> Self {
        Self { val: 0, hi: true }
    }

    pub fn update(&mut self, data: u8) {
        let [hi, lo] = self.val.to_le_bytes();
        self.val = u16::from_le_bytes(if self.hi { [data, lo] } else { [hi, data] }) & 0x3FFF; // mirror down; ppu memory doesn't go over 0x3FFF

        self.hi = !self.hi;
    }

    pub fn inc(&mut self, inc: u8) {
        let [hi, lo] = self.val.to_le_bytes();
        let new_lo = lo.wrapping_add(inc);
        let new_hi = if lo > new_lo { hi.wrapping_add(1) } else { hi };

        self.val = u16::from_le_bytes([new_hi, new_lo]) & 0x3FFF; // mirror down; ppu memory doesn't go over 0x3FFF
    }

    pub fn reset_latch(&mut self) {
        self.hi = true;
    }
}
