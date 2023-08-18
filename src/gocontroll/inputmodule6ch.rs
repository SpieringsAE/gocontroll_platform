use std::io;

use super::{module::{GOcontrollModule, ModuleSlot, CommunicationDirection, MessageType}, mainboard::MainBoard};

#[allow(unused)]
#[repr(u8)]
#[derive(Debug,Copy, Clone)]
/// Input module channel functions:
/// None            -> Channel is unused\
/// Adc12bit        -> Read the raw ADC value\
/// AnalogmV        -> ADC converted to mV\
/// DigitalIn       -> Read pin as a digital input high or low\
/// FrequencyIn     -> Measure the frequency of the incoming signal\
/// DutyCycleLow    -> Measure the low period duty cycle\
/// DutyCycleHigh   -> Measure the high period duty cycle\
/// Rpm             -> PulseCounter converted to RPM with a set pulses per rotation configuration\
/// PulseCounter    -> Count the pulses on an input
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

#[allow(unused)]
#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum InputModule6ChPullDown {
    PullDownNone = 0,
    PullDown3_3k = 3,
    PullDown4_7k = 1,
    PullDown10k = 2,
}

#[allow(unused)]
#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum InputModule6ChPullUp {
    PulUpnNone = 0,
    PullUp3_3k = 3,
    PullUp4_7k = 1,
    PullUp10k = 2,
}

#[allow(unused)]
#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum InputModule6ChVoltageRange {
    Voltage0_5V = 0,
    Voltage0_12V = 1,
    Voltage0_24V = 2,
}

#[allow(unused)]
#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum InputModuleSupply{
    On = 1,
    Off = 2,
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
}

const MODULEID:u8 =11;
const MESSAGELENGTH:usize = 55;

#[allow(unused)]
#[derive(Debug,Clone,Copy)]
pub struct InputModule6Ch {
    slot: ModuleSlot,
    pulse_counter_reset: [u8; 6],
    sync_counter: [u32; 6],
    tx_data: [u8;56],
    rx_data: [u8;56],
}

#[allow(unused)]
impl InputModule6Ch {
    /// Create a new 6 channel input module object
    /// 
    /// # Arguments
    /// 
    /// * `slot` - The module slot that the module occupies
    /// * `channels` - An array with all the channel configurations
    /// * `supply` - The configuration of the module sensor supplies
    /// 
    /// # Examples
    /// 
    /// ```
    /// static mut INPUT_MODULE: InputModule6Ch = InputModule6Ch::new( ModuleSlot::Moduleslot1,
    /// [
    ///     Some(InputModule6ChConfig::new(InputModule6ChFunction::AnalogmV, InputModule6ChPullDown::PullDown10k, InputModule6ChPullUp::PulUpnNone, InputModule6ChVoltageRange::Voltage0_5V,0u8,10u16)),
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    ///     None
    /// ],
    /// Inputmodule6chSupplyConfig::new(InputModuleSupply::On, InputModuleSupply::On, InputModuleSupply::On));
    /// ```
    pub const fn new(slot: ModuleSlot, channels: [Option<InputModule6ChConfig>;6],
    supply: Inputmodule6chSupplyConfig ) -> InputModule6Ch {
        let mut tx_data = [0u8;56];
        let mut index = 0;
        while index < 6 {
            match channels[index] {
                Some(config)=> {
                    tx_data[index*6] = config.function as u8;
                    tx_data[index*6+1] = config.pull_up as u8 | (config.pull_down as u8) << 2 | (config.input_voltage as u8) << 6;
                    match config.function {
                        InputModule6ChFunction::None => (),
                        InputModule6ChFunction::Adc12Bit | InputModule6ChFunction::AnalogmV => {
                            tx_data[index*6+2] = (config.analog_filter_samples >> 8) as u8;
                            tx_data[index*6+3] = config.analog_filter_samples as u8;
                        },
                        _ => {
                            tx_data[index*6+2] = config.pulses_per_rotation;
                        }
                    }
                    index +=1;
                },
                None => {index+=1;}
            }
        }

        tx_data[42] = supply.sensor_supplies[0] as u8;
        tx_data[43] = supply.sensor_supplies[1] as u8;
        tx_data[44] = supply.sensor_supplies[2] as u8;
        InputModule6Ch {
            slot,
            pulse_counter_reset : [0u8; 6],
            sync_counter : [0u32;6],
            tx_data,
            rx_data: [0u8;56],
        }
    }

    pub fn get_values(&mut self, mainboard: &mut MainBoard) -> io::Result<[i32;6]> {
        let mut result: [i32;6] = [0;6];
        mainboard.send_receive_module_spi(
            1,
            CommunicationDirection::FromModule,
            MODULEID,
            MessageType::Data,
            1,
            self.slot as usize,
            &mut self.tx_data,
            &mut self.rx_data,
            MESSAGELENGTH
        )?;
        for i in 0..5 {
            result[i] = i32::from_le_bytes(self.rx_data[i*8+6..i*8+10].try_into().unwrap());
        }
        Ok(result)
    }

    pub fn reset_pulse_counter(&mut self, mainboard: &mut MainBoard, channel: InputModuleChannel, value: i32) -> io::Result<()> {
        self.tx_data[6] = channel as u8;
        self.tx_data[7] = value as u8;
        self.tx_data[8] = {value >> 8} as u8;
        self.tx_data[9] = {value >> 16} as u8;
        self.tx_data[10] = {value >> 24} as u8;
        mainboard.send_module_spi(
            1,
            CommunicationDirection::ToModule,
            MODULEID,
            MessageType::Data,
            2,
            self.slot as usize,
            &mut self.tx_data,
            MESSAGELENGTH
        )
    }
}

impl GOcontrollModule for InputModule6Ch {
    fn put_configuration(&mut self, mainboard: &mut MainBoard) -> io::Result<()>{

        mainboard.check_module(self)?;
        
        mainboard.send_module_spi(
            1,
            CommunicationDirection::ToModule,
            MODULEID,
            MessageType::Configuration,
            1,
            self.slot as usize,
            &mut self.tx_data,
            MESSAGELENGTH
        )
    }
    fn get_slot(&self) -> ModuleSlot {
        self.slot
    }
}

#[derive(Debug,Copy, Clone)]
pub struct InputModule6ChConfig {
    function: InputModule6ChFunction,
    pull_down: InputModule6ChPullDown,
    pull_up: InputModule6ChPullUp,
    input_voltage: InputModule6ChVoltageRange,
    pulses_per_rotation: u8,
    analog_filter_samples:u16
}

#[allow(unused)]
impl InputModule6ChConfig {
    pub const fn new(function: InputModule6ChFunction, pull_down: InputModule6ChPullDown, pull_up: InputModule6ChPullUp,
    input_voltage: InputModule6ChVoltageRange, pulses_per_rotation: u8, analog_filter_samples: u16) -> InputModule6ChConfig {
        InputModule6ChConfig {function, pull_down, pull_up, input_voltage, pulses_per_rotation, analog_filter_samples}
    }
}

#[derive(Debug,Copy, Clone)]
pub struct Inputmodule6chSupplyConfig {
    sensor_supplies: [InputModuleSupply; 3],
}

#[allow(unused)]
impl Inputmodule6chSupplyConfig {
    pub const fn new(supply1: InputModuleSupply, supply2: InputModuleSupply, supply3: InputModuleSupply) -> Inputmodule6chSupplyConfig {
        Inputmodule6chSupplyConfig { sensor_supplies: [supply1,supply2,supply3] }
    }
}