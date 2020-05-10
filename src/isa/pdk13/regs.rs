use super::{Byte, IoAddr};

// === Special IO Addresses ===
pub const IO_ADDR_FLAGS: IoAddr = 0x00;
pub const IO_ADDR_SP: IoAddr = 0x02;
pub const IO_ADDR_CLKMD: IoAddr = 0x03;
pub const IO_ADDR_INTEN: IoAddr = 0x04;
pub const IO_ADDR_INTRQ: IoAddr = 0x05;
pub const IO_ADDR_T16M: IoAddr = 0x06;
pub const IO_ADDR_TM2B: IoAddr = 0x09;
pub const IO_ADDR_EOSCR: IoAddr = 0x0A;
pub const IO_ADDR_INTEGS: IoAddr = 0x0C;
pub const IO_ADDR_PADIER: IoAddr = 0x0D;
pub const IO_ADDR_PA: IoAddr = 0x10;
pub const IO_ADDR_PAC: IoAddr = 0x11;
pub const IO_ADDR_PAPH: IoAddr = 0x12;
pub const IO_ADDR_TM2S: IoAddr = 0x17;
pub const IO_ADDR_GPCC: IoAddr = 0x1A;
pub const IO_ADDR_MISC: IoAddr = 0x1B;
pub const IO_ADDR_TM2C: IoAddr = 0x1C;
pub const IO_ADDR_TM2CT: IoAddr = 0x1D;
pub const IO_ADDR_GPCS: IoAddr = 0x1E;


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
pub const FLAGS_ARITH_MASK: Byte =
    FLAG_ZERO_MASK | FLAG_CARRY_MASK | FLAG_AUX_CARRY_MASK | FLAG_OVERFLOW_MASK;
