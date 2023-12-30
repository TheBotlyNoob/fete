use std::ptr::NonNull;

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

#[derive(Clone)]
pub struct Bus {
    pub vram: [u8; 2048],
}

impl Default for Bus {
    fn default() -> Self {
        Self { vram: [0; 2048] }
    }
}

impl Bus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mirror(&self, addr: u16) -> Option<&u8> {
        // SAFETY: all ptrs come from valid references.
        unsafe { Some(&*self.mirror_addr(addr)?.as_ptr()) }
    }

    pub fn mirror_mut(&mut self, addr: u16) -> Option<&mut u8> {
        // SAFETY: we have a unique ref to self
        unsafe { Some(&mut *self.mirror_addr(addr)?.as_ptr()) }
    }

    fn mirror_addr(&self, addr: u16) -> Option<NonNull<u8>> {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.vram.get(mirror_down_addr as usize).map(NonNull::from)
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0b00100000_00000111;
                todo!("PPU is not supported yet")
            }
            _ => None,
        }
    }

    #[must_use]
    /// Reads a byte from memory, _without_ incrementing the program counter.
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
        if let Some(v) = self.mirror_mut(addr) {
            *v = val;
        } else {
            panic!("ignoring memory write at: {addr:#02x}");
        }
    }

    #[must_use]
    /// Reads a little-endian, 16-bit number from memory, _without_ incrementing the program counter.
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
