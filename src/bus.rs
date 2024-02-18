use std::{ops::RangeInclusive, ptr::NonNull};

use crate::rom::Rom;

#[derive(Clone)]
pub struct Bus<'rom> {
    pub vram: [u8; 2048],
    pub rom: Rom<'rom>,
}

impl<'rom> Bus<'rom> {
    pub const RAM_RANGE: RangeInclusive<u16> = (0x0000..=0x1FFF);
    pub const ROM_RANGE: RangeInclusive<u16> = (0x8000..=0xFFFF);
    pub const PPU_REGISTER_RANGE: RangeInclusive<u16> = (0x2000..=0x3FFF);

    #[must_use]
    pub const fn new(rom: Rom<'rom>) -> Self {
        Self {
            vram: [0; 2048],
            rom,
        }
    }

    #[must_use]
    pub fn mirror(&self, addr: u16) -> Option<&u8> {
        // SAFETY: all ptrs come from valid references.
        unsafe { Some(&*self.mirror_addr(addr)?.as_ptr()) }
    }

    pub fn mirror_mut(&mut self, addr: u16) -> Option<&mut u8> {
        // SAFETY: we have a unique ref to self
        unsafe { Some(&mut *self.mirror_addr(addr)?.as_ptr()) }
    }

    fn mirror_addr(&self, addr: u16) -> Option<NonNull<u8>> {
        if Self::RAM_RANGE.contains(&addr) {
            let mirror_down_addr = addr & 0b0000_0111_1111_1111;
            self.vram.get(mirror_down_addr as usize).map(NonNull::from)
        } else if Self::ROM_RANGE.contains(&addr) {
            let mirror_down_addr = {
                let addr = addr - Self::ROM_RANGE.start();
                if self.rom.prg_rom.len() == 0x4000 && addr >= 0x4000 {
                    addr % 0x4000 // mirror, if needed
                } else {
                    addr
                }
            };
            self.rom
                .prg_rom
                .get(mirror_down_addr as usize)
                .map(NonNull::from)
        } else if Self::PPU_REGISTER_RANGE.contains(&addr) {
            // let _mirror_down_addr = addr & 0b0010_0000_0000_0111;
            todo!("PPU is not supported yet")
        } else {
            None
        }
    }

    /// Reads a byte from memory.
    /// # WARNING
    ///
    /// This does not increment the program counter; use [`Cpu::take`](crate::cpu::Cpu::take) for that.
    #[must_use]
    pub fn mem_read(&self, addr: u16) -> u8 {
        if let Some(&val) = self.mirror(addr) {
            val
        } else {
            log::warn!("ignoring memory read at: {addr:#02x}");
            0
        }
    }

    /// Writes a byte to memory.
    pub fn mem_write(&mut self, addr: u16, val: u8) {
        if Self::ROM_RANGE.contains(&addr) {
            log::warn!("attempt to write to cartridge ROM: {addr:#02x}");
            return;
        }

        if let Some(v) = self.mirror_mut(addr) {
            *v = val;
        } else {
            log::warn!("ignoring memory write at: {addr:#02x}");
        }
    }

    #[must_use]
    /// Reads a little-endian, 16-bit number from memory.
    ///
    /// # WARNING
    ///
    /// This does not increment the program counter; use [`Cpu::take_u16`](crate::cpu::Cpu::take_u16) for that.
    pub fn mem_read_u16(&self, addr: u16) -> u16 {
        let lo = self.mem_read(addr);
        let hi = self.mem_read(addr + 1);
        u16::from_le_bytes([lo, hi])
    }

    /// Writes a little-endian, 16-bit number to memory.
    pub fn mem_write_u16(&mut self, addr: u16, val: u16) {
        let [lo, hi] = val.to_le_bytes();
        self.mem_write(addr, lo);
        self.mem_write(addr + 1, hi);
    }
}
