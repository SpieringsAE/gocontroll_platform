use std::io;
use std::sync::{Arc,Mutex};

use spidev::Spidev;

use super::{
	module::{
		GOcontrollModule,
		ModuleSlot,
		CommunicationDirection,
		MessageType
	},
	mainboard::MainBoard
};

#[cfg("async")]
use tokio::{
	task::spawn_blocking,
	
};

#[allow(unused)]
#[repr(u8)]
#[derive(Debug,Copy, Clone)]
/// Input module channel functions:
/// None            -> Channel is unused\
/// Adc12bit        -> Read the raw ADC value\
/// AnalogmV        -> ADC converted to mV\
/// DigitalIn       -> Read pin as a digital input high or low\
/// FrequencyIn     -> Measure the frequency of the incoming signal\
/// DutyCycleLow    -> Measure the low period duty cycle\
/// DutyCycleHigh   -> Measure the high period duty cycle\
/// Rpm             -> PulseCounter converted to RPM with a set pulses per rotation configuration\
/// PulseCounter    -> Count the pulses on an input
pub enum InputModule6ChFunction {
	None = 0,
	Adc12Bit = 1,
	AnalogmV = 2,
	DigitalIn = 3,
	FrequencyIn = 4,
	DutyCycleLow = 5,
	DutyCycleHigh = 6,
	Rpm = 7,
	PulseCounter = 8,
}

#[allow(unused)]
#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum InputModule6ChPullDown {
	PullDownNone = 0,
	PullDown3_3k = 3,
	PullDown4_7k = 1,
	PullDown10k = 2,
}

#[allow(unused)]
#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum InputModule6ChPullUp {
	PulUpnNone = 0,
	PullUp3_3k = 3,
	PullUp4_7k = 1,
	PullUp10k = 2,
}

#[allow(unused)]
#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum InputModule6ChVoltageRange {
	Voltage0_5V = 0,
	Voltage0_12V = 1,
	Voltage0_24V = 2,
}

#[allow(unused)]
#[repr(u8)]
#[derive(Debug,Copy, Clone)]
pub enum InputModuleSupply{
	On = 1,
	Off = 2,
}

#[allow(unused)]
#[repr(u8)]
pub enum InputModuleChannel {
	Channel1 = 0u8,
	Channel2 = 1u8,
	Channel3 = 2u8,
	Channel4 = 3u8,
	Channel5 = 4u8,
	Channel6 = 5u8,
}

const MODULEID:u8 =11;
const MESSAGELENGTH:usize = 55;

#[allow(unused)]
#[derive(Debug)]
pub struct InputModule6Ch {
	slot: ModuleSlot,
	pulse_counter_reset: [u8; 6],
	sync_counter: [u32; 6],
	tx: [u8;56],
	rx: [u8;56],
	spidev: Option<Arc<Mutex<Spidev>>>,
}

#[allow(unused)]
impl InputModule6Ch {
	/// Create a new 6 channel input module object
	/// 
	/// # Arguments
	/// 
	/// * `slot` - The module slot that the module occupies
	/// * `channels` - An array with all the channel configurations
	/// * `supply` - The configuration of the module sensor supplies
	/// 
	/// # Examples
	/// 
	/// ```
	/// let mut input_module: InputModule6Ch = InputModule6Ch::new( ModuleSlot::Moduleslot1,
	/// [
	///     Some(InputModule6ChConfig::new(InputModule6ChFunction::AnalogmV, InputModule6ChPullDown::PullDown10k, InputModule6ChPullUp::PulUpnNone, InputModule6ChVoltageRange::Voltage0_5V,0u8,10u16)),
	///     None,
	///     None,
	///     None,
	///     None,
	///     None
	/// ],
	/// Inputmodule6chSupplyConfig::new(InputModuleSupply::On, InputModuleSupply::On, InputModuleSupply::On));
	/// ```
	pub const fn new(slot: ModuleSlot, channels: [Option<InputModule6ChConfig>;6],
	supply: Inputmodule6chSupplyConfig ) -> InputModule6Ch {
		let mut tx_data = [0u8;56];
		let mut index = 0;
		while index < 6 {
			match channels[index] {
				Some(config)=> {
					tx_data[index*6] = config.function as u8;
					tx_data[index*6+1] = config.pull_up as u8 | (config.pull_down as u8) << 2 | (config.input_voltage as u8) << 6;
					match config.function {
						InputModule6ChFunction::None => (),
						InputModule6ChFunction::Adc12Bit | InputModule6ChFunction::AnalogmV => {
							tx_data[index*6+2] = (config.analog_filter_samples >> 8) as u8;
							tx_data[index*6+3] = config.analog_filter_samples as u8;
						},
						_ => {
							tx_data[index*6+2] = config.pulses_per_rotation;
						}
					}
					index +=1;
				},
				None => {index+=1;}
			}
		}

		tx_data[42] = supply.sensor_supplies[0] as u8;
		tx_data[43] = supply.sensor_supplies[1] as u8;
		tx_data[44] = supply.sensor_supplies[2] as u8;
		InputModule6Ch {
			slot,
			pulse_counter_reset : [0u8; 6],
			sync_counter : [0u32;6],
			tx: tx_data,
			rx: [0u8;56],
			spidev: None,
		}
	}

