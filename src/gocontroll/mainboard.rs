use std::{io::{self, prelude::*},fs, path::PathBuf};
use super::module::GOcontrollModule;
use spidev::{Spidev, SpidevOptions,SpiModeFlags};
use i2c_linux::I2c;

#[allow(unused)]
#[derive(Debug,Clone)]
pub enum LedControl {
    None,
    Rukr,
    Gpio,
}

#[allow(unused)]
pub enum AdcChannel {
    K30,
    K15A,
    K15B,
    K15C,
}

#[allow(unused)]
#[repr(u8)]
#[derive(Copy,Clone)]
pub enum EnclosureLed {
    Led1,
    Led2,
    Led3,
    Led4,
}

#[allow(unused)]
#[repr(u8)]
enum ModuleResetState {
    High = 1,
    Low = 0
}

#[allow(unused)]
pub enum AdcConverter {
    None,
    Mcp3004([Option<PathBuf>;4]),
    Ads1015,
}

#[allow(unused)]
#[repr(u8)]
#[derive(Debug,Clone,Copy)]
pub enum ModuleLayout {
    None=0,
    ModulineIV=7,
    ModulineMini=3,
    ModulineDisplay=1,
}

#[allow(unused)]
pub struct MainBoard {
    led_control: LedControl,
    adc: AdcConverter,
    pub module_layout: ModuleLayout,
    pub modules: [Option<usize>;8],
    resets: [Option<fs::File>;8],
}

#[allow(unused)]
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

#[allow(unused)]
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

#[allow(unused)]
const GPIO_LEDS: [&str;12] = [
    "/sys/class/leds/Status1-r/brightness",
    "/sys/class/leds/Status1-g/brightness",
    "/sys/class/leds/Status1-b/brightness",
    "/sys/class/leds/Status2-r/brightness",
    "/sys/class/leds/Status2-g/brightness",
    "/sys/class/leds/Status2-b/brightness",
    "/sys/class/leds/Status3-r/brightness",
    "/sys/class/leds/Status3-g/brightness",
    "/sys/class/leds/Status3-b/brightness",
    "/sys/class/leds/Status4-r/brightness",
    "/sys/class/leds/Status4-g/brightness",
    "/sys/class/leds/Status4-b/brightness",
];

#[allow(unused)]
const RUKR_LEDS: &str = "/dev/i2c-2";

#[allow(unused)]
const ADS_ADC: &str = "/dev/i2c-2";

#[allow(unused)]
impl MainBoard {
    pub const fn new() -> MainBoard {
        MainBoard { led_control: LedControl::None,
            adc: AdcConverter::None,
            module_layout: ModuleLayout::None, 
            modules: [None,None,None,None,None,None,None,None],
            resets: [None,None,None,None,None,None,None,None],
        }
    }

    pub async fn initialize_main_board(&mut self, modules: &mut [&mut dyn GOcontrollModule]) -> io::Result<()>{
        let hw = fs::read_to_string("/sys/firmware/devicetree/base/hardware").expect("Cannot find hardware spec, are you running this on a Moduline product?");
        if hw.contains("Moduline IV V3.06") {
            self.module_layout = ModuleLayout::ModulineIV;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004([None,None,None,None]);
        } else if hw.contains("Moduline Mini V1.11") {
            self.module_layout = ModuleLayout::ModulineMini;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004([None,None,None,None]);
        } else if hw.contains("Moduline Screen V1.04") {
            self.module_layout = ModuleLayout::ModulineDisplay;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004([None,None,None,None]);
        } else if hw.contains("Moduline IV V3.00") {
            self.module_layout = ModuleLayout::ModulineIV;
            self.led_control = LedControl::Gpio;
            self.adc = AdcConverter::Ads1015;
        } else if hw.contains("Moduline IV V3.01") {
            self.module_layout = ModuleLayout::ModulineIV;
            self.led_control = LedControl::Gpio;
            self.adc = AdcConverter::Ads1015;
        } else if hw.contains("Moduline IV V3.02") {
            self.module_layout = ModuleLayout::ModulineIV;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Ads1015;
        } else if hw.contains("Moduline IV V3.03") {
            self.module_layout = ModuleLayout::ModulineIV;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Ads1015;
        } else if hw.contains("Moduline IV V3.04") {
            self.module_layout = ModuleLayout::ModulineIV;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Ads1015;
        } else if hw.contains("Moduline IV V3.05") {
            self.module_layout = ModuleLayout::ModulineIV;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Ads1015;
        } else if hw.contains("Moduline Mini V1.03") {
            self.module_layout = ModuleLayout::ModulineMini;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Ads1015;
        } else if hw.contains("Moduline Mini V1.05") {
            self.module_layout = ModuleLayout::ModulineMini;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004([None,None,None,None]);
        } else if hw.contains("Moduline Mini V1.06") {
            self.module_layout = ModuleLayout::ModulineMini;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004([None,None,None,None]);
        } else if hw.contains("Moduline Mini V1.07") {
            self.module_layout = ModuleLayout::ModulineMini;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004([None,None,None,None]);
        } else if hw.contains("Moduline Mini V1.10") {
            self.module_layout = ModuleLayout::ModulineMini;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004([None,None,None,None]);
        } else if hw.contains("Moduline Screen V1.02") {
            self.module_layout = ModuleLayout::ModulineDisplay;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004([None,None,None,None]);
        } else if hw.contains("Moduline Screen V1.03") {
            self.module_layout = ModuleLayout::ModulineDisplay;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004([None,None,None,None]);
        } else {
            self.module_layout = ModuleLayout::ModulineIV;
            self.led_control = LedControl::Rukr;
            self.adc = AdcConverter::Mcp3004([None,None,None,None]);
        }

        self.adc = self.get_adcs()?;

        self.initialize_leds()?;

        if modules.len() > self.module_layout as usize +1 { panic!("Cannot initialize more than {} modules on this controller", modules.len());}
        for i in 0..self.module_layout as usize {
            self.resets[i] = Some(Self::create_reset(i)?);
        }

        self.init_modules(modules)?;

        modules.iter_mut().try_for_each(|module| -> io::Result<()>{
            module.put_configuration(self)
        })?;

        Ok(())
    }

