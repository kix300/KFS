#![no_std]
#![no_main]

pub mod device;
pub mod panic;
pub mod qemu;
#[cfg(kfs_test)]
pub mod tests;
pub mod vga_buffer;

use device::keyboard::Keyboard;

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
        let mut keyboard = Keyboard::default();
        loop {
            let c = match keyboard.input() {
                Some(key) => key,
                None => continue,
            };

            print!("{}", c); //TODO : create a console
        }
    }
}
