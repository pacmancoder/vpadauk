use vpadauk::{
    isa::pdk13::ir::{IrSlot, IrOpcode},
    mcu::{
        pms150c::{Pms150c, Emulator, pins},
        host_adapter::{HostAdapter, Pin, AnalogSignal},
    }
};

struct BlinkHostAdapter {
    current_cycle : u64,
}


impl BlinkHostAdapter {
    fn new() -> Self {
        Self {
            current_cycle: 0,
        }
    }
}

impl HostAdapter for BlinkHostAdapter {
    fn read_pin_digital(&self, pin: Pin) -> bool { false }

    fn write_pin_digital(&mut self, pin: Pin, value: bool) {
        println!("Pin {:?} changed to value {}", pin, value);
    }
    fn read_pin_analog(&self, pin: Pin) -> AnalogSignal { AnalogSignal::from_u16(0) }
    fn write_pin_analog(&mut self, pin: Pin, value: AnalogSignal) {}
    fn set_pin_output_enabled(&mut self, pin: Pin, enabled: bool) {}
    fn set_pin_pull_up_enabled(&mut self, pin: Pin, enabled: bool) {}
}

fn main() {
    let mut mcu  = Pms150c::new();

    let firmware = include_bytes!("blink.fw");
    for (address, word) in firmware.chunks(2).enumerate() {
        if word.len() != 2 {
            panic!("Invalid firmware");
        }
        let instruction = (word[0] as u16) | ((word[1] as u16) << 8);
        let ir = IrSlot::from_instruction(instruction);
        println!(
            "[{:#04X}] Opcode: {:?}; Addr/immediate operand: {}, Bit operand: {}, Rom address operand: {}",
            address,
            ir.ir_opcode(),
            ir.mem_address(),
            ir.bit_index(),
            ir.rom_address()
        );
        mcu.write_rom(address, instruction).unwrap();
    }
    let mut host = BlinkHostAdapter::new();

    // 4096 * 27 cycles should trigger pin change twice
    for i in 0..(4096 * 27) {
        mcu.step(&mut host);
    }
}