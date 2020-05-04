mod opcode_stamp;
mod ir;
mod translation;
mod limit;
mod pdk_core;
mod ops;

use core::{
    ops::Deref,
    fmt::{Display, Formatter, Binary, LowerHex},
};

use failure::Fail;

use self::ir::{IrOpcode, IrSlot};
use self::opcode_stamp::OpcodeStamp;

#[derive(Debug, Fail)]
pub enum Pdk13Error {
    #[fail(display = "Provided work is too big for pdk13 word; Max: 8191, actual: {}", _0)]
    TooBigWord(Word),
    #[fail(display = "Unknown instruction: {:#b}", _0)]
    UnknownInstruction(Word),
}

pub type Pdk13Result<T> = Result<T, Pdk13Error>;

pub type Word = u16;
pub type Byte = u8;
pub type IoAddr = u8;
pub type NearRamAddr = u8;
pub type FarRamAddr = u16;
pub type RomAddr = u16;