use std::{fs,
    sync::{Mutex,Arc},
};
use rayon::prelude::*;

use super::module::{GOcontrollModule, self};


#[derive(Debug,Clone)]
pub enum LedControl {
    None,
    Rukr,
    Gpio,
}

#[derive(Debug,Clone)]
pub enum AdcConverter {
    Mcp3004,
    Ads1015,
}

#[derive(Debug,Clone)]
pub enum ModuleLayout {
    ModulineIV,
    ModulineMini,
    ModulineDisplay,
}

pub struct MainBoard {
    led_control: LedControl,
    adc: AdcConverter,
    module: Option<&'static mut dyn GOcontrollModule>,
    // modules: [Option<&'static mut dyn GOcontrollModule>;8],
    module_layout: ModuleLayout,
}

impl MainBoard {
    pub const fn new(module: Option<&'static mut dyn GOcontrollModule>) -> MainBoard {
        MainBoard { led_control: LedControl::None, adc: AdcConverter::Mcp3004, module: module, module_layout: ModuleLayout::ModulineIV }
    }

    pub fn initialize_main_board(&mut self) {
        let hw = fs::read_to_string("/sys/firmware/devicetree/base/hardware").unwrap();
        if hw.eq("Moduline IV V3.06") {
            self.module_layout = ModuleLayout::ModulineIV;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004;
        } else if hw.eq("Moduline Mini V1.11") {
            self.module_layout = ModuleLayout::ModulineMini;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004;
        } else if hw.eq("Moduline Screen V1.04") {
            self.module_layout = ModuleLayout::ModulineDisplay;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004;
        } else if hw.eq("Moduline IV V3.00") {
            self.module_layout = ModuleLayout::ModulineIV;
            self.led_control = LedControl::Gpio;
            self.adc = AdcConverter::Ads1015;
        } else if hw.eq("Moduline IV V3.01") {
            self.module_layout = ModuleLayout::ModulineIV;
            self.led_control = LedControl::Gpio;
            self.adc = AdcConverter::Ads1015;
        } else if hw.eq("Moduline IV V3.02") {
            self.module_layout = ModuleLayout::ModulineIV;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Ads1015;
        } else if hw.eq("Moduline IV V3.03") {
            self.module_layout = ModuleLayout::ModulineIV;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Ads1015;
        } else if hw.eq("Moduline IV V3.04") {
            self.module_layout = ModuleLayout::ModulineIV;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Ads1015;
        } else if hw.eq("Moduline IV V3.05") {
            self.module_layout = ModuleLayout::ModulineIV;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Ads1015;
        } else if hw.eq("Moduline Mini V1.03") {
            self.module_layout = ModuleLayout::ModulineMini;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Ads1015;
        } else if hw.eq("Moduline Mini V1.05") {
            self.module_layout = ModuleLayout::ModulineMini;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004;
        } else if hw.eq("Moduline Mini V1.06") {
            self.module_layout = ModuleLayout::ModulineMini;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004;
        } else if hw.eq("Moduline Mini V1.07") {
            self.module_layout = ModuleLayout::ModulineMini;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004;
        } else if hw.eq("Moduline Mini V1.10") {
            self.module_layout = ModuleLayout::ModulineMini;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004;
        } else if hw.eq("Moduline Screen V1.02") {
            self.module_layout = ModuleLayout::ModulineDisplay;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004;
        } else if hw.eq("Moduline Screen V1.03") {
            self.module_layout = ModuleLayout::ModulineDisplay;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004;
        }
    }

    // pub const fn add_module(&mut self, module: Arc<Mutex<impl GOcontrollModule>>) -> Arc<Mutex<impl GOcontrollModule>> {
    //     // let inserted_module  =  module;
    //     let slot = module.lock().unwrap().get_slot();
    //     if self.modules[slot as usize].is_none() {
    //         self.modules[slot as usize] = Some(module.clone());
    //         return module
    //     }
    //     panic!("module slot {} is already occupied!", slot as u8);
    // }

    pub fn configure_modules(&mut self) {
        // let _ = for (maybe_module) in self.modules.iter() {
        //     match *maybe_module {
        //         Some(module) => {
        //             module.put_configuration();
        //         },
        //         None => ()
        //     }
        // };

        self.module.as_mut().unwrap().put_configuration();
    }
}