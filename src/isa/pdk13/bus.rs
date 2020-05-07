use super::{
    ir::IrSlot,
    regs::{FLAGS_IO_ADDR, SP_IO_ADDR},
    Byte, IoAddr, RamAddr, RomAddr, Word,
};
use crate::isa::pdk13::regs::{
    FLAG_AUX_CARRY_MASK, FLAG_AUX_CARRY_OFFSET, FLAG_CARRY_MASK, FLAG_CARRY_OFFSET,
    FLAG_OVERFLOW_MASK, FLAG_OVERFLOW_OFFSET, FLAG_ZERO_MASK, FLAG_ZERO_OFFSET,
};

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

    fn read_sp(&self) -> RamAddr;
    fn write_sp(&mut self, addr: RamAddr);

    fn read_flags(&self) -> Byte;
    fn write_flags(&mut self, flags: Byte);

    fn is_zero_flag(&self) -> bool;
    fn is_carry_flag(&self) -> bool;
    fn is_aux_carry_flag(&self) -> bool;
    fn is_overflow_flag(&self) -> bool;

    fn set_zero_flag(&mut self, value: bool);
    fn set_carry_flag(&mut self, value: bool);
    fn set_aux_carry_flag(&mut self, value: bool);
    fn set_overflow_flag(&mut self, value: bool);
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

    fn read_sp(&self) -> RamAddr {
        self.read_io(SP_IO_ADDR)
    }

    fn write_sp(&mut self, addr: u8) {
        self.write_io(SP_IO_ADDR, addr)
    }

    fn read_flags(&self) -> Byte {
        self.read_io(FLAGS_IO_ADDR)
    }

    fn write_flags(&mut self, flags: u8) {
        self.write_io(FLAGS_IO_ADDR, flags)
    }

    fn is_zero_flag(&self) -> bool {
        self.read_flags() & FLAG_ZERO_MASK != 0
    }

    fn is_carry_flag(&self) -> bool {
        self.read_flags() & FLAG_CARRY_MASK != 0
    }

    fn is_aux_carry_flag(&self) -> bool {
        self.read_flags() & FLAG_AUX_CARRY_MASK != 0
    }

    fn is_overflow_flag(&self) -> bool {
        self.read_flags() & FLAG_OVERFLOW_MASK != 0
    }

    fn set_zero_flag(&mut self, value: bool) {
        self.write_flags(
            (self.read_flags() & !FLAG_ZERO_MASK) | ((value as u8) << FLAG_ZERO_OFFSET),
        );
    }

    fn set_carry_flag(&mut self, value: bool) {
        self.write_flags(
            (self.read_flags() & !FLAG_CARRY_MASK) | ((value as u8) << FLAG_CARRY_OFFSET),
        );
    }

    fn set_aux_carry_flag(&mut self, value: bool) {
        self.write_flags(
            (self.read_flags() & !FLAG_AUX_CARRY_MASK) | ((value as u8) << FLAG_AUX_CARRY_OFFSET),
        );
    }

    fn set_overflow_flag(&mut self, value: bool) {
        self.write_flags(
            (self.read_flags() & !FLAG_OVERFLOW_MASK) | ((value as u8) << FLAG_OVERFLOW_OFFSET),
        );
    }
}
