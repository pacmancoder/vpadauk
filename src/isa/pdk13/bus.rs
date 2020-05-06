use super::{ir::IrSlot, Byte, IoAddr, RamAddr, RomAddr, Word};

pub trait Bus {
    fn write_io(&mut self, addr: IoAddr, value: Byte);
    fn read_io(&self, addr: IoAddr) -> Byte;

    fn write_ram(&mut self, addr: RamAddr, value: Byte);
    fn read_ram(&self, addr: RamAddr) -> Byte;

    fn read_rom(&self, addr: RomAddr) -> IrSlot;

    fn write_tim16(&mut self, value: Word);
    fn read_tim16(&self) -> Word;

    fn reset(&mut self);
    fn stop_exe(&mut self);
    fn stop_sys(&mut self);
    fn wdt_reset(&mut self);
}

pub trait BusExt {
    fn read_ram_word(&self, addr: RamAddr) -> Word;
    fn write_ram_word(&mut self, addr: RamAddr, value: Word);
}

impl<T> BusExt for T
where
    T: Bus,
{
    fn read_ram_word(&self, addr: RamAddr) -> Word {
        let lo = self.read_ram(addr) as Word;
        let hi = self.read_ram(addr.wrapping_add(1)) as Word;
        lo | (hi << 8)
    }

    fn write_ram_word(&mut self, addr: RamAddr, value: Word) {
        self.write_ram(addr, value as u8);
        self.write_ram(addr.wrapping_add(1), (value >> 8) as u8);
    }
}
