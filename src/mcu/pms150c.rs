use core::cell::Cell;

use crate::{
    isa::pdk13::*,
    mcu::host_adapter::HostAdapter,
};

const IO_SPACE_SIZE: usize = 0x20;   // 32 bytes;
const RAM_SPACE_SIZE: usize = 0x40;  // 64 bytes;
const ROM_SPACE_SIZE: usize = 0x400; // 1024 bytes;

const IO_ADDRESS_MASK: IoAddr = IO_SPACE_SIZE as IoAddr - 1;
const RAM_ADDRESS_MASK: RamAddr = RAM_SPACE_SIZE as RamAddr - 1;
const ROM_ADDRESS_MASK: RomAddr = ROM_SPACE_SIZE as RomAddr - 1;

const IHRC_FREQUENCY: u32 = 16_000_000; // 16 MHz
const ILRC_FREQUENCY: u32 = 62_000;     // 62 KHz

mod pins {
    use crate::mcu::host_adapter::Pin;

    pub const PA0: Pin = Pin(0b00000001);
    pub const PA3: Pin = Pin(0b00001000);
    pub const PA4: Pin = Pin(0b00010000);
    pub const PA5: Pin = Pin(0b00100000);
    pub const PA6: Pin = Pin(0b01000000);
    pub const PA7: Pin = Pin(0b10000000);

    pub(crate) const ALL_PINS: [Pin; 6] = [PA0, PA3, PA4, PA5, PA6, PA7];
}

pub struct Pms150c {
    core: PdkCore,
    state: State,
}

trait Emulator {
    /// Returns current frequency to emulate. Please check before each stepping process
    /// to perform correct step count
    fn get_frequency(&self) -> u32;
    /// Performs emulation step
    fn step(&mut self, host: &mut dyn HostAdapter);
    /// Resets internal emulator state and adjusts host state
    fn init(&mut self, host: &mut dyn HostAdapter);
    /// Write rom memory to specified address
    fn write_rom(&mut self, address: usize, value: Word) -> Pdk13Result<()>;
}

struct State {
    io: [Byte; IO_SPACE_SIZE],
    ram: [Byte; RAM_SPACE_SIZE],
    rom: [IrSlot; ROM_SPACE_SIZE],
    clock_frequency: u32,
    pa: Cell<Byte>,
    pac: Byte,
    paph: Byte,
}

impl State {
    fn new() -> Self {
        Self {
            io: [0; IO_SPACE_SIZE],
            ram: [0; RAM_SPACE_SIZE],
            rom: [IrSlot::default(); ROM_SPACE_SIZE],
            clock_frequency: ILRC_FREQUENCY,
            pa: Cell::new(0),
            pac: 0,
            paph: 0,
        }
    }

    fn reset(&mut self) {
        self.io[regs::IO_ADDR_CLKMD as usize] = 0b11110110;
        self.io[regs::IO_ADDR_PADIER as usize] = 0b11111001;
        self.clock_frequency = ILRC_FREQUENCY;
        self.pa.set(0);
        self.pac = 0;
        self.paph = 0;
    }
}

impl Pms150c {
    pub fn new() -> Self {
        Self {
            core: PdkCore::new(),
            state: State::new(),
        }
    }
}

impl Emulator for Pms150c {
    fn get_frequency(&self) -> u32 {
        self.state.clock_frequency
    }

    fn step(&mut self, host: &mut dyn HostAdapter) {
        self.core.step(&mut HostBridge::new(&mut self.state, host));
    }

    fn init(&mut self, host: &mut dyn HostAdapter) {
        for pin in pins::ALL_PINS.iter().copied() {
            host.set_pin_output_enabled(pin, false);
            host.set_pin_pull_up_enabled(pin, false);
            host.write_pin_digital(pin, false);
        }

        self.state.reset();
    }

    fn write_rom(&mut self, address: usize, value: Word) -> Pdk13Result<()> {
        if address >= ROM_SPACE_SIZE {
            return Err(Pdk13Error::TooBigAddress(address, ROM_SPACE_SIZE));
        }
        self.state.rom[address] = IrSlot::from_instruction(value)?;
        Ok(())
    }
}

