#![no_std]
#![no_main]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
use core::panic::PanicInfo;

/// Test runner qui n'utilise pas le VGA buffer
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    // On ne peut pas utiliser println! car il accède au VGA buffer
    // Les tests vont simplement paniquer s'ils échouent
    for test in tests {
        test();
    }
    // Si on arrive ici, tous les tests ont réussi
    // On sort avec un code de succès (dans un vrai environnement de test)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[test_case]
fn test_addition() {
    assert_eq!(2 + 2, 4);
}

#[test_case]
fn test_vga_color_code() {
    use vga_buffer::{Color, ColorCode};
    let code = ColorCode::new(Color::White, Color::Black);
    // Test que la couleur est créée correctement
    assert_eq!(code.0, 0x0F); // White (15) sur Black (0) = 0x0F
}

// Entry point appelé depuis l'ASM
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn start(_magic: u32, _addr: u32) -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    vga_buffer::WRITER
        .lock()
        .write_str(", Created by kix!")
        .unwrap();
    println!(" hello world depuis println! fait main ");

    loop {}
}

// Point d'entrée pour les tests
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

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
    // On ne peut pas utiliser println! ici car VGA buffer n'est pas dispo
    // Le test va juste crasher et cargo test va le détecter
    loop {}
}
