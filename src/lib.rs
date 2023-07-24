mod gocontroll;

#[cfg(test)]
mod tests {
    use crate::gocontroll::{
        mainboard::*,
        module::*,
        inputmodule6ch::*,
        inputmodule10ch::*,
    };
    use std::sync::{Mutex,Arc};


    #[test]
    fn it_works() {
        static mut MAINBOARD: MainBoard = MainBoard::new();
        // static input_module_1: Arc<Mutex<dyn GOcontrollModule>> = unsafe { MAINBOARD.add_module(
        //     InputModule6ChBuilder::new(ModuleSlot::Moduleslot1)
        //         .configure_channel(
        //             InputModule6ChFunction::AnalogmV,
        //             InputModule6ChPullDown::PullDown4_7k,
        //             InputModule6ChPullUp::PulUpnNone,
        //             InputModule6ChVoltageRange::Voltage0_5V,
        //             0,
        //             10)
        //         .configure_channel(
        //             InputModule6ChFunction::AnalogmV,
        //             InputModule6ChPullDown::PullDown4_7k,
        //             InputModule6ChPullUp::PulUpnNone,
        //             InputModule6ChVoltageRange::Voltage0_5V,
        //             0,
        //             10)
        //         .configure_supply(
        //             InputModuleSupply::On,
        //             InputModuleSupply::On,
        //             InputModuleSupply::On)
        //         .build()
        //     )
        // };
        // static input_module_2: Arc<Mutex<dyn GOcontrollModule>> = unsafe { MAINBOARD.add_module(
        //         InputModule10ChBuilder::new(ModuleSlot::Moduleslot2)
        //         .configure_channel(
        //             InputModule10ChFunction::AnalogmV,
        //             InputModule10ChPullDown::PullDown3_3k,
        //             InputModule10ChPullUp::PulUpnNone)
        //         .build()
        //     )
        // };
        static MODULE: Arc<Mutex<dyn GOcontrollModule>> = InputModule6ChBuilder::new(ModuleSlot::Moduleslot1)
        .configure_channel(
                        InputModule6ChFunction::AnalogmV,
                        InputModule6ChPullDown::PullDown4_7k,
                        InputModule6ChPullUp::PulUpnNone,
                        InputModule6ChVoltageRange::Voltage0_5V,
                        0,
                        10)
        .build();
        unsafe { MAINBOARD.configure_modules();
        MAINBOARD.initialize_main_board()};
    }
}
