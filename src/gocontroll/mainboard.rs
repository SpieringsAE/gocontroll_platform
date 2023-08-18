use std::{io::{self, prelude::*},fs};
use super::module::{GOcontrollModule, CommunicationDirection, MessageType};
use spidev::{Spidev, SpidevOptions, SpidevTransfer,SpiModeFlags};

#[allow(unused)]
#[derive(Debug,Clone)]
pub enum LedControl {
    None,
    Rukr,
    Gpio,
}

#[allow(unused)]
#[repr(u8)]
enum ModuleResetState {
    High = 1,
    Low = 0
}

#[allow(unused)]
#[derive(Debug,Clone)]
pub enum AdcConverter {
    None,
    Mcp3004,
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
    spidevs: [Option<Spidev>;8],
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
const BOOTMESSAGELENGTHCHECK: usize = 61;
#[allow(unused)]
const BOOTMESSAGELENGTH: usize = 46;
#[allow(unused)]
const MESSAGEOVERLENGTH: usize = 1;
#[allow(unused)]
const SPIERRORMESSAGE: &str = "Incorrectly initialized spi device";

#[allow(unused)]
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

    pub fn initialize_main_board(&mut self, modules: &mut [&mut dyn GOcontrollModule]) -> io::Result<()>{
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
        self.init_modules()?;
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

    pub fn init_modules(&mut self) -> io::Result<()> {
        let mut boottx: [u8;BOOTMESSAGELENGTHCHECK] = [0;BOOTMESSAGELENGTHCHECK];
        let mut bootrx: [u8;BOOTMESSAGELENGTHCHECK] = [0;BOOTMESSAGELENGTHCHECK];
        let mut module_state: [u8;8] = [0;8];
        let iter_thing = self.modules.clone();
        let module_iter: Vec<&usize> = iter_thing.iter().flatten().collect();
        for slot in &module_iter {
            self.reset_module_state(slot, ModuleResetState::High)?;
        }
        for slot in &module_iter {
            self.spi_dummy_send(slot)?;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        for slot in &module_iter {
            self.reset_module_state(slot, ModuleResetState::Low)?;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
        // for slot in &module_iter {
        for slot in &module_iter{
            boottx[6] = 0;
            self.escape_module_bootloader(slot, &mut boottx, &mut bootrx)?;
            if bootrx[0] == 9 {
                module_state[**slot] = bootrx[6];
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        let mut module: usize = 0;
        let mut fault_counter: u8 = 0;
        while module < module_iter.len() {
            bootrx[6] = 0;
            self.escape_module_bootloader(&module, &mut boottx, &mut bootrx)?;
            if fault_counter > 5 {
                panic!("module {module} is unable to escape the bootloader");
            }
            if bootrx[6] == 20 {
                module += 1;
                fault_counter = 0;
            } else if module_state[module] == 20 {
                self.reset_module_state(&module, ModuleResetState::High)?;
                std::thread::sleep(std::time::Duration::from_millis(5));
                self.reset_module_state(&module, ModuleResetState::Low)?;
                std::thread::sleep(std::time::Duration::from_millis(5));
                bootrx[6] = 0;
                self.escape_module_bootloader(&module, &mut boottx, &mut bootrx)?;
                fault_counter += 1;
            }
        }
        Ok(())
    }

    fn spi_dummy_send(&mut self, slot: &usize) -> io::Result<()> {
        const SPIDUMMY: [u8;6] = [1,2,3,4,5,6];
        self.spidevs[*slot].as_mut().expect(SPIERRORMESSAGE).write(&SPIDUMMY)?;
        Ok(())
    }

    fn reset_module_state(&mut self, slot: &usize, state: ModuleResetState) -> io::Result<()> {
        const STATE: [&str;2] = ["0", "1"];
        self.resets[*slot].as_mut().expect("Incorrectly initialized module reset").write(STATE[state as usize].as_bytes())?;
        Ok(())
    }

    fn escape_module_bootloader(&mut self, slot: &usize, tx:&mut [u8], rx: &mut [u8]) ->io::Result<()> {

        tx[0] = 19;
        tx[1] = {BOOTMESSAGELENGTH -1} as u8;
        tx[2] = 19;
        tx[BOOTMESSAGELENGTH-1] = Self::module_checksum(tx, BOOTMESSAGELENGTH)?;
        let mut transfer = SpidevTransfer::read_write(&tx, rx);
        
        self.spidevs[*slot].as_mut().expect(SPIERRORMESSAGE).transfer(&mut transfer)?;
        Self::module_checksum(rx, BOOTMESSAGELENGTH)?;
        Ok(())
    }

    fn module_checksum(data:&[u8], length:usize) -> io::Result<u8> {
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

    pub fn send_module_spi(&mut self, command: u8, direction: CommunicationDirection, module_id: u8, message_type: MessageType, message_index: u8, slot: usize, tx:&mut [u8], length:usize) -> io::Result<()> {
        tx[0] = command;
        tx[1] = {length-1} as u8;
        tx[2] = direction as u8;
        tx[3] = module_id;
        tx[4] = message_type as u8;
        tx[5] = message_index;
        tx[length-1] = Self::module_checksum(&tx, length)?;
        let mut transfer = SpidevTransfer::write(&tx);
        self.spidevs[slot].as_mut().expect(SPIERRORMESSAGE).transfer(&mut transfer)?;
        Ok(())
    }

    pub fn send_receive_module_spi(&mut self, command: u8, direction: CommunicationDirection, module_id: u8, message_type: MessageType, message_index: u8, slot: usize, tx:&mut [u8], rx:&mut [u8], length:usize) -> io::Result<()> {
        tx[0] = command;
        tx[1] = {length-1} as u8;
        tx[2] = direction as u8;
        tx[3] = module_id;
        tx[4] = message_type as u8;
        tx[5] = message_index;
        tx[length-1] = Self::module_checksum(&tx, length)?;
        
        rx[0] = 0;
        rx[length-1] = 0;

        let mut transfer = SpidevTransfer::read_write(&tx,rx);
        self.spidevs[slot].as_mut().expect(SPIERRORMESSAGE).transfer(&mut transfer)?;
        Self::module_checksum(&rx, length)?;
        Ok(())        
    }
}