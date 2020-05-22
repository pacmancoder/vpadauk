use std::{
    fs,
    path::Path,
    collections::LinkedList,
    time::{Duration, Instant}
};
use plotters::prelude::*;

use vpadauk::{
    isa::pdk13::Pdk13Error,
    mcu::{
        pms150c::{Pms150c, pins},
        host_adapter::{HostAdapter, Pin, AnalogSignal}
    }
};
use vpadauk::mcu::pms150c::Emulator;
use vpadauk::isa::pdk13::ir::IrOpcode::Incm;


pub enum EmulationError {
    InvalidRom(String),
    CoreFailed(Pdk13Error)
}

#[derive(Copy, Clone)]
pub enum EmulationAction {
    ChangePinInput(Pin, bool),
}

#[derive(Copy, Clone)]
pub struct TimedEmulationAction {
    duration: Duration,
    action: EmulationAction
}

impl TimedEmulationAction {
    pub fn new(when: Duration, what: EmulationAction) -> Self {
        Self {
            duration: when,
            action: what,
        }
    }
}

pub struct PlottingHostBuilder {
    plot_width: usize,
    plot_height: usize,
    plot_path:  String,
}

impl PlottingHostBuilder {
    pub fn new() -> Self {
        Self {
            plot_width: 1024,
            plot_height: 1024,
            plot_path: "plot.svg".into(),
        }
    }

    pub fn plot_width(mut self, width: usize) -> Self {
        self.plot_width = width;
        self

    }

    pub fn plot_height(mut self, height: usize) -> Self {
        self.plot_height = height;
        self
    }

    pub fn plot_path(mut self, path: &str) -> Self {
        self.plot_path =  path.into();
        self
    }

    pub fn build(self) -> PlottingHost {
        PlottingHost {
            plot_width: self.plot_width,
            plot_height: self.plot_height,
            plot_path: self.plot_path,
            actions: LinkedList::new(),
            pa: HostPortA::default(),
            plot_digital_pins: Vec::new(),
        }
    }
}

#[derive(Default)]
struct HostPortA {
    pin7: bool,
    pin6: bool,
    pin5: bool,
    pin4: bool,
    pin3: bool,
    pin0: bool,
}

pub struct PlottingHost {
    plot_width: usize,
    plot_height: usize,
    plot_path:  String,
    actions: LinkedList<TimedEmulationAction>,
    pa: HostPortA,
    plot_digital_pins: Vec<(f64, [bool; 6])>,
}

impl PlottingHost {
    fn load_rom(path: &Path, mcu: &mut Pms150c) -> Result<(), EmulationError> {
        let rom = fs::read(path)
            .map_err(|_| EmulationError::InvalidRom("Can't open file".into()))?;

        if rom.len() % 2 != 0 {
            return Err(EmulationError::InvalidRom("Rom size should be multiple of 2".into()));
        }

        for (address, word) in rom.chunks(2).enumerate() {
            let instruction = (word[0] as u16) | ((word[1] as u16) << 8);
            mcu.write_rom(address, instruction).map_err(|err| EmulationError::CoreFailed(err))?;
        };

        Ok(())
    }

