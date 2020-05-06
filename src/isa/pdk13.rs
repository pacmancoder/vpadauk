mod bus;
mod ir;
mod limit;
mod opcode_stamp;
mod ops;
mod pdk_core;
mod regs;

#[cfg(test)]
mod test;

use failure::Fail;

#[derive(Debug, Fail)]
pub enum Pdk13Error {
    #[fail(
        display = "Provided work is too big for pdk13 word; Max: 8191, actual: {}",
        _0
    )]
    TooBigWord(Word),
    #[fail(display = "Unknown instruction: {:#b}", _0)]
    UnknownInstruction(Word),
}

pub type Pdk13Result<T> = Result<T, Pdk13Error>;

pub type Word = u16;
pub type Byte = u8;
pub type IoAddr = u8;
pub type RamAddr = u8;
pub type RomAddr = u16;
