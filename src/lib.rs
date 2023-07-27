mod gocontroll;

#[cfg(test)]
mod tests {
    use crate::gocontroll::{
        mainboard::*,
        module::*,
        inputmodule6ch::*,
        inputmodule10ch::*,
    };


    #[test]
    fn it_works() {
        static mut MAINBOARD: MainBoard = MainBoard::new();
        static mut MODULE: InputModule6Ch = InputModule6Ch::new( ModuleSlot::Moduleslot1,
        [
            Some(InputModule6ChConfig::new(InputModule6ChFunction::AnalogmV, InputModule6ChPullDown::PullDown10k, InputModule6ChPullUp::PulUpnNone, InputModule6ChVoltageRange::Voltage0_5V,0u8,10u16)),
            None,
            None,
            None,
            None,
            None
        ],
        Inputmodule6chSupplyConfig::new(InputModuleSupply::On, InputModuleSupply::On, InputModuleSupply::On));

        unsafe {
            MAINBOARD.initialize_main_board();
            let _ = MODULE.put_configuration(&mut MAINBOARD);
        };
    }
}