    pub fn run(&mut self, rom_path: &Path, time: Duration) -> Result<(), EmulationError> {
        self.plot_digital_pins.clear();
        self.pa = HostPortA::default();
        let mut actions = self.actions.clone();


        let mut mcu = Pms150c::new();
        Self::load_rom(rom_path, &mut mcu)?;

        let mut time_passed = Duration::from_nanos(0);

        let mut steps = 0;

        loop {
            if !actions.is_empty() {
                if time_passed >= actions.front().unwrap().duration {
                    match actions.front().unwrap().action {
                        EmulationAction::ChangePinInput(pin, value) => {
                            match pin {
                                pins::PA7 => self.pa.pin7 = value,
                                pins::PA6 => self.pa.pin6 = value,
                                pins::PA5 => self.pa.pin5 = value,
                                pins::PA4 => self.pa.pin4 = value,
                                pins::PA3 => self.pa.pin3 = value,
                                pins::PA0 => self.pa.pin0 = value,
                                _ => unreachable!(),
                            }
                        },
                    }
                    actions.pop_front();
                }
            }

            let plot_state = [
                self.pa.pin0,
                self.pa.pin3,
                self.pa.pin4,
                self.pa.pin5,
                self.pa.pin6,
                self.pa.pin7,
            ];
            self.plot_digital_pins.push(
                ((time_passed.as_nanos() as f64) / 1000000000 as f64, // in seconds
                 plot_state));

            let step_frequency = mcu.get_frequency();
            mcu.step(self);
            let step_period_ns = 1_000_000_000_f64 / step_frequency as f64;
            time_passed += Duration::from_nanos(step_period_ns as u64);
            steps += 1;

            if time_passed > time {
                break;
            }
        }

        // plotting

        let passed_nanos = (time_passed.as_nanos() as f64) / 1000000000 as f64;

        let plot_path = self.plot_path.clone();
        let mut root_area =
            BitMapBackend::new(&plot_path, (self.plot_width as u32, self.plot_height as u32))
                .into_drawing_area();

        let h1_font = ("sans-serif", 60).into_font();
        let h2_font = ("sans-serif", 20).into_font();

        root_area.titled("PMS150C emulation result", h1_font);
        root_area.fill(&WHITE).unwrap();

        let plots = root_area.split_evenly((6, 1));
        plots[0].titled("PA0", h2_font.clone());
        plots[1].titled("PA3", h2_font.clone());
        plots[2].titled("PA4", h2_font.clone());
        plots[3].titled("PA5", h2_font.clone());
        plots[4].titled("PA6", h2_font.clone());
        plots[5].titled("PA7", h2_font.clone());

        // PA0
        {
            let mut cc = ChartBuilder::on(&plots[0])
                .margin(5)
                .set_all_label_area_size(20)
                .build_ranged(-0f64..passed_nanos, 0usize..1usize).unwrap();

            cc.configure_mesh()
                .x_labels(10)
                .y_labels(2)
                .disable_mesh()
                .x_label_formatter(&|v| format!("{}", *v as u64))
                .y_label_formatter(&|v| format!("{}", v))
                .draw().unwrap();


            cc.draw_series(LineSeries::new(
                self.plot_digital_pins.iter().map(|record| (record.0, record.1[0] as usize)),
                &RED,
            )).unwrap().label("PA0");
        }

        // PA3
        {
            let mut cc = ChartBuilder::on(&plots[1])
                .margin(5)
                .set_all_label_area_size(20)
                .build_ranged(-0f64..passed_nanos, 0usize..1usize).unwrap();

            cc.configure_mesh()
                .x_labels(10)
                .y_labels(2)
                .disable_mesh()
                .x_label_formatter(&|v| format!("{}", *v as u64))
                .y_label_formatter(&|v| format!("{}", v))
                .draw().unwrap();


            cc.draw_series(LineSeries::new(
                self.plot_digital_pins.iter().map(|record| (record.0, record.1[1] as usize)),
                &RED,
            )).unwrap().label("PA3");
        }

        // PA4
        {
            let mut cc = ChartBuilder::on(&plots[2])
                .margin(5)
                .set_all_label_area_size(20)
                .build_ranged(-0f64..passed_nanos, 0usize..1usize).unwrap();

            cc.configure_mesh()
                .x_labels(10)
                .y_labels(2)
                .disable_mesh()
                .x_label_formatter(&|v| format!("{}", *v as u64))
                .y_label_formatter(&|v| format!("{}", v))
                .draw().unwrap();


            cc.draw_series(LineSeries::new(
                self.plot_digital_pins.iter().map(|record| (record.0, record.1[2] as usize)),
                &RED,
            )).unwrap().label("PA4");
        }

        // PA5
        {
            let mut cc = ChartBuilder::on(&plots[3])
                .margin(5)
                .set_all_label_area_size(20)
                .build_ranged(-0f64..passed_nanos, 0usize..1usize).unwrap();

            cc.configure_mesh()
                .x_labels(10)
                .y_labels(2)
                .disable_mesh()
                .x_label_formatter(&|v| format!("{}", *v as u64))
                .y_label_formatter(&|v| format!("{}", v))
                .draw().unwrap();


            cc.draw_series(LineSeries::new(
                self.plot_digital_pins.iter().map(|record| (record.0, record.1[3] as usize)),
                &RED,
            )).unwrap().label("PA5");
        }

        // PA6
        {
            let mut cc = ChartBuilder::on(&plots[4])
                .margin(5)
                .set_all_label_area_size(20)
                .build_ranged(-0f64..passed_nanos, 0usize..1usize).unwrap();

            cc.configure_mesh()
                .x_labels(10)
                .y_labels(2)
                .disable_mesh()
                .x_label_formatter(&|v| format!("{}", *v as u64))
                .y_label_formatter(&|v| format!("{}", v))
                .draw().unwrap();


            cc.draw_series(LineSeries::new(
                self.plot_digital_pins.iter().map(|record| (record.0, record.1[4] as usize)),
                &RED,
            )).unwrap().label("PA6");
        }

        // PA7
        {
            let mut cc = ChartBuilder::on(&plots[5])
                .margin(5)
                .set_all_label_area_size(20)
                .build_ranged(-0f64..passed_nanos, 0usize..1usize).unwrap();

            cc.configure_mesh()
                .x_labels(10)
                .y_labels(2)
                .disable_mesh()
                .x_label_formatter(&|v| format!("{}", *v as u64))
                .y_label_formatter(&|v| format!("{}", v))
                .draw().unwrap();


            cc.draw_series(LineSeries::new(
                self.plot_digital_pins.iter().map(|record| (record.0, record.1[5] as usize)),
                &RED,
            )).unwrap().label("PA7");
        }

        Ok(())
    }
}

impl HostAdapter for PlottingHost {
    fn read_pin_digital(&self, pin: Pin) -> bool {
        match pin {
            pins::PA7 => self.pa.pin7,
            pins::PA6 => self.pa.pin6,
            pins::PA5 => self.pa.pin5,
            pins::PA4 => self.pa.pin4,
            pins::PA3 => self.pa.pin3,
            pins::PA0 => self.pa.pin0,
            _ => unreachable!(),
        }
    }

    fn write_pin_digital(&mut self, pin: Pin, value: bool) {
        match pin {
            pins::PA7 => self.pa.pin7 = value,
            pins::PA6 => self.pa.pin6 = value,
            pins::PA5 => self.pa.pin5 = value,
            pins::PA4 => self.pa.pin4 = value,
            pins::PA3 => self.pa.pin3 = value,
            pins::PA0 => self.pa.pin0 = value,
            _ => unreachable!(),
        }
    }

    fn read_pin_analog(&self, pin: Pin) -> AnalogSignal { AnalogSignal::from_u16(0) }
    fn write_pin_analog(&mut self, pin: Pin, value: AnalogSignal) {}
    fn set_pin_output_enabled(&mut self, pin: Pin, enabled: bool) {}
    fn set_pin_pull_up_enabled(&mut self, pin: Pin, enabled: bool) {}
}