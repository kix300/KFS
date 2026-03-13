//! Interrupt Descriptor Tables
//! This table will be use to interrupt cpu when needed
//! when a key is pressed the cpu should be interrupt do what the key do and then go back to what he was
//! IDT need GDT
//! IDT need to be init + have exception

const IDT_ENTRY_SIZE: usize = 256;

static mut IDT: [InterruptDescriptor; IDT_ENTRY_SIZE] = [InterruptDescriptor::empty(); IDT_ENTRY_SIZE];

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct InterruptDescriptor {
    isr_low: u16, //0..15 first bits of the address of ISR
    selector: u16, // Selector in GDT
    zero: u8, //unused set to 0
    type_attributes: u8, // Gate type, dpl and p fileds
    isr_high: u16, // 15..31 last bits of the address of ISR
}

#[repr(C, packed)]
struct Idtr {
    limit: u16,
    base: u32,
}
#[no_mangle]
pub extern "C" fn exception_handler() -> ! {
    unsafe {
        core::arch::asm!("cli; hlt", options(nostack, noreturn));
    }
}

use crate::device::mouse::MouseEvent;
#[no_mangle]
pub extern "C" fn irq_handler(irq_num: u32) {
    match irq_num {
        0 => {},
        1 => {
            let scancode = crate::device::keyboard::inb(0x60);
            if scancode == 0x0E {
                crate::tty::tty::TTY.lock().remove_buffer();
            } else if let Some(c) = crate::device::keyboard::KEYBOARD.lock().process(scancode) {
                if c != '\0' {
                    crate::tty::tty::TTY.lock().add_buffer(c as u8);
                }
            }
        },
        12 => {
            let byte = crate::device::mouse::inb(0x60);
            if let Some(event) = crate::device::mouse::MOUSE.lock().process(byte) {
                match event {
                    MouseEvent::ButtonPressed(btn)  => println!("Mouse press: {:?}", btn),
                    MouseEvent::ButtonReleased(btn) => println!("Mouse release: {:?}", btn),
                    MouseEvent::Move { delta_x, delta_y } => println!("Move: {},{}", delta_x, delta_y),
                    MouseEvent::WheelUp   => crate::vga_buffer::WRITER.lock().scroll_up(),
                    MouseEvent::WheelDown   => crate::vga_buffer::WRITER.lock().scroll_down(),
                }
            }
        },
        _ => {
            println!("IRQ {}", irq_num);
        }
    }

    unsafe {
        crate::x86::idt::PICS
            .lock()
            .notify_end_of_interrupt((irq_num + 32) as u8);
    }
}



impl InterruptDescriptor {
    const fn empty() -> Self {
        InterruptDescriptor {
            isr_low:         0,
            selector:        0,
            zero:            0,
            type_attributes: 0,
            isr_high:        0,
        }
    }
    fn set(isr: u32, flags: u8) -> Self{
        InterruptDescriptor{
            isr_low:         (isr & 0xFFFF) as u16,
            selector:        0x08,
            zero:            0,
            type_attributes: flags,
            isr_high:        (isr >>16) as u16,
        }
    }
}
fn idt_set_descriptor(vector: u8, isr: u32, flags: u8) {
    unsafe {
        IDT[vector as usize] = InterruptDescriptor::set(isr, flags);
    }
}
fn idt_load() {
    let base = { core::ptr::addr_of!(IDT) as u32 };
    let idtr = Idtr {
        limit: (core::mem::size_of::<[InterruptDescriptor; IDT_ENTRY_SIZE]>() - 1) as u16,
        base,
    };
    unsafe {
        core::arch::asm!("lidt [{}]", in(reg) &idtr, options(nostack));
    }
}
extern "C" {
static isr_stub_table: [u32; 32];
static irq_stub_table: [u32; 16];
}

pub fn init_idt() {
    idt_set_descriptor(0, exception_handler as *const() as u32, 0x8E);
    unsafe {
        for i in 0..32u8 {
            idt_set_descriptor(
                i,
                isr_stub_table[i as usize],
                0x8E,
            );
        }
        for i in 0..16u8 {
            idt_set_descriptor(32+i, irq_stub_table[i as usize], 0x8E);
        }
    }
    idt_load();

    #[cfg(kfs_test)]
    unsafe {core::arch::asm!("int3")};
}

use crate::{pic8259::pic8259::ChainedPics, println};
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe{ ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)});

