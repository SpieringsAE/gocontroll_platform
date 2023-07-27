use super::module::{GOcontrollModule, ModuleSlot};
use super::inputmodule6ch::{InputModuleSupply, InputModule6ChPullUp, InputModule6ChConfig};
use super::mainboard::MainBoard;

#[repr(u8)]
#[derive(Debug,Copy, Clone)]
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
#[derive(Debug,Copy, Clone)]
pub enum InputModule10ChPullDown {
    PullDownNone = 0,
    PullDown3_3k = 3,
}

#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum InputModule10ChPullUp {
    PulUpnNone = 0,
    PullUp3_3k = 3,
}

#[derive(Debug,Copy, Clone)]
pub struct InputModule10ChConfig {
    function: InputModule10ChFunction,
    pull_down: InputModule10ChPullDown,
    pull_up: InputModule10ChPullUp
}

impl InputModule10ChConfig {
    pub const fn new(function: InputModule10ChFunction, pull_down: InputModule10ChPullDown, pull_up: InputModule10ChPullUp) -> InputModule10ChConfig {
        InputModule10ChConfig { function, pull_down, pull_up }
    }
}

#[derive(Debug,Clone,Copy)]
pub struct InputModule10Ch {
    slot: ModuleSlot,
    sensor_supply: InputModuleSupply,
    channels: [Option<InputModule10ChConfig>;10],
    module_identifier: u32,
    pulse_counter_reset: [u8; 10],
    sync_counter: [u32; 6],
    tx_data: [u8;56],
}

impl InputModule10Ch {
    pub const fn new(slot: ModuleSlot, channels: [Option<InputModule10ChConfig>;10],
    sensor_supply: InputModuleSupply ) -> InputModule10Ch {
        InputModule10Ch { slot, sensor_supply, channels, module_identifier: 0,
            pulse_counter_reset: [0u8;10], sync_counter: [0u32;6], tx_data: [0u8;56] }
    }
}

impl GOcontrollModule for InputModule10Ch {
    fn put_configuration(&mut self, mainboard: &mut MainBoard) -> Result<(),()>{

        mainboard.check_module(self)?;

        let mut index: usize = 0;

        let _ = self.channels.iter().map(|channel| {
            match channel {
                Some(config) => {
                    self.tx_data[index*4] = config.function as u8;
                    self.tx_data[index*4 +1] = (config.pull_up as u8) | ((config.pull_down as u8) << 2 );
                    index +=1;
                },
                None => index +=1
            }
        });
        
        self.tx_data[46] = self.sensor_supply as u8;
        
        //send spi
        Ok(())
    }
    fn get_slot(&self) -> ModuleSlot {
        self.slot
    }
}