	#[cfg(feature="async")]
	pub async fn get_values_async(&self) -> io::Result<[i32;6]> {
		let mut result: [i32;6] = [0;6];
		let mut tx:[u8;56] = [0;56];
		let mut rx:[u8;56] = [0;56];
		MainBoard::send_receive_module_spi(
			self.get_spidev(),
			1,
			CommunicationDirection::FromModule,
			MODULEID,
			MessageType::Data,
			1,
			&mut tx,
			&mut rx,
			MESSAGELENGTH
		)?;
		for i in 0..5 {
			result[i] = i32::from_le_bytes(rx[i*8+6..i*8+10].try_into().unwrap());
		}
		Ok(result)
	}

	pub fn get_values_sync(&self) -> io::Result<[i32;6]> {
		let mut result: [i32;6] = [0;6];
		let mut tx:[u8;56] = [0;56];
		let mut rx:[u8;56] = [0;56];
		MainBoard::send_receive_module_spi(
			self.get_spidev(),
			1,
			CommunicationDirection::FromModule,
			MODULEID,
			MessageType::Data,
			1,
			&mut tx,
			&mut rx,
			MESSAGELENGTH
		)?;
		for i in 0..5 {
			result[i] = i32::from_le_bytes(rx[i*8+6..i*8+10].try_into().unwrap());
		}
		Ok(result)
	}

	pub fn get_values(&self) -> io::Result<[i32;6]> {
		let mut result: [i32;6] = [0;6];
		let mut tx:[u8;56] = [0;56];
		let mut rx:[u8;56] = [0;56];
		MainBoard::send_receive_module_spi(
			self.get_spidev(),
			1,
			CommunicationDirection::FromModule,
			MODULEID,
			MessageType::Data,
			1,
			&mut tx,
			&mut rx,
			MESSAGELENGTH
		)?;
		for i in 0..5 {
			result[i] = i32::from_le_bytes(rx[i*8+6..i*8+10].try_into().unwrap());
		}
		Ok(result)
	}

	pub fn reset_pulse_counter(&self, channel: InputModuleChannel, value: i32) -> io::Result<()> {
		let mut tx:[u8;56] = [0;56];
		tx[6] = channel as u8;
		tx[7] = value as u8;
		tx[8] = {value >> 8} as u8;
		tx[9] = {value >> 16} as u8;
		tx[10] = {value >> 24} as u8;
		MainBoard::send_module_spi(
			self.get_spidev(),
			1,
			CommunicationDirection::ToModule,
			MODULEID,
			MessageType::Data,
			2,
			&mut tx,
			MESSAGELENGTH
		)
	}
}

impl GOcontrollModule for InputModule6Ch {
	fn put_configuration(&mut self, mainboard: &mut MainBoard) -> io::Result<()>{

		mainboard.check_module(self)?;

		self.spidev = Some(Arc::new(Mutex::new(MainBoard::create_spi(self.slot as usize)?)));
		
		MainBoard::send_module_spi(
			self.get_spidev(),
			1,
			CommunicationDirection::ToModule,
			MODULEID,
			MessageType::Configuration,
			1,
			&mut self.tx,
			MESSAGELENGTH
		)
	}
	fn get_slot(&self) -> ModuleSlot {
		self.slot
	}

	fn get_spidev(&self) -> Arc<Mutex<Spidev>> {
		self.spidev.as_ref().unwrap().clone()
	}
}

#[derive(Debug,Copy, Clone)]
pub struct InputModule6ChConfig {
	function: InputModule6ChFunction,
	pull_down: InputModule6ChPullDown,
	pull_up: InputModule6ChPullUp,
	input_voltage: InputModule6ChVoltageRange,
	pulses_per_rotation: u8,
	analog_filter_samples:u16
}

#[allow(unused)]
impl InputModule6ChConfig {
	pub const fn new(function: InputModule6ChFunction, pull_down: InputModule6ChPullDown, pull_up: InputModule6ChPullUp,
	input_voltage: InputModule6ChVoltageRange, pulses_per_rotation: u8, analog_filter_samples: u16) -> InputModule6ChConfig {
		InputModule6ChConfig {function, pull_down, pull_up, input_voltage, pulses_per_rotation, analog_filter_samples}
	}
}

#[derive(Debug,Copy, Clone)]
pub struct Inputmodule6chSupplyConfig {
	sensor_supplies: [InputModuleSupply; 3],
}

#[allow(unused)]
impl Inputmodule6chSupplyConfig {
	pub const fn new(supply1: InputModuleSupply, supply2: InputModuleSupply, supply3: InputModuleSupply) -> Inputmodule6chSupplyConfig {
		Inputmodule6chSupplyConfig { sensor_supplies: [supply1,supply2,supply3] }
	}
}