    pub fn check_module(&mut self, module: &dyn GOcontrollModule) -> io::Result<()> {
        if module.get_slot() as u8 > self.module_layout as u8 {
            println!("Could not initialize module in {}, it doesn't exist on this controller.", module.get_slot());
            return Err(io::Error::from(io::ErrorKind::AddrNotAvailable))
        }

        if self.modules[module.get_slot() as usize].is_none() {
            self.modules[module.get_slot() as usize] = Some(module.get_slot() as usize);
        } else {
            panic!("{} is trying to be occupied by 2 or more modules, check your module initialisation.", module.get_slot())
        }
        Ok(())
    }

    fn init_modules(&mut self, modules: &mut [&mut dyn GOcontrollModule]) -> io::Result<()> {
        
        let mut module_state: [u8;8] = [0;8];
        for module in &mut *modules {
            self.reset_module_state(&(module.get_slot() as usize), ModuleResetState::High)?;
        }
        for module in &mut *modules {
            module.spi_dummy_send()?;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        for module in &mut *modules {
            self.reset_module_state(&(module.get_slot() as usize), ModuleResetState::Low)?;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
        for module in &mut *modules{
        let escape_res = module.escape_module_bootloader()?;
            if escape_res.bootloader == 9 {
                module_state[module.get_slot() as usize] = escape_res.firmware;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        let mut module: usize = 0;
        let mut fault_counter: u8 = 0;
        while module < modules.len() {
            if fault_counter > 5 {
                panic!("module in slot {} is unable to escape the bootloader", module + 1);
            }
            if modules[module].escape_module_bootloader()?.firmware == 20 {
                module += 1;
                fault_counter = 0;
            } else if module_state[module] == 20 {
                self.reset_module_state(&module, ModuleResetState::High)?;
                std::thread::sleep(std::time::Duration::from_millis(5));
                self.reset_module_state(&module, ModuleResetState::Low)?;
                std::thread::sleep(std::time::Duration::from_millis(5));
                modules[module].escape_module_bootloader()?;
                fault_counter += 1;
            }
        }
        Ok(())
    }

    fn reset_module_state(&mut self, slot: &usize, state: ModuleResetState) -> io::Result<()> {
        const STATE: [&str;2] = ["0", "1"];
        self.resets[*slot].as_mut().expect("Incorrectly initialized module reset").write(STATE[state as usize].as_bytes())?;
        Ok(())
    }

    pub fn module_checksum(data:&[u8], length:usize) -> io::Result<u8> {
        let mut check_sum:u8 = 0;
        for index in 0..length-1 {
            check_sum = check_sum.wrapping_add(data[index]);
        }
        if check_sum == data[length-1] {
            return Ok(check_sum)
        } else {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }
    }

    pub fn create_spi(slot: usize) -> io::Result<Spidev> {
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
        fs::File::options()
            .read(false)
            .write(true)
            .open(RESETS[slot])
    }

    fn get_adcs(&self) -> io::Result<AdcConverter> {
        match &self.adc {
            AdcConverter::Ads1015 => {
                Ok(AdcConverter::Ads1015)
            },
            AdcConverter::Mcp3004(adcs) => {
                for device in fs::read_dir("/sys/bus/iio/devices/")? {  
                    let mut dev = device?;   
                    let mut dev_path = (&dev).path();
                    dev_path.push(&dev.file_name());
                    let mut adcs_temp: [Option<PathBuf>;4] = [None,None,None,None];
                    dev_path.set_file_name("name");
                    if fs::read_to_string(&dev_path).unwrap().contains("mcp3004") {
                        for index in 0..4 {
                            dev_path.set_file_name(format!("in_voltage{}_raw",index));
                            adcs_temp[index] = Some(dev_path.clone());
                        }
                        return Ok(AdcConverter::Mcp3004(adcs_temp));
                    }
                }
                Err(io::Error::from(io::ErrorKind::NotFound))
            },
            AdcConverter::None => {
                panic!("get_adcs was called before the main board was initialized, this is not allowed to happen, exitting...");
            }
        }
    }

    pub async fn read_adc_channel(&self, channel: AdcChannel) -> io::Result<u16> {
        match &self.adc {
            AdcConverter::Mcp3004(adcs) => {
                let mut buffer = String::with_capacity(5);
                // let mut adc_file;
                match channel {
                    AdcChannel::K30 => {
                        return Ok(Self::convert_mcp(fs::read_to_string(adcs[3].as_ref().unwrap())?.trim_end()));
                    },
                    AdcChannel::K15A => {
                        return Ok(Self::convert_mcp(fs::read_to_string(adcs[0].as_ref().unwrap())?.trim_end()));
                    },
                    AdcChannel::K15B => {
                        return Ok(Self::convert_mcp(fs::read_to_string(adcs[1].as_ref().unwrap())?.trim_end()));
                    },
                    AdcChannel::K15C => {
                        return Ok(Self::convert_mcp(fs::read_to_string(adcs[2].as_ref().unwrap())?.trim_end()));
                    }
                }
            },
            AdcConverter::Ads1015 => {
                let mut rx: [u8;2] = [0;2];
                let mut tx_config: [u8;2] = [0xf3, 0xe3]; //address of k30 is 0xf3
                let convert: u8 = 0;
                let config: u8 = 1;
                let mut adc_temp = I2c::from_path(ADS_ADC)?;
                adc_temp.smbus_set_slave_address(0x48, false)?;
                match channel {
                    AdcChannel::K30 => {
                        adc_temp.smbus_write_block_data(config,&tx_config)?;
                        adc_temp.smbus_read_block_data(convert, &mut rx)?;
                        return Ok(Self::convert_ads(rx))
                    },
                    AdcChannel::K15A => {
                        tx_config[0] = 0xc3; //address of k15a
                        adc_temp.smbus_write_block_data(config,&tx_config)?;
                        adc_temp.smbus_read_block_data(convert, &mut rx)?;
                        return Ok(Self::convert_ads(rx))
                    },
                    AdcChannel::K15B => {
                        tx_config[0] = 0xd3; //address of k15b
                        adc_temp.smbus_write_block_data(config,&tx_config)?;
                        adc_temp.smbus_read_block_data(convert, &mut rx)?;
                        return Ok(Self::convert_ads(rx))
                    },
                    AdcChannel::K15C => {
                        tx_config[0] = 0xe3; //address of k15c
                        adc_temp.smbus_write_block_data(config,&tx_config)?;
                        adc_temp.smbus_read_block_data(convert, &mut rx)?;
                        return Ok(Self::convert_ads(rx))
                    }
                }
            },
            AdcConverter::None => { panic!("Can't read adc because main board is not initialized yet")}
        }
    }

    fn convert_mcp(string_val: &str) -> u16 {
        if string_val.eq("") {
            return 0;
        } else {
            return (string_val.parse::<f32>().unwrap()*25.54f32) as u16
        }
    }
    fn convert_ads(read_buff:[u8;2]) -> u16 {
        if (read_buff[0] & 0x80) >> 7 == 1 {
            return 0
        } else {
            return ((((read_buff[0] as u16) << 4) | ((read_buff[1] as u16 & 0xf0) >> 4)) as f32 * 15.608f32) as u16
        }
    }

    fn initialize_leds(&self) -> io::Result<()> {
        match &self.led_control {
            LedControl::Rukr => {
                let mut led_temp = I2c::from_path(RUKR_LEDS)?;
                led_temp.smbus_set_slave_address(0x14, false)?;
                led_temp.smbus_write_byte_data(23, 255)?;
                led_temp.smbus_write_byte_data(0, 64)?;
                Ok(())
            },
            _ => {
                Ok(())
            }
        }
    }

    pub async fn set_led(&self, led: EnclosureLed, red: u8, green: u8, blue: u8) -> io::Result<()> {
        match &self.led_control {
            LedControl::Rukr => {
                let mut led_temp = I2c::from_path(RUKR_LEDS)?;
                led_temp.smbus_set_slave_address(0x14, false)?;

                led_temp.smbus_write_byte_data(0x0B+(&(led as u8))*3, red)?;
                led_temp.smbus_write_byte_data(0x0B+(&(led as u8))*3+1, green)?;
                led_temp.smbus_write_byte_data(0x0B+(&(led as u8))*3+2, blue)?;
            },
            LedControl::Gpio => {
                fs::write(GPIO_LEDS[&(led as usize)*3], if red>0 {"1"} else {"0"})?;
                fs::write(GPIO_LEDS[&(led as usize)*3+1], if green>0 {"1"} else {"0"})?;
                fs::write(GPIO_LEDS[&(led as usize)*3+2], if blue>0 {"1"} else {"0"})?;
            },
            LedControl::None => {
                panic!("Cannot set led, either MainBoard::initialize_main_board was not executed yet, or this controller has no available leds.")
            }
        }
        Ok(())
    }
}