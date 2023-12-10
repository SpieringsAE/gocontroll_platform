use std::{io::{self, prelude::*},fs, path::PathBuf,sync::{Arc,Mutex}};
#[cfg(feature = "modules")]
use super::module::{GOcontrollModule,EscapeBootloaderResponse,BOOTMESSAGELENGTH,BOOTMESSAGELENGTHCHECK,CommunicationDirection,MessageType};
#[cfg(feature = "modules")]
use spidev::{Spidev, SpidevOptions,SpiModeFlags};
#[cfg(any(feature = "leds", feature = "adcs"))]
use i2c_linux::I2c;

#[cfg(feature = "leds")]
#[allow(unused)]
#[derive(Debug,Clone)]
pub enum LedControl {
    None,
    Rukr,
    Gpio,
}

#[cfg(feature = "adcs")]
#[allow(unused)]
#[repr(u8)]
pub enum AdcChannel {
    K30=0xf3,
    K15A=0xc3,
    K15B=0xd3,
    K15C=0xe3,
}

#[cfg(feature = "leds")]
#[allow(unused)]
#[repr(u8)]
#[derive(Copy,Clone)]
pub enum EnclosureLed {
    Led1,
    Led2,
    Led3,
    Led4,
}

#[cfg(feature = "modules")]
#[allow(unused)]
#[repr(u8)]
enum ModuleResetState {
    High = 1,
    Low = 0
}

#[cfg(feature = "adcs")]
#[allow(unused)]
#[derive(Debug)]
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
    #[cfg(feature = "leds")]
    led_control: LedControl,
    #[cfg(feature = "adcs")]
    adc: AdcConverter,
    pub module_layout: ModuleLayout,
    #[cfg(feature = "modules")]
    pub modules: [Option<usize>;8],
    #[cfg(feature = "modules")]
    resets: [Option<fs::File>;8],
}

#[cfg(feature = "modules")]
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

#[cfg(feature = "modules")]
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

#[cfg(feature = "leds")]
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

#[cfg(feature = "leds")]
#[allow(unused)]
const RUKR_LEDS: &str = "/dev/i2c-2";

#[cfg(feature = "adcs")]
#[allow(unused)]
const ADS_ADC: &str = "/dev/i2c-2";

#[allow(unused)]
impl MainBoard {
    /// Create a new MainBoard object
    /// 
    /// # Examples
    /// 
    /// ```
    /// use gocontroll_platform::gocontroll::mainboard::MainBoard;
    /// let mut mainboard = MainBoard::new();
    /// ```
    pub const fn new() -> MainBoard {
        MainBoard {
            #[cfg(feature = "leds")]
            led_control: LedControl::None,
            #[cfg(feature = "adcs")]
            adc: AdcConverter::None,
            module_layout: ModuleLayout::None, 
            #[cfg(feature = "modules")]
            modules: [None,None,None,None,None,None,None,None],
            #[cfg(feature = "modules")]
            resets: [None,None,None,None,None,None,None,None],
        }
    }

