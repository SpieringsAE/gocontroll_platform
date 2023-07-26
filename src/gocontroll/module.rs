use core::fmt::Debug;

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

/// The trait for modules, if you have designed your own module and wish to use it, implement this trait for it and it will be accepted.
pub trait GOcontrollModule: Send + Sync {
    /// This function sends the module configuration over the spi bus to the module, it is called by MainBoard::configure_modules().
    fn put_configuration(&mut self);
    /// Simple getter for the module slot.
    fn get_slot(&self)->ModuleSlot;
}
// impl Debug for dyn GOcontrollModule {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "slot number {{{}}}", self.get_slot() as u8 + 1)   
//     }
// }

// unsafe impl Send for dyn GOcontrollModule {
    
// }