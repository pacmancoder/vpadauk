#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pin(pub(crate) usize);

impl Pin {
    pub(crate) fn from_port_bit_mask(self, value: u8) -> Self {
        Self(value as usize)
    }
    pub(crate) fn port_bit_mask(self) -> u8 {
        self.0 as u8
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnalogSignal(u16);

impl AnalogSignal {
    #[cfg(float_arithmetic)]
    const fn from_float(value: f32) -> Self {
        if value <= 0f32 {
            Self(0)
        } else if value >= 1f32 {
            Self(f32::MAX)
        } else {
            Self((value * u16::MAX as f32) as u16)
        }
    }

    #[cfg(float_arithmetic)]
    const fn as_float(self) -> f32 {
        (self.0 as f32) / (u16::MAX as f32)
    }

    const fn from_u16(value: u16) -> Self {
        Self(value)
    }

    const fn as_u16(self) -> u16 {
        self.0
    }
}

pub trait HostAdapter {
    fn read_pin_digital(&self, pin: Pin) -> bool;
    fn write_pin_digital(&mut self, pin: Pin, value: bool);

    fn read_pin_analog(&self, pin: Pin) -> AnalogSignal;
    fn write_pin_analog(&mut self, pin: Pin, value: AnalogSignal);

    fn set_pin_output_enabled(&mut self, pin: Pin, enabled: bool);
    fn set_pin_pull_up_enabled(&mut self, pin: Pin, enabled: bool);
}
