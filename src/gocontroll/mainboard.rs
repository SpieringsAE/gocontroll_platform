use std::{fs,
    sync::{Mutex,Arc},
};
use rayon::prelude::*;

use super::module::GOcontrollModule;


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

#[derive(Debug,Clone)]
pub struct MainBoard {
    led_control: LedControl,
    adc: AdcConverter,
    pub modules: [Option<Arc<Mutex<dyn GOcontrollModule>>>;8],
    module_layout: ModuleLayout,
}

impl MainBoard {
    pub const fn new() -> MainBoard {
        MainBoard { led_control: LedControl::None, adc: AdcConverter::Mcp3004, modules: [None,None,None,None,None,None,None,None], module_layout: ModuleLayout::ModulineIV }
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

    pub fn add_module(&mut self, module: Arc<Mutex<dyn GOcontrollModule>>) -> Arc<Mutex<dyn GOcontrollModule>> {
        let inserted_module  =  module;
        let slot = inserted_module.lock().unwrap().get_slot();
        if self.modules[slot as usize].is_none() {
            self.modules[slot as usize] = Some(inserted_module.clone());
            return self.modules[slot as usize].clone().unwrap()
        }
        panic!("module slot {} is already occupied!", slot as u8);
    }

    pub fn configure_modules(&self) {
        let _ = self.modules.iter()
            .flatten()
            .map(|module| module.lock().unwrap().put_configuration());
        }
}