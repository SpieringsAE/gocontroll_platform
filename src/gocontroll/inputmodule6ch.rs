use super::{module::{GOcontrollModule, ModuleSlot}, mainboard::MainBoard};

#[repr(u8)]
#[derive(Debug,Copy, Clone)]
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
#[derive(Debug,Copy, Clone)]
pub enum InputModule6ChPullDown {
    PullDownNone = 0,
    PullDown3_3k = 3,
    PullDown4_7k = 1,
    PullDown10k = 2,
}

#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum InputModule6ChPullUp {
    PulUpnNone = 0,
    PullUp3_3k = 3,
    PullUp4_7k = 1,
    PullUp10k = 2,
}

#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum InputModule6ChVoltageRange {
    Voltage0_5V = 0,
    Voltage0_12V = 1,
    Voltage0_24V = 2,
}

#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum InputModuleSupply{
    On = 1,
    Off = 2,
}

#[derive(Debug,Clone,Copy)]
pub struct InputModule6Ch {
    slot: ModuleSlot,
    channels: [Option<InputModule6ChConfig>;6],
    supply: Inputmodule6chSupplyConfig,
    module_identifier: u32,
    pulse_counter_reset: [u8; 6],
    sync_counter: [u32; 6],
    tx_data: [u8;56],
}

impl InputModule6Ch {
    pub const fn new(slot: ModuleSlot, channels: [Option<InputModule6ChConfig>;6],
    supply: Inputmodule6chSupplyConfig ) -> InputModule6Ch {
        InputModule6Ch {
            slot,
            channels, supply,
            module_identifier : 0, pulse_counter_reset : [0u8; 6], sync_counter : [0u32;6],
            tx_data : [0u8;56]
        }
    }
}

impl GOcontrollModule for InputModule6Ch {
    fn put_configuration(&mut self, mainboard: &mut MainBoard) -> Result<(),()>{

        mainboard.check_module(self)?;

        let mut index: usize = 0;

        let _ = self.channels.iter().map(|channel| {
            match channel {
                Some(config)=> {
                    self.tx_data[index*6] = config.function as u8;
                    self.tx_data[index*6+1] = config.pull_up as u8 | (config.pull_down as u8) << 2 | (config.input_voltage as u8) << 6;
                    match config.function {
                        InputModule6ChFunction::None => (),
                        InputModule6ChFunction::Adc12Bit | InputModule6ChFunction::AnalogmV => {
                            self.tx_data[index*6+2] = (config.analog_filter_samples >> 8) as u8;
                            self.tx_data[index*6+3] = config.analog_filter_samples as u8;
                        },
                        _ => {
                            self.tx_data[index*6+2] = config.pulses_per_rotation;
                        }
                    }
                    index +=1;
                },
                None => {index+=1;}
            }
        });

        self.tx_data[42] = self.supply.sensor_supplies[0] as u8;
        self.tx_data[43] = self.supply.sensor_supplies[1] as u8;
        self.tx_data[44] = self.supply.sensor_supplies[2] as u8;
        
        //send spi
        Ok(())
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
impl Inputmodule6chSupplyConfig {
    pub const fn new(supply1: InputModuleSupply, supply2: InputModuleSupply, supply3: InputModuleSupply) -> Inputmodule6chSupplyConfig {
        Inputmodule6chSupplyConfig { sensor_supplies: [supply1,supply2,supply3] }
    }
}