#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]

#[derive(Default)]
pub struct Cpu {
    pub reg_a: u8,
    pub status: u8,
    pub pc: u16,
}

impl Cpu {
    #[must_use = "does nothing without calling `.interpret()`"]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn interpret(&mut self, prog: &[u8]) {
        loop {
            let op = self.take(prog).expect("no op code");

            match op {
                _ => todo!("implement op code {:02x}", op),
            }
        }
    }

    fn take(&mut self, prog: &[u8]) -> Option<u8> {
        let &byte = prog.get(self.pc as usize)?;
        self.pc += 1;
        Some(byte)
    }
}
