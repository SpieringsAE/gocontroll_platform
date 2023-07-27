mod gocontroll;

#[cfg(test)]
mod tests {
    use crate::gocontroll::{
        mainboard::*,
        module::*,
        inputmodule6ch::*,
        inputmodule10ch::*,
        outputmodule6ch::*,
    };


    #[test]
    fn it_works() {
        static mut MAINBOARD: MainBoard = MainBoard::new();
        static mut INPUT_MODULE: InputModule6Ch = InputModule6Ch::new( ModuleSlot::Moduleslot1,
        [
            Some(InputModule6ChConfig::new(InputModule6ChFunction::AnalogmV, InputModule6ChPullDown::PullDown10k, InputModule6ChPullUp::PulUpnNone, InputModule6ChVoltageRange::Voltage0_5V,0u8,10u16)),
            None,
            None,
            None,
            None,
            None
        ],
        Inputmodule6chSupplyConfig::new(InputModuleSupply::On, InputModuleSupply::On, InputModuleSupply::On));
        static mut OUTPUT_MODULE: OutputModule6Ch = OutputModule6Ch::new(ModuleSlot::Moduleslot2,
        [
            Some(OutputModule6ChConfig::new(OutputModule6ChFunction::HalfBridge, Some(5000), None, None)),
            None,
            None,
            None,
            None,
            None,
        ],
        OutputModule6ChFrequecyConfig::new(OutputModule6ChFrequency::Freq1KHz,OutputModule6ChFrequency::Freq1KHz, OutputModule6ChFrequency::Freq1KHz));
        static mut INPUT_MODULE_10CH: InputModule10Ch = InputModule10Ch::new(ModuleSlot::Moduleslot3,
        [
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        InputModuleSupply::On);
        
        unsafe {
            let _ = MAINBOARD.initialize_main_board(&mut vec![&mut INPUT_MODULE,&mut OUTPUT_MODULE,&mut INPUT_MODULE_10CH]);
        };
    }
}
