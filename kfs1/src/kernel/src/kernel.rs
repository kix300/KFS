#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod panic;
pub mod tests;

// Entry point appelé depuis l'ASM
// #[cfg(not(test))]
#[no_mangle]
pub extern "C" fn start(_magic: u32, _addr: u32) -> ! {
    
    #[cfg(test)]
    test_main();
    
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    vga_buffer::WRITER
    .lock()
    .write_str(", Created by kix!")
    .unwrap();
println!(" hello world depuis println! fait main ");

    loop {}
}

//Point d'entrée pour les tests
// #[cfg(test)]
// #[no_mangle]
// pub extern "C" fn start() -> ! {
//     test_main();
//     loop {}
// }