struct HostBridge<'a> {
    state: &'a mut State,
    host: &'a mut dyn HostAdapter,
}

fn decode_sys_freq(clkmd: Byte) -> u32 {
    let freq_flags = {
        let freq_flags_lo = (clkmd >> 5) & 0x07;
        let freq_flags_hi = (clkmd >> 3) & 0x01;
        freq_flags_lo | (freq_flags_hi << 3)
    };
    let ihrc_enabled = (clkmd & 0b00010000) != 0;
    let ilrc_enabled = (clkmd & 0b00000100) != 0;

    if (matches!(freq_flags, 0b0000 | 0b0001 | 0b1000 | 0b1001 | 0b1011 | 0b1100) & !ihrc_enabled)
        | (matches!(freq_flags, 0b0110 | 0b0111 | 0b1010) & !ilrc_enabled)
    {
        // User code stopped clocking
        return 0;
    }

    match freq_flags {
        0b0000 => IHRC_FREQUENCY  / 4,
        0b0001 => IHRC_FREQUENCY / 2,
        0b0010 ..= 0b0101 => 0,
        0b0110 => ILRC_FREQUENCY / 4,
        0b0111 => ILRC_FREQUENCY,
        0b1000 => IHRC_FREQUENCY / 16,
        0b1001 => IHRC_FREQUENCY / 8,
        0b1010 => ILRC_FREQUENCY / 16,
        0b1011 => IHRC_FREQUENCY / 32,
        0b1100 => IHRC_FREQUENCY / 64,
        0b1101 ..= 0b1111 => 0,
        0b10000 ..= 0xFF => unreachable!(),
    }
}

impl<'a> HostBridge<'a> {
    pub fn new(state: &'a mut State, host: &'a mut dyn HostAdapter) -> Self {
        Self { state, host }
    }

    fn on_change_clkmd(&mut self, clkmd: Byte) {
        self.state.clock_frequency = decode_sys_freq(clkmd);
        self.set_watchdog_enabled(clkmd & 0b00000010 != 0);
        self.set_pa5_reset_enabled(clkmd & 0b00000001 != 0);
    }

    fn set_watchdog_enabled(&mut self, _enabled: bool) {}
    fn set_pa5_reset_enabled(&mut self, _enabled: bool) {}

    fn on_change_pa(&mut self, pa: Byte) {
        self.host.write_pin_digital(pins::PA7, (pa & pins::PA7.port_bit_mask()) != 0);
        self.host.write_pin_digital(pins::PA6, (pa & pins::PA6.port_bit_mask()) != 0);
        self.host.write_pin_digital(pins::PA5, (pa & pins::PA5.port_bit_mask()) != 0);
        self.host.write_pin_digital(pins::PA4, (pa & pins::PA4.port_bit_mask()) != 0);
        self.host.write_pin_digital(pins::PA3, (pa & pins::PA3.port_bit_mask()) != 0);
        self.host.write_pin_digital(pins::PA0, (pa & pins::PA0.port_bit_mask()) != 0);

        self.state.pa.set(pa);
    }

    fn on_change_pac(&mut self, pac: Byte) {
        self.host.set_pin_output_enabled(pins::PA7, (pac & pins::PA7.port_bit_mask()) != 0);
        self.host.set_pin_output_enabled(pins::PA6, (pac & pins::PA6.port_bit_mask()) != 0);
        self.host.set_pin_output_enabled(pins::PA5, (pac & pins::PA5.port_bit_mask()) != 0);
        self.host.set_pin_output_enabled(pins::PA4, (pac & pins::PA4.port_bit_mask()) != 0);
        self.host.set_pin_output_enabled(pins::PA3, (pac & pins::PA3.port_bit_mask()) != 0);
        self.host.set_pin_output_enabled(pins::PA0, (pac & pins::PA0.port_bit_mask()) != 0);

        self.state.pac = pac;
    }

