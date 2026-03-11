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

use device::mouse::{Mouse, MouseEvent};

fn init(){
    x86::gdt::gdt_init();
    x86::idt::init_idt();
    unsafe {x86::idt::PICS.lock().initialize() };
    unsafe {core::arch::asm!("sti")};
}
// debug init
// fn init(){
//     x86::gdt::gdt_init();
//     x86::idt::init_idt();
//     unsafe {
//         x86::idt::PICS.lock().initialize();
//         // Lis et affiche les masques après initialize()
//         let masks = x86::idt::PICS.lock().read_masks();
//         println!("PIC masks: PIC1={:#010b} PIC2={:#010b}", masks[0], masks[1]);
//         // Force démasquage IRQ0 (timer) et IRQ1 (clavier) seulement
//         x86::idt::PICS.lock().write_masks(
//             0b11111100, // IRQ0 + IRQ1 activés
//             0b11111111, // tout masqué sur PIC2
//         );
//         core::arch::asm!("sti");
//     };
// }

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
        // let mut mouse = Mouse::default();
        // match mouse.init() {
        //     Ok(()) => println!("Mouse initialized successfully!"),
        //     Err(e) => println!("Mouse init error: {}", e),
        // }
       loop {
            unsafe { core::arch::asm!("hlt") };
        }
    }
    #[cfg(kfs_test)]
    tests::run_tests();
}
