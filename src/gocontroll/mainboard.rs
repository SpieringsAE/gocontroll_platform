use std::{io::{self, Error},fs};
use super::module::GOcontrollModule;

#[derive(Debug,Clone)]
pub enum LedControl {
    None,
    Rukr,
    Gpio,
}

#[derive(Debug,Clone)]
pub enum AdcConverter {
    None,
    Mcp3004,
    Ads1015,
}

#[repr(u8)]
#[derive(Debug,Clone,Copy)]
pub enum ModuleLayout {
    None=0,
    ModulineIV=7,
    ModulineMini=3,
    ModulineDisplay=1,
}

pub struct MainBoard {
    led_control: LedControl,
    adc: AdcConverter,
    pub module_layout: ModuleLayout,
    pub modules: [Option<()>;8]
}

impl MainBoard {
    pub const fn new() -> MainBoard {
        MainBoard { led_control: LedControl::None,
            adc: AdcConverter::None,
            module_layout: ModuleLayout::None, 
            modules: [None,None,None,None,None,None,None,None]}
    }

    pub fn initialize_main_board(&mut self, modules: &mut Vec<&mut dyn GOcontrollModule>) -> io::Result<()>{
        let hw = fs::read_to_string("/sys/firmware/devicetree/base/hardware")?;
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
        } else {
            self.module_layout = ModuleLayout::ModulineIV;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004;
        }
        modules.iter_mut().try_for_each(|module| -> Result<(),()>{ let _ = module.put_configuration(self)?; Ok(())}).unwrap_or(return Err(io::Error::from(io::ErrorKind::InvalidData)));
        Ok(())
    }

    pub fn check_module(&mut self, module: &dyn GOcontrollModule) -> Result<(),()> {
        if module.get_slot() as u8 > self.module_layout as u8 {
            println!("Could not initialize module in {}, it doesn't exist on this controller./n", module.get_slot());
            return Err(());
        }

        if self.modules[module.get_slot() as usize].is_none() {
            self.modules[module.get_slot() as usize] = Some(());
        } else {
            panic!("{} is trying to be occupied by 2 or more modules, check you module initialisation.\n", module.get_slot())
        }
        Ok(())
    }
}