    fn on_change_paph(&mut self, paph: Byte) {
        self.host.set_pin_pull_up_enabled(pins::PA7, (paph & pins::PA7.port_bit_mask()) != 0);
        self.host.set_pin_pull_up_enabled(pins::PA6, (paph & pins::PA6.port_bit_mask()) != 0);
        self.host.set_pin_pull_up_enabled(pins::PA5, (paph & pins::PA5.port_bit_mask()) != 0);
        self.host.set_pin_pull_up_enabled(pins::PA4, (paph & pins::PA4.port_bit_mask()) != 0);
        self.host.set_pin_pull_up_enabled(pins::PA3, (paph & pins::PA3.port_bit_mask()) != 0);
        self.host.set_pin_pull_up_enabled(pins::PA0, (paph & pins::PA0.port_bit_mask()) != 0);

        self.state.paph = paph;
    }

    fn on_read_pa(&self) {
        // clear all input pins
        let mut new_pa = self.state.pa.get() & self.state.pac;
        // Update pins state from host
        if self.host.read_pin_digital(pins::PA7) {
            new_pa |= pins::PA7.port_bit_mask();
        };
        if self.host.read_pin_digital(pins::PA6) {
            new_pa |= pins::PA6.port_bit_mask();
        };
        if self.host.read_pin_digital(pins::PA5) {
            new_pa |= pins::PA5.port_bit_mask();
        };
        if self.host.read_pin_digital(pins::PA4) {
            new_pa |= pins::PA4.port_bit_mask();
        };
        if self.host.read_pin_digital(pins::PA3) {
            new_pa |= pins::PA3.port_bit_mask();
        };
        if self.host.read_pin_digital(pins::PA0) {
            new_pa |= pins::PA0.port_bit_mask();
        };

        self.state.pa.set(new_pa);
    }
}

impl<'a> Bus for HostBridge<'a> {
    fn write_io(&mut self, addr: IoAddr, value: Byte) {
        use regs::*;

        self.state.io[(addr & IO_ADDRESS_MASK) as usize] = value;
        match addr & IO_ADDRESS_MASK {
            IO_ADDR_FLAGS => {}
            0x01 => {}
            IO_ADDR_SP => {},
            IO_ADDR_CLKMD => self.on_change_clkmd(value),
            IO_ADDR_INTEN => {},
            IO_ADDR_INTRQ => {},
            IO_ADDR_T16M => {},
            0x07 => {},
            0x08 => {},
            IO_ADDR_TM2B => {},
            IO_ADDR_EOSCR => {},
            0x0B => {},
            IO_ADDR_INTEGS => {},
            IO_ADDR_PADIER => {},
            0x0E => {},
            0x0F => {},
            IO_ADDR_PA => self.on_change_pa(value),
            IO_ADDR_PAC => self.on_change_pac(value),
            IO_ADDR_PAPH => self.on_change_paph(value),
            0x13 => {},
            0x14 => {},
            0x15 => {},
            0x16 => {},
            IO_ADDR_TM2S => {},
            0x18 => {},
            0x19 => {},
            IO_ADDR_GPCC => {},
            IO_ADDR_MISC => {},
            IO_ADDR_TM2C => {},
            IO_ADDR_TM2CT => {},
            IO_ADDR_GPCS => {},
            0x1F => {},
            // Unreachable io addresses
            0x20 ..= 0xFF => unreachable!(),
        }
    }

    fn read_io(&self, addr: u8) -> u8 {
        if addr == regs::IO_ADDR_PA {
            self.on_read_pa();
        }

        self.state.io[(addr & IO_ADDRESS_MASK) as usize]
    }

    fn write_ram(&mut self, addr: RamAddr, value: Byte) {
        self.state.ram[(addr & RAM_ADDRESS_MASK) as usize] = value;
    }

    fn read_ram(&self, addr: RamAddr) -> Byte {
        self.state.ram[(addr & RAM_ADDRESS_MASK) as usize]
    }

    fn read_rom(&self, addr: RomAddr) -> IrSlot {
        self.state.rom[(addr & ROM_ADDRESS_MASK) as usize]
    }

    fn write_tim16(&mut self, value: u16) {}

    fn read_tim16(&self) -> u16 { 0 }

    fn reset(&mut self) {
        self.state.reset();
    }

    fn stop_exe(&mut self) {}

    fn stop_sys(&mut self) {}

    fn wdt_reset(&mut self) {}
}
