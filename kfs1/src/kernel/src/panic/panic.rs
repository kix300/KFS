//! //! Panic Handler for Kernel Error Management
//!
//! This module implements the panic handler required for no_std environments.
//! When the kernel encounters an unrecoverable error (like an assertion failure,
//! out-of-bounds access, or explicit panic), this handler is invoked to report
//! the error and halt execution safely.
//!
//! # Panic Behavior
//! The handler provides two different behaviors depending on the compilation mode:
//!
//! ## Normal Mode (not test)
//! - Displays the panic information to the VGA buffer
//! - Enters an infinite loop to halt the kernel safely
//! - Prevents the processor from executing undefined code
//!
//! ## Test Mode (cfg(test))
//! - Prints a standardized "[failed]" marker for test harness detection
//! - Displays detailed error information for debugging
//! - Enters an infinite loop (could be enhanced to exit QEMU for automation)
//!
//! # Implementation Details
//! The `PanicInfo` structure provided by Rust contains:
//! - The panic message (if provided)
//! - File name and line number where the panic occurred
//! - Optional payload data
//!
//! This information is formatted and displayed using the `println!` macro,
//! which writes to the VGA buffer, making panic messages visible on screen.

use crate::println;
use core::panic::PanicInfo;

// Panic handler pour le mode normal
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Panic handler pour les tests
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}", info);
    loop {}
}
