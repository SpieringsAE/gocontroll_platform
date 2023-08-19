use std::io;

use spidev::Spidev;

use super::module::{GOcontrollModule, ModuleSlot, MessageType, CommunicationDirection};
use super::inputmodule6ch::InputModuleSupply;
use super::mainboard::MainBoard;

#[allow(unused)]
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

#[allow(unused)]
#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum InputModule10ChPullDown {
    PullDownNone = 0,
    PullDown3_3k = 3,
}

#[allow(unused)]
#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum InputModule10ChPullUp {
    PulUpnNone = 0,
    PullUp3_3k = 3,
}

#[allow(unused)]
#[repr(u8)]
pub enum InputModuleChannel {
    Channel1 = 0u8,
    Channel2 = 1u8,
    Channel3 = 2u8,
    Channel4 = 3u8,
    Channel5 = 4u8,
    Channel6 = 5u8,
    Channel7 = 6u8,
    Channel8 = 7u8,
    Channel9 = 8u8,
    Channel10 = 9u8,
}

const MODULEID:u8 =12;
const MESSAGELENGTH:usize = 50;

#[derive(Debug,Copy, Clone)]
pub struct InputModule10ChConfig {
    function: InputModule10ChFunction,
    pull_down: InputModule10ChPullDown,
    pull_up: InputModule10ChPullUp
}

#[allow(unused)]
impl InputModule10ChConfig {
    pub const fn new(function: InputModule10ChFunction, pull_down: InputModule10ChPullDown, pull_up: InputModule10ChPullUp) -> InputModule10ChConfig {
        InputModule10ChConfig { function, pull_down, pull_up }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct InputModule10Ch {
    slot: ModuleSlot,
    pulse_counter_reset: [u8; 10],
    sync_counter: [u32; 6],
    tx_data: [u8;56],
    rx_data: [u8;56],
    spidev: Option<Spidev>,
}

#[allow(unused)]
impl InputModule10Ch {
    pub const fn new(slot: ModuleSlot, channels: [Option<InputModule10ChConfig>;10],
    sensor_supply: InputModuleSupply ) -> InputModule10Ch {
        let mut tx_data = [0u8;56];
        let mut index: usize = 0;

        while index < 10 {
            match channels[index] {
                Some(config) => {
                    tx_data[index*4] = config.function as u8;
                    tx_data[index*4 +1] = (config.pull_up as u8) | ((config.pull_down as u8) << 2 );
                    index +=1;
                },
                None => index +=1
            }
        }
        
        tx_data[46] = sensor_supply as u8;
        InputModule10Ch { slot, pulse_counter_reset: [0u8;10], sync_counter: [0u32;6], tx_data, rx_data: [0u8;56], spidev: None }
    }

    pub async fn get_values(&mut self) -> io::Result<[i32;10]> {
        let mut result: [i32;10] = [0;10];
        MainBoard::send_receive_module_spi(
            1,
            CommunicationDirection::FromModule,
            MODULEID,
            MessageType::Data,
            1,
            self,
            MESSAGELENGTH
        )?;
        for i in 0..9 {
            result[i] = i32::from_le_bytes(self.rx_data[i*4+6..i*4+10].try_into().unwrap());
        }
        Ok(result)
    }

    pub async fn reset_pulse_counter(&mut self, channel: InputModuleChannel, value: i32) -> io::Result<()> {
        self.tx_data[6] = channel as u8;
        self.tx_data[7] = value as u8;
        self.tx_data[8] = {value >> 8} as u8;
        self.tx_data[9] = {value >> 16} as u8;
        self.tx_data[10] = {value >> 24} as u8;
        MainBoard::send_module_spi(
            1,
            CommunicationDirection::ToModule,
            MODULEID,
            MessageType::Data,
            2,
            self,
            MESSAGELENGTH
        )
    }

}

impl GOcontrollModule for InputModule10Ch {
    fn put_configuration(&mut self, mainboard: &mut MainBoard) -> io::Result<()>{

        mainboard.check_module(self)?;

        self.spidev = Some(MainBoard::create_spi(self.slot as usize)?);
        
        MainBoard::send_module_spi(
            1,
            CommunicationDirection::ToModule,
            MODULEID,
            MessageType::Configuration,
            1,
            self,
            MESSAGELENGTH
        )
    }
    fn get_slot(&self) -> ModuleSlot {
        self.slot
    }

    fn get_spidev(&self) -> Option<Spidev> {
        self.spidev
    }

    fn get_tx(&self) -> &[u8] {
        &self.tx_data
    }

    fn get_rx(&self) -> &[u8] {
        &self.rx_data
    }
}