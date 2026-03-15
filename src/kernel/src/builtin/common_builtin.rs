use crate::device::keyboard::{inb, outb};
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
