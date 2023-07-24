use super::module::{GOcontrollModule, ModuleSlot};
use super::inputmodule6ch::InputModuleSupply;
use std::sync::{Mutex,Arc};

#[repr(u8)]
pub enum InputModule10ChFunction {
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
pub enum InputModule10ChPullDown {
    PullDownNone = 0,
    PullDown3_3k = 3,
}

#[repr(u8)]
pub enum InputModule10ChPullUp {
    PulUpnNone = 0,
    PullUp3_3k = 3,
}

#[derive(Debug,Clone,Copy)]
pub struct InputModule10Ch {
    slot: ModuleSlot,
    sensor_supply: u8,
    module_identifier: u32,
    pulse_counter_reset: [u8; 10],
    sync_counter: [u32; 6],
    configuration: [u8;40],
    tx_data: [u8;56],
}

impl GOcontrollModule for InputModule10Ch {
    fn put_configuration(&mut self) {
        
        for (place, data) in self.tx_data.iter_mut().zip(self.configuration.iter()) {
            *place = *data;
        }
        self.tx_data[46] = self.sensor_supply;
        
        //send spi
    }

    fn get_slot(&self) ->ModuleSlot {
        self.slot
    }
}

#[derive(Debug)]
pub struct InputModule10ChBuilder {
    slot: ModuleSlot,
    sensor_supply: u8,
    configuration: [u8;40],
    _index: usize,
}

impl InputModule10ChBuilder {
    
    pub fn new(slot: ModuleSlot) -> InputModule10ChBuilder {
        InputModule10ChBuilder {slot, sensor_supply: 0u8, configuration: [0;40], _index: 0 }
    }
    pub fn configure_channel(&mut self, function: InputModule10ChFunction, pull_down: InputModule10ChPullDown, pull_up: InputModule10ChPullUp) -> &mut Self {
        if self._index >= 10 {
            panic!("Too many channels configured, this module only has 6 available channels!");
        }
        self.configuration[self._index*4] = function as u8;
        self.configuration[self._index*4+1] = (pull_up as u8) | ((pull_down as u8) << 2 );
        self._index+=1;
        self
    }
    pub fn configure_supply(&mut self, supply: InputModuleSupply) -> &mut Self {
        self.sensor_supply = supply as u8;

        self
    }

    pub fn build(self) -> Arc<Mutex<InputModule10Ch>> {
        Arc::new(Mutex::new(InputModule10Ch {
            slot: self.slot,
            sensor_supply: self.sensor_supply,
            module_identifier: 0u32,
            pulse_counter_reset: [0;10],
            sync_counter: [0;6],
            configuration: self.configuration,
            tx_data: [0u8;56],
        }))
    }
}