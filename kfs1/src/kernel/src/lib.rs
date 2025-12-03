#![no_std] // on enleve std
#![no_main] // ici on tej le main car on veux overwrite crt0 qui est la fonction appeler avant main
// et dans notre cas kernel_main

mod vga_buffer;

use core::panic::PanicInfo;

// VGA text mode constants
const VGA_MEMORY: usize = 0xB8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

// Entry point appelé depuis l'ASM
#[unsafe(no_mangle)] // ca cest pour empecher le compilateur de changer le nom de la fonction
pub extern "C" fn kernel_main(_magic: u32, _addr: u32) {
    let vga = VGA_MEMORY as *mut u16;
    let msg = b"Hello from Rust kernel! Created by kix!";
    
    unsafe {
        // Clear screen
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            *vga.add(i) = 0x0F00 | b' ' as u16;
        }
        
        // Print message
        for (i, &byte) in msg.iter().enumerate() {
            *vga.add(i) = 0x0F00 | byte as u16;
        }
        vga_buffer::print_something();
    }
    
    // Halt
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

// Panic handler requis en no_std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}
