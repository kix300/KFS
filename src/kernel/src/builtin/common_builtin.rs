use crate::device::keyboard::{inb, outb};
use crate::println;
pub fn reboot() -> ! {
    unsafe {
        core::arch::asm!("cli", options(nostack));
    }
    while { inb(0x64) } & 0x02 != 0 {}
        outb(0x64, 0xFE);

    loop {
        unsafe {
           core::arch::asm!("hlt", options(nostack));
        }
    }
}

pub fn help(){
    println!("Command :");
    println!("help : show all command");
    println!("reboot : reboot computer");
    println!("miguel: ???");
}


pub fn miguel(){
    println!("Ca fait des rouuuuges cette aprem");
}

pub fn clear(){
    crate::vga_buffer::WRITER.lock().clear();
}

