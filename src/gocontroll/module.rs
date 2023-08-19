use core::fmt::{Debug,Display};
use std::io;

use spidev::Spidev;

use super::mainboard::MainBoard;


///////////////////////////////////////////////////////

#[allow(unused)]
pub const BOOTMESSAGELENGTHCHECK: usize = 61;
#[allow(unused)]
pub const BOOTMESSAGELENGTH: usize = 46;
#[allow(unused)]
pub const MESSAGEOVERLENGTH: usize = 1;
#[allow(unused)]
pub const SPIERRORMESSAGE: &str = "Incorrectly initialized spi device";

#[allow(unused)]
#[repr(u8)]
#[derive(Copy,Clone,Debug)]
/// All potentially available module slots, code that tries to configure a module on a slot that doesn't exist (slot 5 on a mini for example) will panic upon initialization.
pub enum ModuleSlot {
    Moduleslot1 = 0u8,
    Moduleslot2 = 1u8,
    Moduleslot3 = 2u8,
    Moduleslot4 = 3u8,
    Moduleslot5 = 4u8,
    Moduleslot6 = 5u8,
    Moduleslot7 = 6u8,
    Moduleslot8 = 7u8,
}

#[allow(unused)]
#[repr(u8)]
pub enum CommunicationDirection {
    ToModule = 1u8,
    FromModule = 2u8,
}

#[allow(unused)]
#[repr(u8)]
pub enum MessageType {
    ModuleId = 1u8,
    Configuration = 2u8,
    Data = 3u8,
    Feedback = 4u8,
}

impl Display for ModuleSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Module slot {}", *self as u8 +1)
    }
}

/// The trait for modules, if you have designed your own module and wish to use it, implement this trait for it and it will be accepted.
pub trait GOcontrollModule: Send + Sync {
    /// Initializes the module.
    fn put_configuration(&mut self, mainboard: &mut MainBoard) -> io::Result<()>;

    fn get_slot(&self) -> ModuleSlot;

    fn send_module_spi(&mut self, command: u8, direction: CommunicationDirection, module_id: u8, message_type: MessageType, message_index: u8, length:usize) -> io::Result<()>;

    fn send_receive_module_spi(&mut self, command: u8, direction: CommunicationDirection, module_id: u8, message_type: MessageType, message_index: u8, length:usize) -> io::Result<()>;

    fn escape_module_bootloader(&mut self) ->io::Result<()>;

    fn spi_dummy_send(&mut self) -> io::Result<()>;
}