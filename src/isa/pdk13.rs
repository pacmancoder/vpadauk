mod bus;
pub mod ir;
mod opcode_stamp;
mod ops;
mod pdk_core;

pub mod regs;

#[cfg(test)]
mod test;

use failure::Fail;

pub use bus::{Bus, BusExt};
pub use ir::{IrOpcode, IrSlot, IrSlotBuilder};
pub use pdk_core::PdkCore;

#[derive(Debug, Fail)]
pub enum Pdk13Error {
    #[fail(display = "Too big address: {}, address space size: {}", _0, _1)]
    TooBigAddress(usize, usize),
}

pub type Pdk13Result<T> = Result<T, Pdk13Error>;

pub type Word = u16;
pub type Byte = u8;
pub type IoAddr = u8;
pub type RamAddr = u8;
pub type RomAddr = u16;
