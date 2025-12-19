//! QEMU Exit Device Driver for Test Automation
//!
//! This module provides a mechanism to programmatically exit QEMU from within
//! the kernel. This is particularly useful for automated testing, where the
//! kernel needs to signal test completion and return a success or failure code
//! to the host system.
//!
//! The driver interfaces with QEMU's `isa-debug-exit` device through I/O port
//! 0xF4. This is a special QEMU-specific device that allows guest code to
//! terminate the emulator with a specific exit code.
//!
//! # QEMU Configuration
//! To use this driver, QEMU must be started with the isa-debug-exit device:
//! ```bash
//! qemu-system-i386 -device isa-debug-exit,iobase=0xf4,iosize=0x04 ...
//! ```
//!
//! # Exit Codes
//! The module defines two exit codes:
//! - `Success (0x10)`: Indicates successful test completion
//! - `Failed (0x11)`: Indicates test failure
//!
//! These values are written to port 0xF4 using the x86 `out` instruction,
//! which triggers QEMU to exit with the specified code.
//!
//! # Note
//! This functionality only works in QEMU and will have no effect (or cause
//! undefined behavior) on real hardware or other emulators.
//! NOT USED YET
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        // Utilisation directe de l'instruction outl (out long/dword)
        // Port 0xf4 est le port isa-debug-exit de QEMU
        core::arch::asm!(
            "out dx, eax",
            in("dx") 0xf4u16,
            in("eax") exit_code as u32,
            options(nomem, nostack)
        );
    }
}
