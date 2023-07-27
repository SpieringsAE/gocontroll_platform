use core::fmt::{Debug,Display};

use super::mainboard::MainBoard;


///////////////////////////////////////////////////////

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

impl Display for ModuleSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Module slot {}", *self as u8 +1)
    }
}

/// The trait for modules, if you have designed your own module and wish to use it, implement this trait for it and it will be accepted.
pub trait GOcontrollModule: Send + Sync {
    /// Initializes the module.
    fn put_configuration(&mut self, mainboard: &mut MainBoard) -> Result<(),()>;

    fn get_slot(&self) -> ModuleSlot;
}