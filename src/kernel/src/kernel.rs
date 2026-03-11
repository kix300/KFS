#![no_std]
#![no_main]

pub mod device;
pub mod panic;
pub mod qemu;
#[cfg(kfs_test)]
pub mod tests;
pub mod vga_buffer;
pub mod x86;
pub mod pic8259;


fn init(){
    x86::gdt::gdt_init();
    x86::idt::init_idt();
    unsafe {x86::idt::PICS.lock().initialize() };
    unsafe {
        x86::idt::PICS.lock().write_masks(
            0b11111000,
            0b11101111,
        );
    }
    let _ = crate::device::mouse::MOUSE.lock().init();
    unsafe { core::arch::asm!("sti") };
}
#[no_mangle]
pub extern "C" fn start(_magic: u32, _addr: u32) -> ! {
    init();

    #[cfg(not(test))]
    {
        use core::fmt::Write;
        vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
        vga_buffer::WRITER
            .lock()
            .write_str(", Created by kix!")
            .unwrap();
        println!(" hello world depuis println! fait main ");
        loop {
            unsafe { core::arch::asm!("hlt") };
        }
    }
    #[cfg(kfs_test)]
    tests::run_tests();
}
