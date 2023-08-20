use std::io;

use spidev::Spidev;

use super::{module::{GOcontrollModule,ModuleSlot,MessageType,CommunicationDirection,BOOTMESSAGELENGTH, BOOTMESSAGELENGTHCHECK, EscapeBootloaderRespnse},
    mainboard::MainBoard};

    #[allow(unused)]
#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum OutputModule6ChFunction {
    None = 1,
    HalfBridge = 2,
    LowSideDutyCycle = 3,
    HighSideDutyCycle = 4,
    LowSideSwitch = 5,
    HighSideSwitch = 6,
    PeakAndHold = 7,
    FrequencyOut = 8,
}

#[allow(unused)]
#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum OutputModule6ChFrequency {
    Freq100Hz = 1,
    Freq200Hz = 2,
    Freq500Hz = 3,
    Freq1KHz = 4,
    Freq2KHz = 5,
    Freq5KHz = 6,
    Freq10KHz = 7,
}

#[allow(unused)]
#[derive(Debug,Copy, Clone)]
pub struct OutputModule6ChConfig {
    function: OutputModule6ChFunction,
    peak_current: Option<u16>,
    peak_time: Option<u16>,
    max_current: Option<u16>,
}

#[allow(unused)]
impl OutputModule6ChConfig {
    pub const fn new(function: OutputModule6ChFunction, max_current: Option<u16>, peak_current: Option<u16>, peak_time: Option<u16>)-> OutputModule6ChConfig {
        OutputModule6ChConfig {function, max_current, peak_current, peak_time}
    }
}

#[allow(unused)]
#[derive(Debug,Copy, Clone)]
pub struct OutputModule6ChFrequecyConfig {
    frequencies: [OutputModule6ChFrequency;3],
}

#[allow(unused)]
impl OutputModule6ChFrequecyConfig {
    pub const fn new(channel1_2: OutputModule6ChFrequency, channel3_4: OutputModule6ChFrequency, channel5_6: OutputModule6ChFrequency) -> OutputModule6ChFrequecyConfig {
        OutputModule6ChFrequecyConfig {frequencies: [channel1_2,channel3_4,channel5_6]}
    }
}

#[allow(unused)]
pub struct OutputModule6ChFeedback {
    temperature: i16,
    groundshift: u16,
    channel1_current: i16,
    channel2_current: i16,
    channel3_current: i16,
    channel4_current: i16,
    channel5_current: i16,
    channel6_current: i16,
    fault_codes: u32,
}

const MODULEID:u8 =22;
const MESSAGELENGTH:usize = 44;

#[allow(unused)]
#[derive(Debug)]
pub struct OutputModule6Ch {
    slot: ModuleSlot,
    tx_data: [u8;50],
    tx_data_2: [u8;50],
    rx_data: [u8;50],
    spidev: Option<Spidev>,
}

#[allow(unused)]
impl OutputModule6Ch {
    pub const fn new(slot: ModuleSlot, channels: [Option<OutputModule6ChConfig>;6], frequency_channels: OutputModule6ChFrequecyConfig) -> OutputModule6Ch {
        let mut tx_data = [0u8;50];
        let mut tx_data_2 = [0u8;50];
        let mut index = 0;
        while index < 6 {
            match channels[index] {
                Some(config) => {
                    tx_data[index + 6] = {config.function as u8} << 4 | frequency_channels.frequencies[index/2] as u8;
                    match config.max_current {
                        Some(max) => {
                            tx_data[index*2+12] = max as u8;
                            tx_data[index*2+13] = {max >> 8} as u8;
                        },
                        None => {
                            tx_data[index*2+12] = 255;
                            tx_data[index*2+13] = 255;
                        }
                    }
                    match config.peak_current {
                        Some(peak_curr) => {
                            tx_data_2[index*2+6] = peak_curr as u8;
                            tx_data_2[index*2+7] = {peak_curr >> 8} as u8;
                        },
                        None => {
                            tx_data_2[index*2+6] = 0;
                            tx_data_2[index*2+7] = 0;
                        }
                    }
                    match config.peak_time {
                        Some(peak_t) => {
                            tx_data_2[index*2+18] = peak_t as u8;
                            tx_data_2[index*2+19] = {peak_t >> 8} as u8;
                        },
                        None => {
                            tx_data_2[index*2+18] = 0;
                            tx_data_2[index*2+19] = 0;
                        }
                    }
                    index += 1;
                },
                None => {index +=1}
            }
        }
        OutputModule6Ch {slot, tx_data, tx_data_2, rx_data: [0u8;50], spidev: None}
    }

