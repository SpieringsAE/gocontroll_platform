pub mod gocontroll;

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
        let mut mainboard: MainBoard = MainBoard::new();
        let mut input_module: InputModule6Ch = InputModule6Ch::new( ModuleSlot::Moduleslot1,
        [
            Some(InputModule6ChConfig::new(InputModule6ChFunction::AnalogmV, InputModule6ChPullDown::PullDown10k, InputModule6ChPullUp::PulUpnNone, InputModule6ChVoltageRange::Voltage0_5V,0u8,10u16)),
            None,
            None,
            None,
            None,
            None
        ],
        Inputmodule6chSupplyConfig::new(InputModuleSupply::On, InputModuleSupply::On, InputModuleSupply::On));
        let mut output_module: OutputModule6Ch = OutputModule6Ch::new(ModuleSlot::Moduleslot2,
        [
            Some(OutputModule6ChConfig::new(OutputModule6ChFunction::HalfBridge, Some(5000), None, None)),
            None,
            None,
            None,
            None,
            None,
        ],
        OutputModule6ChFrequecyConfig::new(OutputModule6ChFrequency::Freq1KHz,OutputModule6ChFrequency::Freq1KHz, OutputModule6ChFrequency::Freq1KHz));
        let mut input_module_10ch: InputModule10Ch = InputModule10Ch::new(ModuleSlot::Moduleslot3,
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
        let modules: &mut [&mut dyn GOcontrollModule] = &mut [&mut input_module, &mut output_module, &mut input_module_10ch];
        mainboard.init(modules);

        //other initialisation
    }
}
