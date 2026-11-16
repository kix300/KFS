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
    println!("clear: clear screen");
    println!("stack: print the kernel stack");
}

pub fn stack() {
    let esp: usize;

    unsafe {
        core::arch::asm!(
            "mov {}, esp",
            out(reg) esp,
            options(nomem, nostack, preserves_flags)
        );
    }

    let bottom = core::ptr::addr_of!(stack_bottom) as usize ;
    let top = core::ptr::addr_of!(stack_top) as usize ;

    if esp < bottom || esp >= top {
        println!("invalid kernel stack pointer: {:#010x}", esp);
        return;
    }

    let available = (top - esp) / core::mem::size_of::<u32>();
    let count = if available < 32 { available } else { 32 };

    println!("kernel stack: {:#010x}..{:#010x}", bottom, top);
    println!("ESP = {:#010x}", esp);
    for index in 0..count {
        let address = esp + index * core::mem::size_of::<u32>();
        let value = unsafe { core::ptr::read_volatile(address as *const u32) };
        println!("[{:#010x}] = {:#010x}", address, value);
    }
}

extern "C" {
    static stack_bottom: u8;
    static stack_top: u8;
}


pub fn miguel(){
    println!("Ca fait des rouuuuges cette aprem");
}

pub fn clear(){
    crate::vga_buffer::WRITER.lock().clear();
}
