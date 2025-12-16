#![no_std]
#![no_main]

mod panic;
#[cfg(kfs_test)]
pub mod tests;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn start(_magic: u32, _addr: u32) -> ! {
    #[cfg(kfs_test)]
    tests::run_tests();

    #[cfg(not(test))]
    {
        use core::fmt::Write;
        vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
        vga_buffer::WRITER
            .lock()
            .write_str(", Created by kix!")
            .unwrap();
        println!(" hello world depuis println! fait main ");
    }

    loop {}
}
