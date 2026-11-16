//! This module exists only to import symbols from the kernel that has been
//! compiled as a library.

#![no_std]
#![no_main]
// Force presence of the test code for both `cargo test` and `cargo clippy --tests`
extern crate kernel;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)) }
    }
}
