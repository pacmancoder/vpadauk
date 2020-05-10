use crate::isa::pdk13::*;

const IO_SPACE_SIZE: usize = 0x20; // 32 bytes;
const RAM_SPACE_SIZE: usize = 0x40; // 64 bytes;
const ROM_SPACE_SIZE: usize = 0x400; // 1024 bytes;

const IO_ADDRESS_MASK: IoAddr = IO_SPACE_SIZE as IoAddr - 1;
const RAM_ADDRESS_MASK: RamAddr = RAM_SPACE_SIZE as RamAddr - 1;
const ROM_ADDRESS_MASK: RomAddr = ROM_SPACE_SIZE as RomAddr - 1;

pub trait McuPeripherals {}

struct McuBus {
    io: [Byte; IO_SPACE_SIZE],
    ram: [Byte; RAM_SPACE_SIZE],
    rom: [IrSlot; ROM_SPACE_SIZE],
}

impl Bus for McuBus {
    fn write_io(&mut self, addr: u8, value: u8) {
        self.io[(addr & IO_ADDRESS_MASK) as usize] = value;
        // TODO: io writes handling
    }

    fn read_io(&self, addr: u8) -> u8 {
        self.io[(addr & IO_ADDRESS_MASK) as usize]
    }

    fn write_ram(&mut self, addr: RamAddr, value: Byte) {
        self.ram[(addr & RAM_ADDRESS_MASK) as usize] = value;
    }

    fn read_ram(&self, addr: RamAddr) -> Byte {
        self.ram[(addr & RAM_ADDRESS_MASK) as usize]
    }

    fn read_rom(&self, addr: RomAddr) -> IrSlot {
        self.rom[(addr & ROM_ADDRESS_MASK) as usize]
    }

    fn write_tim16(&mut self, value: u16) {
        unimplemented!()
    }

    fn read_tim16(&self) -> u16 {
        unimplemented!()
    }

    fn reset(&mut self) {
        unimplemented!()
    }

    fn stop_exe(&mut self) {
        unimplemented!()
    }

    fn stop_sys(&mut self) {
        unimplemented!()
    }

    fn wdt_reset(&mut self) {
        unimplemented!()
    }
}
