use crate::isa::pdk13::bus::Bus;
use crate::isa::pdk13::ir::{IrOpcode, IrSlot, IrSlotBuilder};

struct MockBus {
    pub io: [u8; 0x20],       // 32 bytes io space
    pub ram: [u8; 0x40],      // 64 bytes ram space
    pub rom: [IrSlot; 0x400], // 1K word rom space
    pub tim16: u16,
    pub reset_active: bool,
    pub wdt_reset_active: bool,
    pub stop_sys_active: bool,
    pub stop_exe_active: bool,
}

impl MockBus {
    pub fn new() -> Self {
        Self {
            io: [0; 0x20],
            ram: [0; 0x40],
            rom: [IrSlot::default(); 0x400],
            tim16: 0,
            reset_active: false,
            wdt_reset_active: false,
            stop_sys_active: false,
            stop_exe_active: false,
        }
    }
}

impl Bus for MockBus {
    fn write_io(&mut self, addr: u8, value: u8) {
        self.io[(addr & 0x1F) as usize] = value;
    }

    fn read_io(&self, addr: u8) -> u8 {
        self.io[(addr & 0x1F) as usize]
    }

    fn write_ram(&mut self, addr: u8, value: u8) {
        self.ram[(addr & 0x3F) as usize] = value;
    }

    fn read_ram(&self, addr: u8) -> u8 {
        self.ram[(addr & 0x3F) as usize]
    }

    fn read_rom(&self, addr: u16) -> IrSlot {
        self.rom[(addr & 0x3FF) as usize]
    }

    fn write_tim16(&mut self, value: u16) {
        self.tim16 = value;
    }

    fn read_tim16(&self) -> u16 {
        self.tim16
    }

    fn reset(&mut self) {
        self.reset_active = true;
    }

    fn stop_exe(&mut self) {
        self.stop_exe_active = true;
    }

    fn stop_sys(&mut self) {
        self.stop_sys_active = true;
    }

    fn wdt_reset(&mut self) {
        self.wdt_reset_active = true;
    }
}
