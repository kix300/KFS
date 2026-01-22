//! Interrupt Descriptor Tables
//! This table will be use to interrupt cpu when needed
//! when a key is pressed the cpu should be interrupt do what the key do and then go back to what he was
//! IDT need GDT
//! IDT need to be init + have exception

pub const IDT_ENTRY_SIZE: usize = 256;

pub struct InterruptDescriptor {
    isr_low: u16,   // The lower 16 bits of the ISR's address
    kernel_cs: u16, // The GDT segment selector that the CPU will load into CS before calling the ISR
    reserved: u8,   // Set to zero
    attributes: u8, // Type and attributes; see the IDT page
    isr_high: u16,
}

impl InterruptDescriptor {}
