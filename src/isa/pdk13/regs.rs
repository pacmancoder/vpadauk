use super::{Byte, IoAddr};

// === Special IO Addresses ===
pub const FLAGS_IO_ADDR: IoAddr = 0x00;
pub const SP_IO_ADDR: IoAddr = 0x02;
pub const INTEN_IO_ADDR: IoAddr = 0x04;


// === FLAGS ===
pub const FLAG_ZERO_MASK: Byte = 0x01;
pub const FLAG_ZERO_OFFSET: Byte = 0x00;
pub const FLAG_CARRY_MASK: Byte = 0x02;
pub const FLAG_CARRY_OFFSET: Byte = 0x01;
pub const FLAG_AUX_CARRY_MASK: Byte = 0x04;
pub const FLAG_AUX_CARRY_OFFSET: Byte = 0x02;
pub const FLAG_OVERFLOW_MASK: Byte = 0x08;
pub const FLAG_OVERFLOW_OFFSET: Byte = 0x03;

// === FLAG GROUPS ===
pub const FLAGS_ARITH_MASK: Byte = FLAG_ZERO_MASK
    | FLAG_CARRY_MASK
    | FLAG_AUX_CARRY_MASK
    | FLAG_OVERFLOW_MASK;