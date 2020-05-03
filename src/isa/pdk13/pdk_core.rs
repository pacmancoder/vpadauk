use super::{IoAddr, Byte, RamAddr, RomAddr, Word};

const IO_SPACE_SIZE: usize = 32;

const FLAGS_IO_ADDR: IoAddr = 0x00;
const SP_IO_ADDR: IoAddr = 0x02;
const INTEN_IO_ADDR: IoAddr = 0x04;

const FLAG_ZERO_MASK: Byte = 0x01;
const FLAG_CARRY_MASK: Byte = 0x02;
const FLAG_AUX_CARRY_MASK: Byte = 0x04;
const FLAG_OVERFLOW_MASK: Byte = 0x08;

trait Bus {
    fn write_acc(addr: IoAddr, value: Byte);
    fn read_acc(addr: IoAddr) -> Byte;

    fn write_io(addr: IoAddr, value: Byte);
    fn read_io(addr: IoAddr) -> Byte;

    fn write_ram(addr: RamAddr, value: Byte);
    fn read_ram(addr: RamAddr) -> Byte;

    fn read_rom(addr: RomAddr) -> Word;

    fn write_tim16(value: Word);
    fn read_tim16() -> Word;

    fn reset();
    fn stopexe();
    fn stopsys();
    fn wdtreset();
}

struct PdkCore<B: Bus> {
    acc: Byte,
    pc: RomAddr,
    skip_cycles: usize,
    bus: B,
}

impl<B: Bus> PdkCore<B> {
    fn new(bus: B) -> Self {
        Self {
            acc: 0,
            pc: 0,
            skip_cycles: 0,
            bus
        }
    }

    fn bus(&mut self) -> &mut B {
        &mut self.bus
    }

    fn step(&mut self) {

    }
}