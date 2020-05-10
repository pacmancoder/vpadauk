pub trait HostAdapter {
    fn read_pin_digital(pin: usize) -> bool;
    fn write_pin_digital(pin: usize, value: bool);

    fn read_pin_analog(pin: usize) -> u16;
    fn write_pin_analog(pin: usize, value: u16);

    fn set_pin_output_enabled(pin: usize, enabled: bool);
    fn set_pin_pull_up_enabled(pin: usize, enabled: bool);
}
