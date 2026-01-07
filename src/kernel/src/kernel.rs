#![no_std]
#![no_main]

pub mod device;
pub mod panic;
pub mod qemu;
#[cfg(kfs_test)]
pub mod tests;
pub mod vga_buffer;

// use device::keyboard::Keyboard;
use device::mouse::{MouseEvent, Mouse};

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
        let mut mouse = Mouse::default();
        match mouse.init() {
            Ok(()) => println!("Mouse initialized successfully!"),
            Err(e) => println!("Mouse init error: {}", e),
        }
        let mut keyboard = Keyboard::default();
        loop {
            if let Some(c) = keyboard.input() {
                print!("{}", c);
            }

            //bad in loop need GDT
            if let Some(event) = mouse.handle_interrupt() {
                match event {
                    MouseEvent::WheelUp => println!("Wheel UP!"),
                    MouseEvent::WheelDown => println!("Wheel DOWN!"),
                    MouseEvent::Move { delta_x, delta_y } => {
                        let (x, y) = mouse.position();
                        println!("Mouse moved: delta=({}, {}), pos=({}, {})", delta_x, delta_y, x, y);
                    },
                    MouseEvent::ButtonPressed(btn) => println!("Button pressed: {:?}", btn),
                    MouseEvent::ButtonReleased(btn) => println!("Button released: {:?}", btn),
                }
            }
        }
    }
}
