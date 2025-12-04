#![no_std] // on enleve std
#![no_main] // ici on tej le main car on veux overwrite crt0 qui est la fonction appeler avant main
// et dans notre cas kernel_main

mod vga_buffer;
use core::panic::PanicInfo;


// Entry point appelé depuis l'ASM
#[unsafe(no_mangle)] // ca cest pour empecher le compilateur de changer le nom de la fonction
pub extern "C" fn start(_magic: u32, _addr: u32) {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    vga_buffer::WRITER.lock().write_str(", Created by kix!").unwrap();
    println!(" hello world depuis println! fait main ");
    panic!("Some panic message");
}

// Panic handler requis en no_std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {
    }
}
