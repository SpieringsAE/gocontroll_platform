use super::{module::{GOcontrollModule,ModuleSlot},
    mainboard::MainBoard};

#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum OutputModule6ChFunction {
    None = 1,
    HalfBridge = 2,
    LowSideDutyCycle = 3,
    HighSideDutyCycle = 4,
    LowSideSwitch = 5,
    HighSideSwitch = 6,
    PeakAndHold = 7,
    FrequencyOut = 8,
}

#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum OutputModule6ChFrequency {
    Freq100Hz = 1,
    Freq200Hz = 2,
    Freq500Hz = 3,
    Freq1KHz = 4,
    Freq2KHz = 5,
    Freq5KHz = 6,
    Freq10KHz = 7,
}

#[derive(Debug,Copy, Clone)]
pub struct OutputModule6ChConfig {
    function: OutputModule6ChFunction,
    peak_current: Option<u16>,
    peak_time: Option<u16>,
    max_current: Option<u16>,
}

impl OutputModule6ChConfig {
    pub const fn new(function: OutputModule6ChFunction, max_current: Option<u16>, peak_current: Option<u16>, peak_time: Option<u16>)-> OutputModule6ChConfig {
        OutputModule6ChConfig {function, max_current, peak_current, peak_time}
    }
}

#[derive(Debug,Copy, Clone)]
pub struct OutputModule6ChFrequecyConfig {
    frequencies: [OutputModule6ChFrequency;3],
}

impl OutputModule6ChFrequecyConfig {
    pub const fn new(channel1_2: OutputModule6ChFrequency, channel3_4: OutputModule6ChFrequency, channel5_6: OutputModule6ChFrequency) -> OutputModule6ChFrequecyConfig {
        OutputModule6ChFrequecyConfig {frequencies: [channel1_2,channel3_4,channel5_6]}
    }
}


#[derive(Debug,Clone,Copy)]
pub struct OutputModule6Ch {
    slot: ModuleSlot,
    channels: [Option<OutputModule6ChConfig>;6],
    frequency_channels: OutputModule6ChFrequecyConfig,
    module_identifier: u32,
    tx_data: [u8;50],
}

impl OutputModule6Ch {
    pub const fn new(slot: ModuleSlot, channels: [Option<OutputModule6ChConfig>;6], frequency_channels: OutputModule6ChFrequecyConfig) -> OutputModule6Ch {
        OutputModule6Ch {slot, channels, frequency_channels, module_identifier: 0u32, tx_data: [0u8;50]}
    }
}

impl GOcontrollModule for OutputModule6Ch {
    fn put_configuration(&mut self, mainboard: &mut MainBoard) -> Result<(),()> {
        mainboard.check_module(self)?;

        let mut index: usize = 0;

        Ok(())
    }
    fn get_slot(&self) -> ModuleSlot {
        self.slot
    }
}