    pub async fn set_outputs_get_feedback(&mut self, channel1: u16, channel2: u16, channel3: u16, channel4: u16, channel5: u16, channel6:u16) -> io::Result<OutputModule6ChFeedback> {
        let mut feedback: OutputModule6ChFeedback = OutputModule6ChFeedback { temperature: 0, groundshift: 0, channel1_current: 0, channel2_current: 0, channel3_current: 0, channel4_current: 0, channel5_current: 0, channel6_current: 0, fault_codes: 0x10000000 };
        let mut potential_err: Option<io::Error> = None;  
        self.tx_data[6] = channel1 as u8;
        self.tx_data[7] = {channel1 >> 8} as u8;
        self.tx_data[12] = channel2 as u8;
        self.tx_data[13] = {channel2 >> 8} as u8;
        self.tx_data[18] = channel3 as u8;
        self.tx_data[19] = {channel3 >> 8} as u8;
        self.tx_data[24] = channel4 as u8;
        self.tx_data[25] = {channel4 >> 8} as u8;
        self.tx_data[30] = channel5 as u8;
        self.tx_data[31] = {channel5 >> 8} as u8;
        self.tx_data[36] = channel6 as u8;
        self.tx_data[37] = {channel6 >> 8} as u8;

        self.send_receive_module_spi(
            1, 
            CommunicationDirection::ToModule,
            MODULEID,
            MessageType::Data,
            1,
            MESSAGELENGTH
        ).is_err_and(|err| if err.kind() == io::ErrorKind::InvalidData {
            feedback.fault_codes |= 0x20000000;
            true
        } else {
            potential_err = Some(err);
            false
        });
        if potential_err.is_some() {
            return Err(potential_err.unwrap());
        }
        feedback.temperature = i16::from_le_bytes(self.rx_data[6..7].try_into().unwrap());
        feedback.groundshift = u16::from_le_bytes(self.rx_data[8..9].try_into().unwrap());
        feedback.fault_codes = u32::from_le_bytes(self.rx_data[22..25].try_into().unwrap());
        feedback.channel1_current = i16::from_le_bytes(self.rx_data[10..11].try_into().unwrap());
        feedback.channel2_current = i16::from_le_bytes(self.rx_data[12..13].try_into().unwrap());
        feedback.channel3_current = i16::from_le_bytes(self.rx_data[14..15].try_into().unwrap());
        feedback.channel4_current = i16::from_le_bytes(self.rx_data[16..17].try_into().unwrap());
        feedback.channel5_current = i16::from_le_bytes(self.rx_data[18..19].try_into().unwrap());
        feedback.channel6_current = i16::from_le_bytes(self.rx_data[20..21].try_into().unwrap());
        Ok(feedback)
    }
}

impl GOcontrollModule for OutputModule6Ch {
    fn put_configuration(&mut self, mainboard: &mut MainBoard) -> io::Result<()> {
        mainboard.check_module(self)?;

        self.send_module_spi(
            1,
            CommunicationDirection::ToModule,
            MODULEID,
            MessageType::Configuration,
            1,
            MESSAGELENGTH
        )?;
        self.tx_data = self.tx_data_2;
        std::thread::sleep(std::time::Duration::from_micros(500));
        self.send_module_spi(
            1,
            CommunicationDirection::ToModule,
            MODULEID,
            MessageType::Configuration,
            2,
            MESSAGELENGTH
        )        
    }

    fn get_slot(&self) -> ModuleSlot {
        self.slot
    }

    fn spi_dummy_send(&mut self) -> io::Result<()> {
        const SPIDUMMY: [u8;6] = [1,2,3,4,5,6];
        let mut transfer = spidev::SpidevTransfer::write(&SPIDUMMY);
        self.spidev.as_mut().unwrap().transfer(&mut transfer)?;
        Ok(())
    }

    fn escape_module_bootloader(&mut self) ->io::Result<EscapeBootloaderRespnse> {
        let mut tx: [u8;BOOTMESSAGELENGTHCHECK] = [0;BOOTMESSAGELENGTHCHECK];
        let mut rx: [u8;BOOTMESSAGELENGTHCHECK] = [0;BOOTMESSAGELENGTHCHECK];
        tx[0] = 19;
        tx[1] = {BOOTMESSAGELENGTH -1} as u8;
        tx[2] = 19;
        tx[BOOTMESSAGELENGTH-1] = MainBoard::module_checksum(&tx, BOOTMESSAGELENGTH)?;
        let mut transfer = spidev::SpidevTransfer::read_write(&tx, &mut rx);
        
        self.spidev.as_mut().unwrap().transfer(&mut transfer)?;
        MainBoard::module_checksum(&rx, BOOTMESSAGELENGTH)?;
        Ok(EscapeBootloaderRespnse{ bootloader: rx[0], firmware: rx[6]})
    }

    fn send_module_spi(&mut self, command: u8, direction: CommunicationDirection, module_id: u8, message_type: MessageType, message_index: u8, length:usize) -> io::Result<()> {
        self.tx_data[0] = command;
        self.tx_data[1] = {length-1} as u8;
        self.tx_data[2] = direction as u8;
        self.tx_data[3] = module_id;
        self.tx_data[4] = message_type as u8;
        self.tx_data[5] = message_index;
        self.tx_data[length-1] = MainBoard::module_checksum(&self.tx_data, length)?;
        let mut transfer = spidev::SpidevTransfer::write(&self.tx_data);
        self.spidev.as_mut().unwrap().transfer(&mut transfer)?;
        Ok(())
    }

    fn send_receive_module_spi(&mut self, command: u8, direction: CommunicationDirection, module_id: u8, message_type: MessageType, message_index: u8, length:usize) -> io::Result<()> {
        self.tx_data[0] = command;
        self.tx_data[1] = {length-1} as u8;
        self.tx_data[2] = direction as u8;
        self.tx_data[3] = module_id;
        self.tx_data[4] = message_type as u8;
        self.tx_data[5] = message_index;
        self.tx_data[length-1] = MainBoard::module_checksum(&self.tx_data, length)?;
        self.rx_data[0] = 0;
        self.rx_data[length-1] = 0;

        let mut transfer = spidev::SpidevTransfer::read_write(&self.tx_data,&mut self.rx_data);
        self.spidev.as_mut().unwrap().transfer(&mut transfer)?;
        MainBoard::module_checksum(&self.rx_data, length)?;
        Ok(())        
    }
}