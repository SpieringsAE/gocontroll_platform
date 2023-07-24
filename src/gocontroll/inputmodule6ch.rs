use super::module::{GOcontrollModule, ModuleSlot};
use std::sync::{Mutex,Arc};

#[repr(u8)]
#[derive(Copy,Clone)]
pub enum InputModule6ChFunction {
    None = 0,
    Adc12Bit = 1,
    AnalogmV = 2,
    DigitalIn = 3,
    FrequencyIn = 4,
    DutyCycleLow = 5,
    DutyCycleHigh = 6,
    Rpm = 7,
    PulseCounter = 8,
}

#[repr(u8)]
pub enum InputModule6ChPullDown {
    PullDownNone = 0,
    PullDown3_3k = 3,
    PullDown4_7k = 1,
    PullDown10k = 2,
}

#[repr(u8)]
pub enum InputModule6ChPullUp {
    PulUpnNone = 0,
    PullUp3_3k = 3,
    PullUp4_7k = 1,
    PullUp10k = 2,
}

#[repr(u8)]
pub enum InputModule6ChVoltageRange {
    Voltage0_5V = 0,
    Voltage0_12V = 1,
    Voltage0_24V = 2,
}

#[repr(u8)]
pub enum InputModuleSupply{
    On = 1,
    Off = 2,
}

#[derive(Debug,Clone,Copy)]
pub struct InputModule6Ch {
    slot: ModuleSlot,
    sensor_supplies: [u8; 3],
    module_identifier: u32,
    pulse_counter_reset: [u8; 6],
    sync_counter: [u32; 6],
    configuration: [u8;36],
    tx_data: [u8;56],
}

impl GOcontrollModule for InputModule6Ch {
    fn put_configuration(&mut self) {
        
        for (place, data) in self.tx_data.iter_mut().zip(self.configuration.iter()) {
            *place = *data;
        }
        self.tx_data[42] = self.sensor_supplies[0];
        self.tx_data[43] = self.sensor_supplies[1];
        self.tx_data[44] = self.sensor_supplies[2];
        
        //send spi
    }

    fn get_slot(&self) ->ModuleSlot {
        self.slot
    }
}

#[derive(Debug,Copy, Clone)]
pub struct InputModule6ChBuilder {
    slot: ModuleSlot,
    sensor_supplies: [u8; 3],
    configuration: [u8;36],
    _index: usize,
}

impl InputModule6ChBuilder {
    
    pub const fn new(slot: ModuleSlot) -> InputModule6ChBuilder {
        InputModule6ChBuilder {slot, sensor_supplies: [0;3], configuration: [0;36], _index: 0 }
    }
    pub fn configure_channel(&mut self, function: InputModule6ChFunction, pull_down: InputModule6ChPullDown, pull_up: InputModule6ChPullUp, input_voltage: InputModule6ChVoltageRange, pulses_per_rotation: u8, analog_filter_samples:u16) -> &mut Self {
        if self._index >= 6 {
            panic!("Too many channels configured, this module only has 6 available channels!");
        }
        self.configuration[self._index*6] = function as u8;
        self.configuration[self._index*6+1] = (pull_up as u8) | ((pull_down as u8) << 2 ) | ((input_voltage as u8) << 6);
        match function {
            InputModule6ChFunction::None => (),
            InputModule6ChFunction::Adc12Bit | InputModule6ChFunction::AnalogmV => {
                self.configuration[self._index*6+2] = (analog_filter_samples >> 8) as u8;
                self.configuration[self._index*6+3] = analog_filter_samples as u8;
            },
            InputModule6ChFunction::DigitalIn | InputModule6ChFunction::FrequencyIn | InputModule6ChFunction::DutyCycleLow | InputModule6ChFunction::DutyCycleHigh | InputModule6ChFunction::Rpm | InputModule6ChFunction::PulseCounter => {
                self.configuration[self._index*6+2] = pulses_per_rotation;
            }
        }
        self._index+=1;
        self
    }
    pub fn configure_supply(&mut self, supply1: InputModuleSupply, supply2: InputModuleSupply, supply3: InputModuleSupply) -> &mut Self {
        self.sensor_supplies[0] = supply1 as u8;
        self.sensor_supplies[1] = supply2 as u8;
        self.sensor_supplies[2] = supply3 as u8;
        self
    }

    pub fn build(self) -> Arc<Mutex<InputModule6Ch>> {
        Arc::new(Mutex::new(InputModule6Ch {
            slot: self.slot,
            sensor_supplies: self.sensor_supplies,
            module_identifier: 0u32,
            pulse_counter_reset: [0;6],
            sync_counter: [0;6],
            configuration: self.configuration,
            tx_data: [0u8;56],
        }))
    }
}