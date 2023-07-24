use super::module::{GOcontrollModule,ModuleSlot};

#[repr(u8)]
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
pub enum OutputModule6ChFrequency {
    Freq100Hz = 1,
    Freq200Hz = 2,
    Freq500Hz = 3,
    Freq1KHz = 4,
    Freq2KHz = 5,
    Freq5KHz = 6,
    Freq10KHz = 7,
}

#[derive(Debug,Clone,Copy)]
pub struct OuputModule6Ch {
    slot: ModuleSlot,
    module_identifier: u32,
    configuration: [u8;40],
    tx_data: [u8;50],
}