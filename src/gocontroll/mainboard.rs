use std::{io::{self, Error, prelude::*},fs};
use super::module::{GOcontrollModule, ModuleSlot};
use spidev::{Spidev, SpidevOptions, SpidevTransfer,SpiModeFlags};

#[derive(Debug,Clone)]
pub enum LedControl {
    None,
    Rukr,
    Gpio,
}

#[repr(u8)]
enum ModuleResetState {
    High = 1,
    Low = 0
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
    pub modules: [Option<()>;8],
    spidevs: [Option<Spidev>;8],
    resets: [Option<fs::File>;8],
}

const SPIDEVS: [&str;8] = [
    "/dev/spidev1.0",
    "/dev/spidev1.1",
    "/dev/spidev2.0",
    "/dev/spidev2.1",
    "/dev/spidev2.2",
    "/dev/spidev2.3",
    "/dev/spidev0.0",
    "/dev/spidev0.1"    
];

const RESETS: [&str;8] = [
    "/sys/class/leds/ResetM-1/brightness",
    "/sys/class/leds/ResetM-2/brightness",
    "/sys/class/leds/ResetM-3/brightness",
    "/sys/class/leds/ResetM-4/brightness",
    "/sys/class/leds/ResetM-5/brightness",
    "/sys/class/leds/ResetM-6/brightness",
    "/sys/class/leds/ResetM-7/brightness",
    "/sys/class/leds/ResetM-8/brightness",
];

impl MainBoard {
    pub const fn new() -> MainBoard {
        MainBoard { led_control: LedControl::None,
            adc: AdcConverter::None,
            module_layout: ModuleLayout::None, 
            modules: [None,None,None,None,None,None,None,None],
            spidevs: [None,None,None,None,None,None,None,None],
            resets: [None,None,None,None,None,None,None,None],
        }
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

        match self.module_layout {
            ModuleLayout::ModulineDisplay => {
                for i in 0..1 {
                    self.spidevs[i] = Some(Self::create_spi(i)?);
                    self.resets[i] = Some(Self::create_reset(i)?);
                }
            },
            ModuleLayout::ModulineMini => {
                for i in 0..3 {
                    self.spidevs[i] = Some(Self::create_spi(i)?);
                    self.resets[i] = Some(Self::create_reset(i)?);
                }
            },
            ModuleLayout::ModulineIV => {
                for i in 0..7 {
                    self.spidevs[i] = Some(Self::create_spi(i)?);
                    self.resets[i] = Some(Self::create_reset(i)?);
                }
            },
            ModuleLayout::None => {
                panic!("Should not be able to get here");
            }
        }
        modules.iter_mut().try_for_each(|module| -> Result<(),()>{
            let _ = module.put_configuration(self)?; Ok(())
        }).unwrap_or(return Err(io::Error::from(io::ErrorKind::InvalidData)));
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

    pub fn init_module(&mut self, slot: usize) -> io::Result<()> {
        self.reset_module_state(slot, ModuleResetState::High)?;
        self.spi_dummy_send(slot)?;
        Ok(())
    }

    pub fn spi_dummy_send(&mut self, slot: usize) -> io::Result<()> {
        const SPIDUMMY: [u8;6] = [1,2,3,4,5,6];
        self.spidevs[slot].as_mut().expect("Incorrectly initialized spi device").write(&SPIDUMMY)?;
        Ok(())
    }

    fn reset_module_state(&mut self, slot: usize, state: ModuleResetState) -> io::Result<()> {
        const STATE: [&str;2] = ["0", "1"];
        self.resets[slot].as_mut().expect("Incorrectly initialized module reset").write(STATE[state as usize].as_bytes())?;
        Ok(())
    }

    fn create_spi(slot: usize) -> io::Result<Spidev> {
        let mut spi = Spidev::open(SPIDEVS[slot])?;
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(2_000_000)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&options)?;
        Ok(spi)
    }
    fn create_reset(slot: usize) -> io::Result<fs::File> {
        fs::File::create(RESETS[slot])
    }
}