use crate::isa::pdk13::pdk_core::Bus;
use crate::isa::pdk13::ir::IrSlot;

struct MockBus {

}

impl Bus for MockBus {
    fn write_io(&mut self, addr: u8, value: u8) {
        unimplemented!()
    }

    fn read_io(&self, addr: u8) -> u8 {
        unimplemented!()
    }

    fn write_ram(&mut self, addr: u8, value: u8) {
        unimplemented!()
    }

    fn read_ram(&self, addr: u8) -> u8 {
        unimplemented!()
    }

    fn read_rom(&self, addr: u16) -> u16 {
        unimplemented!()
    }

    fn read_ir(&self, addr: u16) -> IrSlot {
        unimplemented!()
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