use bitflags::bitflags;

bitflags! {
    /// Status register flags.
    ///
    /// ```none
    ///  7 6 5 4 3 2 1 0
    ///  N V _ B D I Z C
    ///  | |   | | | | +--- Carry Flag
    ///  | |   | | | +----- Zero Flag
    ///  | |   | | +------- Interrupt Disable
    ///  | |   | +--------- Decimal Mode (not used on NES)
    ///  | |   +----------- Break Command
    ///  | +--------------- Overflow Flag
    ///  +----------------- Negative Flag
    /// ```
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Status: u8 {
        /// Carry flag.
        const CARRY             = 0b0000_0001;
        /// Zero flag.
        const ZERO              = 0b0000_0010;
        /// Interrupt disable flag.
        const INTERRUPT_DISABLE = 0b0000_0100;
        /// Decimal mode flag.
        const DECIMAL_MODE      = 0b0000_1000;
        /// Break flag.
        const BREAK             = 0b0001_0000;
        /// Unused flag.
        const BREAK2            = 0b0010_0000;
        /// Overflow flag.
        const OVERFLOW          = 0b0100_0000;
        /// Negative flag.
        const NEGATIVE          = 0b1000_0000;
    }
}

impl Default for Status {
    fn default() -> Self {
        // Self::INTERRUPT_DISABLE | Self::UNUSED
        Self::empty()
    }
}