    pub fn get_hardware_config(&mut self) -> io::Result<()> {
        let hw = fs::read_to_string("/sys/firmware/devicetree/base/hardware")?;
        match hw.as_str() {
            "Moduline IV V3.06" => {
                self.module_layout = ModuleLayout::ModulineIV;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::Rukr;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Mcp3004([None,None,None,None]);}
            },
            "Moduline Mini V1.11" => {
                self.module_layout = ModuleLayout::ModulineMini;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::Rukr;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Mcp3004([None,None,None,None]);}
            },
            "Moduline Screen V1.04" => {
                self.module_layout = ModuleLayout::ModulineDisplay;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::Rukr;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Mcp3004([None,None,None,None]);}
            },
            "Moduline IV V3.00" => {
                self.module_layout = ModuleLayout::ModulineIV;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::Gpio;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Ads1015;}
            },
            "Moduline IV V3.01" => {
                self.module_layout = ModuleLayout::ModulineIV;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::Gpio;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Ads1015;}
            },
            "Moduline IV V3.02" => {
                self.module_layout = ModuleLayout::ModulineIV;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::Rukr;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Ads1015;}
            },
            "Moduline IV V3.03" => {
                self.module_layout = ModuleLayout::ModulineIV;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::Rukr;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Ads1015;}
            },
            "Moduline IV V3.04" => {
                self.module_layout = ModuleLayout::ModulineIV;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::Rukr;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Ads1015;}
            },
            "Moduline IV V3.05" => {
                self.module_layout = ModuleLayout::ModulineIV;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::Rukr;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Ads1015;}
            },
            "Moduline Mini V1.03" => {
                self.module_layout = ModuleLayout::ModulineMini;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::Rukr;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Ads1015;}
            },
            "Moduline Mini V1.05" => {
                self.module_layout = ModuleLayout::ModulineMini;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::Rukr;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Mcp3004([None,None,None,None]);}
            },
            "Moduline Mini V1.06" => {
                self.module_layout = ModuleLayout::ModulineMini;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::Rukr;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Mcp3004([None,None,None,None]);}
            },
            "Moduline Mini V1.07" => {
                self.module_layout = ModuleLayout::ModulineMini;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::Rukr;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Mcp3004([None,None,None,None]);}
            },
            "Moduline Mini V1.10" => {
                self.module_layout = ModuleLayout::ModulineMini;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::None;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Mcp3004([None,None,None,None]);}
            },
            "Moduline Screen V1.02" => {
                self.module_layout = ModuleLayout::ModulineDisplay;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::None;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Mcp3004([None,None,None,None]);}
            },
            "Moduline Screen V1.03" => {
                self.module_layout = ModuleLayout::ModulineDisplay;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::None;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Mcp3004([None,None,None,None]);}
            },
            "Moduline Screen V1.04" => {
                self.module_layout = ModuleLayout::ModulineDisplay;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::None;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Mcp3004([None,None,None,None]);}
            },
            "Moduline Screen V1.05" => {
                self.module_layout = ModuleLayout::ModulineDisplay;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::None;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Mcp3004([None,None,None,None]);}
            },
            _ => {
                self.module_layout = ModuleLayout::ModulineIV;
                #[cfg(feature = "leds")]{
                self.led_control = LedControl::Rukr;}
                #[cfg(feature = "adcs")]{
                self.adc = AdcConverter::Mcp3004([None,None,None,None]);}
            }
        }
        #[cfg(feature = "adcs")] {
            self.adc = self.get_adcs()?;
        }
        #[cfg(feature = "leds")]
        self.initialize_leds()?;

        Ok(())
    }

    #[cfg(feature = "modules")]
    /// Initializes the MainBoard object.
    /// Gets the hardware configuration, puts the configuration in the provided modules, initializes the adcs and leds
    /// 
    /// # Arguments
    /// 
    /// * `modules` - A mutable array of dyn GOcontrollModule structs i.e. modules that need to be initialized aswell.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use gocontroll_platform::gocontroll::{mainboard::MainBoard,inputmodule6ch::*,module::ModuleSlot};
    /// let mut mainboard = MainBoard::new();
    /// let mut input_module: InputModule6Ch = InputModule6Ch::new( ModuleSlot::Moduleslot1,
    /// [
    ///     Some(InputModule6ChConfig::new(InputModule6ChFunction::AnalogmV, InputModule6ChPullDown::PullDown10k, InputModule6ChPullUp::PulUpnNone, InputModule6ChVoltageRange::Voltage0_5V,0u8,10u16)),
    ///     Some(InputModule6ChConfig::new(InputModule6ChFunction::AnalogmV, InputModule6ChPullDown::PullDown10k, InputModule6ChPullUp::PulUpnNone, InputModule6ChVoltageRange::Voltage0_5V,0u8,10u16)),
    ///     None,
    ///     Some(InputModule6ChConfig::new(InputModule6ChFunction::AnalogmV, InputModule6ChPullDown::PullDown10k, InputModule6ChPullUp::PulUpnNone, InputModule6ChVoltageRange::Voltage0_5V,0u8,10u16)),
    ///     None,
    ///     Some(InputModule6ChConfig::new(InputModule6ChFunction::AnalogmV, InputModule6ChPullDown::PullDown10k, InputModule6ChPullUp::PulUpnNone, InputModule6ChVoltageRange::Voltage0_5V,0u8,10u16)),
    /// ],
    /// Inputmodule6chSupplyConfig::new(InputModuleSupply::On, InputModuleSupply::On, InputModuleSupply::On));
    /// mainboard.init(&mut [&mut input_module]);
    /// ```
    pub fn init(&mut self, modules: &mut [&mut dyn GOcontrollModule]) -> io::Result<()>{
        self.get_hardware_config()?;

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
    #[cfg(feature = "modules")]
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
    #[cfg(feature = "modules")]
    fn init_modules(&mut self, modules: & [&mut dyn GOcontrollModule]) -> io::Result<()> {
        
        let mut module_state: [u8;8] = [0;8];
        for module in modules {
            self.reset_module_state(&(module.get_slot() as usize), ModuleResetState::High)?;
        }
        for module in modules {
            Self::spi_dummy_send(*module)?;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        for module in modules {
            self.reset_module_state(&(module.get_slot() as usize), ModuleResetState::Low)?;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
        for module in modules{
        let escape_res = Self::escape_module_bootloader(*module)?;
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
            if Self::escape_module_bootloader(modules[module])?.firmware == 20 {
                module += 1;
                fault_counter = 0;
            } else if module_state[module] == 20 {
                self.reset_module_state(&module, ModuleResetState::High)?;
                std::thread::sleep(std::time::Duration::from_millis(5));
                self.reset_module_state(&module, ModuleResetState::Low)?;
                std::thread::sleep(std::time::Duration::from_millis(5));
                Self::escape_module_bootloader(modules[module])?;
                fault_counter += 1;
            }
        }
        Ok(())
    }
    #[cfg(feature = "modules")]
    fn reset_module_state(&mut self, slot: &usize, state: ModuleResetState) -> io::Result<()> {
        const STATE: [&str;2] = ["0", "1"];
        self.resets[*slot].as_mut().expect("Incorrectly initialized module reset").write_all(STATE[state as usize].as_bytes())?;
        Ok(())
    }
    #[cfg(feature = "modules")]
    pub fn module_checksum(data:&[u8], length:usize) -> io::Result<u8> {
        let mut check_sum:u8 = 0;
        for item in data.iter().take(length-1) {
            check_sum = check_sum.wrapping_add(*item);
        }
        if check_sum == data[length-1] {
            Ok(check_sum)
        } else {
            Err(io::Error::from(io::ErrorKind::InvalidData))
        }
    }
    #[cfg(feature = "modules")]
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
    #[cfg(feature = "modules")]
    fn create_reset(slot: usize) -> io::Result<fs::File> {
        fs::File::options()
            .read(false)
            .write(true)
            .open(RESETS[slot])
    }
    #[cfg(feature = "adcs")]
    fn get_adcs(&self) -> io::Result<AdcConverter> {
        match &self.adc {
            AdcConverter::Ads1015 => {
                Ok(AdcConverter::Ads1015)
            },
            AdcConverter::Mcp3004(adcs) => {
                for device in fs::read_dir("/sys/bus/iio/devices/")? {  
                    let mut dev = device?;   
                    let mut dev_path = dev.path();
                    dev_path.push(&dev.file_name());
                    let mut adcs_temp: [Option<PathBuf>;4] = [None,None,None,None];
                    dev_path.set_file_name("name");
                    if fs::read_to_string(&dev_path).unwrap().contains("mcp3004") {
                        for (index, adc) in adcs_temp.iter_mut().enumerate() {
                            dev_path.set_file_name(format!("in_voltage{}_raw",index));
                            *adc = Some(dev_path.clone());
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

    #[cfg(feature = "adcs")]
    /// Reads from one of the 4 ADC channels
    /// 
    /// # Arguments
    /// 
    /// * `channel` - The ADC channel to read out
    /// 
    /// # Examples
    /// 
    /// ```
    /// use gocontroll_platform::gocontroll::mainboard::{MainBoard,AdcChannel};
    /// use futures::future::join;
    /// let mut mainboard = MainBoard::new();
    /// mainboard.init(&mut []);
    /// join(mainboard.read_adc_channel(AdcChannel::K30)).unwrap();
    /// ```
    pub fn read_adc_channel(&self, channel: AdcChannel) -> io::Result<u16> {
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
                let mut tx_config: [u8;3] = [0x01, channel as u8, 0xe3]; //address of k30 is 0xf3
                let convert: u8 = 0x00;
                let config: u8 = 0x01;
                let mut adc_temp = I2c::from_path(ADS_ADC)?;
                adc_temp.smbus_set_slave_address(0x48, false)?;
                adc_temp.write_all(&tx_config)?;
                adc_temp.write_all(&[convert])?;
                adc_temp.read_exact(&mut rx)?;
                Ok(Self::convert_ads(rx))
            },
            AdcConverter::None => { panic!("Can't read adc because main board is not initialized yet")}
        }
    }

    #[cfg(feature = "adcs")]
    fn convert_mcp(string_val: &str) -> u16 {
        if string_val.eq("") {
            0
        } else {
            (string_val.parse::<f32>().unwrap()*25.54f32) as u16
        }
    }
    #[cfg(feature = "adcs")]
    fn convert_ads(read_buff:[u8;2]) -> u16 {
        if (read_buff[0] & 0x80) >> 7 == 1 {
            0
        } else {
            ((((read_buff[0] as u16) << 4) | ((read_buff[1] as u16 & 0xf0) >> 4)) as f32 * 15.608f32) as u16
        }
    }
    #[cfg(feature = "leds")]
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
    #[cfg(feature = "leds")]
    pub fn set_led(&self, led: EnclosureLed, red: u8, green: u8, blue: u8) -> io::Result<()> {
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
    #[cfg(feature = "modules")]
    pub fn send_module_spi(spidev: Arc<Mutex<Spidev>>, command: u8, direction: CommunicationDirection, module_id: u8, message_type: MessageType, message_index: u8, tx:&mut [u8], length:usize) -> io::Result<()> {
        tx[0] = command;
        tx[1] = {length-1} as u8;
        tx[2] = direction as u8;
        tx[3] = module_id;
        tx[4] = message_type as u8;
        tx[5] = message_index;
        tx[length-1] = MainBoard::module_checksum(&tx, length)?;
        let mut transfer = spidev::SpidevTransfer::write(&tx);
        spidev.lock().as_mut().unwrap().transfer(&mut transfer)?;
        Ok(())
    }
    #[cfg(feature = "modules")]
    pub fn send_receive_module_spi(spidev: Arc<Mutex<Spidev>>, command: u8, direction: CommunicationDirection, module_id: u8, message_type: MessageType, message_index: u8, tx:&mut [u8], rx:&mut [u8], length:usize) -> io::Result<()> {
        tx[0] = command;
        tx[1] = {length-1} as u8;
        tx[2] = direction as u8;
        tx[3] = module_id;
        tx[4] = message_type as u8;
        tx[5] = message_index;
        tx[length-1] = MainBoard::module_checksum(&tx, length)?;
        rx[0] = 0;
        rx[length-1] = 0;

        let mut transfer = spidev::SpidevTransfer::read_write(&tx,rx);
        spidev.lock().as_mut().unwrap().transfer(&mut transfer)?;
        MainBoard::module_checksum(&rx, length)?;
        Ok(())        
    }
    #[cfg(feature = "modules")]
    pub fn spi_dummy_send(module: &dyn GOcontrollModule) -> io::Result<()> {
        const SPIDUMMY: [u8;6] = [1,2,3,4,5,6];
        let mut transfer = spidev::SpidevTransfer::write(&SPIDUMMY);
        module.get_spidev().lock().as_mut().unwrap().transfer(&mut transfer)?;
        Ok(())
    }
    #[cfg(feature = "modules")]
    pub fn escape_module_bootloader(module: &dyn GOcontrollModule) ->io::Result<EscapeBootloaderResponse> {
        let mut tx: [u8;BOOTMESSAGELENGTHCHECK] = [0;BOOTMESSAGELENGTHCHECK];
        let mut rx: [u8;BOOTMESSAGELENGTHCHECK] = [0;BOOTMESSAGELENGTHCHECK];
        tx[0] = 19;
        tx[1] = {BOOTMESSAGELENGTH -1} as u8;
        tx[2] = 19;
        tx[BOOTMESSAGELENGTH-1] = MainBoard::module_checksum(&tx, BOOTMESSAGELENGTH)?;
        let mut transfer = spidev::SpidevTransfer::read_write(&tx, &mut rx);
        
        module.get_spidev().lock().as_mut().unwrap().transfer(&mut transfer)?;
        MainBoard::module_checksum(&rx, BOOTMESSAGELENGTH)?;
        Ok(EscapeBootloaderResponse{ bootloader: rx[0], firmware: rx[6]})
    }
}